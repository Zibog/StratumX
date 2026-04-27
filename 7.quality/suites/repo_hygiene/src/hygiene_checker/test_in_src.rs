use super::HygieneChecker;
use crate::models::{HygieneRule, HygieneViolation};
use std::fs;
use stratumx_repo_hygiene_support::{find_inline_test_debt_in_file, InlineTestDebtKind};

impl HygieneChecker {
    /// Check for test files in production src/ directories.
    pub fn check_test_files_in_src(&self) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        if let Ok(entries) = self.walk_production_code() {
            for entry in entries {
                if !entry.extension().is_some_and(|ext| ext == "rs") {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&entry) {
                    let relative_path = self.relative_path(&entry);
                    let rule = HygieneRule::TestFileInSrc;
                    if self.waiver_registry.is_waived(&rule, relative_path) {
                        continue;
                    }

                    for debt in find_inline_test_debt_in_file(&entry, &content) {
                        let remediation = match debt.kind {
                            InlineTestDebtKind::MisplacedTestFile => {
                                "Move this test file to 7.quality/suites/ or a crate-level tests/ directory."
                            }
                            _ => {
                                "Move heavy, property, smoke, or integration-style tests out of src/ into 7.quality/suites/."
                            }
                        };
                        violations.push(HygieneViolation {
                            path: relative_path.to_path_buf(),
                            rule: rule.clone(),
                            line_number: Some(debt.line),
                            context: debt.snippet,
                            waived: false,
                            remediation: remediation.to_string(),
                        });
                    }
                }
            }
        }

        violations
    }
}
