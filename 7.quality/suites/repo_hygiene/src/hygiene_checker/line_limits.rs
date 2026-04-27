use super::HygieneChecker;
use crate::models::{HygieneRule, HygieneViolation};
use std::fs;

impl HygieneChecker {
    /// Check all production code files for line limit violations.
    pub fn check_line_limits(&self) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        if let Ok(entries) = self.walk_production_code() {
            for entry in entries {
                if !entry.extension().is_some_and(|ext| ext == "rs") {
                    continue;
                }
                if let Ok(metadata) = fs::metadata(&entry) {
                    if !metadata.is_file() {
                        continue;
                    }
                }
                if let Ok(content) = fs::read_to_string(&entry) {
                    let line_count = content.lines().count();
                    if line_count <= 200 {
                        continue;
                    }

                    let relative_path = self.relative_path(&entry);
                    let rule = HygieneRule::LineLimit {
                        limit: 200,
                        actual: line_count,
                    };

                    if !self.waiver_registry.is_waived(&rule, relative_path) {
                        violations.push(HygieneViolation {
                            path: relative_path.to_path_buf(),
                            rule,
                            line_number: None,
                            context: format!("File has {} lines (limit: 200)", line_count),
                            waived: false,
                            remediation: "Refactor this file to be under 200 lines, or add a waiver entry with justification if the file legitimately needs to be longer.".to_string(),
                        });
                    }
                }
            }
        }

        violations
    }
}
