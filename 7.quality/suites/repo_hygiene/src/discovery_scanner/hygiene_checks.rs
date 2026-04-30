use super::DiscoveryScanner;
use crate::models::{HygieneRule, HygieneViolation};
use std::fs;
use std::path::Path;
use stratumx_repo_hygiene_support::find_inline_test_debt_in_file;

impl DiscoveryScanner {
    /// Scan the repository for hygiene violations.
    pub fn scan_hygiene_violations(&self) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        for entry in self.walk_repo().flatten() {
            let path = entry.path();
            if !path.is_file() || !Self::is_rust_file(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                violations.extend(self.check_line_count(path, &content));
                violations.extend(self.check_test_file_in_src(path));
                violations.extend(self.check_todo_comments(path, &content));
                violations.extend(self.check_allow_attributes(path, &content));
            }
        }

        violations
    }

    fn check_line_count(&self, path: &Path, content: &str) -> Vec<HygieneViolation> {
        const LINE_LIMIT: usize = 200;
        let line_count = content.lines().count();
        if line_count <= LINE_LIMIT {
            return Vec::new();
        }

        let rule = HygieneRule::LineLimit {
            limit: LINE_LIMIT,
            actual: line_count,
        };
        let relative_path = path.strip_prefix(&self.repo_root).unwrap_or(path);

        if self.waiver_registry.is_waived(&rule, relative_path) {
            return Vec::new();
        }

        vec![HygieneViolation {
            path: path.to_path_buf(),
            rule,
            line_number: None,
            context: format!("File has {} lines (limit: {})", line_count, LINE_LIMIT),
            waived: false,
            remediation: "Split this file into smaller, focused modules".to_string(),
        }]
    }

    fn check_test_file_in_src(&self, path: &Path) -> Vec<HygieneViolation> {
        let content = fs::read_to_string(path).unwrap_or_default();
        find_inline_test_debt_in_file(path, &content)
            .into_iter()
            .map(|debt| HygieneViolation {
                path: path.to_path_buf(),
                rule: HygieneRule::TestFileInSrc,
                line_number: Some(debt.line),
                context: debt.snippet,
                waived: false,
                remediation:
                    "Move heavy, property, smoke, or integration-style tests out of src/ and into 7.quality/suites/."
                        .to_string(),
            })
            .collect()
    }

    fn check_todo_comments(&self, path: &Path, content: &str) -> Vec<HygieneViolation> {
        content
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains("TODO"))
            .map(|(line_num, line)| HygieneViolation {
                path: path.to_path_buf(),
                rule: HygieneRule::TodoComment,
                line_number: Some(line_num + 1),
                context: line.trim().to_string(),
                waived: false,
                remediation: "Remove TODO comment or create a tracked issue".to_string(),
            })
            .collect()
    }

    fn check_allow_attributes(&self, path: &Path, content: &str) -> Vec<HygieneViolation> {
        let relative_path = path.strip_prefix(&self.repo_root).unwrap_or(path);
        let mut violations = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.starts_with("#[allow(") {
                continue;
            }

            let attr = trimmed
                .trim_start_matches("#[allow(")
                .trim_end_matches(")]")
                .to_string();
            let rule = HygieneRule::AllowAttribute { attr };

            if !self.waiver_registry.is_waived(&rule, relative_path) {
                violations.push(HygieneViolation {
                    path: path.to_path_buf(),
                    rule,
                    line_number: Some(line_num + 1),
                    context: trimmed.to_string(),
                    waived: false,
                    remediation:
                        "Remove #[allow(...)] attribute or add to waiver registry with justification"
                            .to_string(),
                });
            }
        }

        violations
    }
}
