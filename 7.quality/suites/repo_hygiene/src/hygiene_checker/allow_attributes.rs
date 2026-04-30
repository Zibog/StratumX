use super::HygieneChecker;
use crate::models::{HygieneRule, HygieneViolation};
use std::fs;

impl HygieneChecker {
    /// Check for #[allow(...)] attributes outside waiver registry.
    pub fn check_allow_attributes(&self) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        if let Ok(entries) = self.walk_production_code() {
            for entry in entries {
                if entry.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&entry) {
                    let relative_path = self.relative_path(&entry);
                    for (line_num, line) in content.lines().enumerate() {
                        let trimmed = line.trim();
                        if !trimmed.starts_with("#[allow(") {
                            continue;
                        }

                        let rule = HygieneRule::AllowAttribute {
                            attr: trimmed.to_string(),
                        };
                        if self.waiver_registry.is_waived(&rule, relative_path) {
                            continue;
                        }

                        violations.push(HygieneViolation {
                            path: relative_path.to_path_buf(),
                            rule,
                            line_number: Some(line_num + 1),
                            context: trimmed.to_string(),
                            waived: false,
                            remediation: "Remove #[allow(...)] attribute and fix the underlying issue, or add a waiver entry with justification if the attribute is legitimately needed.".to_string(),
                        });
                    }
                }
            }
        }

        violations
    }
}
