use super::HygieneChecker;
use crate::models::{HygieneRule, HygieneViolation};
use std::fs;

impl HygieneChecker {
    /// Check for prohibited patterns in production code.
    pub fn check_prohibitions(&self) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        if let Ok(entries) = self.walk_production_code() {
            for entry in entries {
                if entry.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&entry) {
                    let relative_path = self.relative_path(&entry);
                    for (line_num, line) in content.lines().enumerate() {
                        violations.extend(self.prohibitions_for_line(
                            relative_path,
                            line_num + 1,
                            line,
                        ));
                    }
                }
            }
        }

        violations
    }

    fn prohibitions_for_line(
        &self,
        relative_path: &std::path::Path,
        line_number: usize,
        line: &str,
    ) -> Vec<HygieneViolation> {
        let mut violations = Vec::new();

        if line.contains("static mut") {
            violations.push(HygieneViolation {
                path: relative_path.to_path_buf(),
                rule: HygieneRule::StaticMut,
                line_number: Some(line_number),
                context: line.trim().to_string(),
                waived: false,
                remediation: "Replace static mut with thread-safe alternatives like Mutex, RwLock, or atomic types.".to_string(),
            });
        }

        if line.contains("fn execute_") || line.contains("pub fn execute_") {
            let rule = HygieneRule::ExecuteBridge;
            if !self.waiver_registry.is_waived(&rule, relative_path) {
                violations.push(HygieneViolation {
                    path: relative_path.to_path_buf(),
                    rule,
                    line_number: Some(line_number),
                    context: line.trim().to_string(),
                    waived: false,
                    remediation: "Remove temporary execute_* bridge function and use proper canonical action paths.".to_string(),
                });
            }
        }

        if line.contains("self.host.") {
            violations.push(HygieneViolation {
                path: relative_path.to_path_buf(),
                rule: HygieneRule::HostBypass,
                line_number: Some(line_number),
                context: line.trim().to_string(),
                waived: false,
                remediation: "Route operation through Command_Spine instead of direct host access."
                    .to_string(),
            });
        }

        if line.contains("\"/home/") || line.contains("\"C:\\") || line.contains("\"/Users/") {
            violations.push(HygieneViolation {
                path: relative_path.to_path_buf(),
                rule: HygieneRule::HardcodedPath,
                line_number: Some(line_number),
                context: line.trim().to_string(),
                waived: false,
                remediation: "Use environment variables, configuration files, or user-provided paths instead of hardcoded paths.".to_string(),
            });
        }

        let trimmed = line.trim_start();
        if !trimmed.starts_with("//")
            && !trimmed.starts_with("/*")
            && line.to_lowercase().contains("fake")
            && line.to_lowercase().contains("default")
        {
            violations.push(HygieneViolation {
                path: relative_path.to_path_buf(),
                rule: HygieneRule::FakeTruth,
                line_number: Some(line_number),
                context: line.trim().to_string(),
                waived: false,
                remediation: "Remove default fake truth pattern and use proper persistence or explicit initialization.".to_string(),
            });
        }

        violations
    }
}
