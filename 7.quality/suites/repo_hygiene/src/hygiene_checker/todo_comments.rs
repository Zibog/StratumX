use super::HygieneChecker;
use crate::models::{HygieneRule, HygieneViolation};
use std::fs;

impl HygieneChecker {
    /// Check for TODO comments in production code.
    pub fn check_todo_comments(&self) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        if let Ok(entries) = self.walk_production_code() {
            for entry in entries {
                if entry.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&entry) {
                    for (line_num, line) in content.lines().enumerate() {
                        if line.to_uppercase().contains("TODO") {
                            violations.push(HygieneViolation {
                                path: self.relative_path(&entry).to_path_buf(),
                                rule: HygieneRule::TodoComment,
                                line_number: Some(line_num + 1),
                                context: line.trim().to_string(),
                                waived: false,
                                remediation: "Remove TODO comment or create a tracked issue and reference it instead.".to_string(),
                            });
                        }
                    }
                }
            }
        }

        violations
    }
}
