use super::CodeCleaner;
use crate::models::{DomainLogicType, DomainLogicViolation};
use std::fs;
use std::path::Path;

impl CodeCleaner {
    /// Identify domain logic in Desktop_App UI code.
    pub fn identify_domain_logic_in_ui(&self) -> Vec<DomainLogicViolation> {
        let mut violations = Vec::new();
        let desktop_app_path = self.desktop_app_path();
        if !desktop_app_path.exists() {
            return violations;
        }

        if let Ok(entries) = self.scan_rust_files(&desktop_app_path) {
            for file_path in entries {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    violations.extend(self.find_domain_logic_in_file(&file_path, &content));
                }
            }
        }

        violations
    }

    fn find_domain_logic_in_file(
        &self,
        file_path: &Path,
        content: &str,
    ) -> Vec<DomainLogicViolation> {
        let mut violations = Vec::new();
        violations.extend(self.detect_parsers(file_path, content));
        violations.extend(self.detect_validators(file_path, content));
        violations.extend(self.detect_business_rules(file_path, content));
        violations
    }

    fn detect_parsers(&self, file_path: &Path, content: &str) -> Vec<DomainLogicViolation> {
        self.find_first_violation(
            file_path,
            content,
            |line| {
                line.contains("fn parse_")
                    || line.contains("::parse(")
                    || line.contains(".parse::<")
            },
            DomainLogicType::Parser,
            "Extract to domain service layer",
        )
    }

    fn detect_validators(&self, file_path: &Path, content: &str) -> Vec<DomainLogicViolation> {
        self.find_first_violation(
            file_path,
            content,
            |line| {
                (line.contains("fn validate_") || line.contains("is_valid_"))
                    && !line.contains("// UI validation")
            },
            DomainLogicType::Validator,
            "Extract to validation service layer",
        )
    }

    fn detect_business_rules(&self, file_path: &Path, content: &str) -> Vec<DomainLogicViolation> {
        self.find_first_violation(
            file_path,
            content,
            |line| {
                line.contains("fn calculate_")
                    || line.contains("fn compute_")
                    || line.contains("fn apply_rule")
            },
            DomainLogicType::BusinessRule,
            "Extract to business logic layer",
        )
    }

    fn find_first_violation<F>(
        &self,
        file_path: &Path,
        content: &str,
        predicate: F,
        violation_type: DomainLogicType,
        target_layer: &str,
    ) -> Vec<DomainLogicViolation>
    where
        F: Fn(&str) -> bool,
    {
        for (line_num, line) in content.lines().enumerate() {
            if predicate(line) {
                let start_line = line_num + 1;
                let end_line = self.find_block_end(content, line_num).unwrap_or(start_line);
                return vec![DomainLogicViolation {
                    file: file_path.to_path_buf(),
                    violation_type,
                    line_range: (start_line, end_line),
                    suggested_target_layer: target_layer.to_string(),
                }];
            }
        }
        Vec::new()
    }

    fn find_block_end(&self, content: &str, start_line: usize) -> Option<usize> {
        let lines: Vec<&str> = content.lines().collect();
        let mut brace_count = 0;
        let mut found_opening = false;

        for (i, line) in lines.iter().enumerate().skip(start_line) {
            for ch in line.chars() {
                if ch == '{' {
                    brace_count += 1;
                    found_opening = true;
                } else if ch == '}' {
                    brace_count -= 1;
                    if found_opening && brace_count == 0 {
                        return Some(i + 1);
                    }
                }
            }
        }

        None
    }
}
