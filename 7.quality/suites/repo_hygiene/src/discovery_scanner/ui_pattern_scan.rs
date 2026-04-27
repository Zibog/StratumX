use super::DiscoveryScanner;
use crate::models::{DomainLogicType, DomainLogicViolation, HostBypassPattern};
use std::fs;
use std::path::Path;

impl DiscoveryScanner {
    /// Scan for host bypass patterns in UI code.
    pub fn scan_host_bypasses(&self) -> Vec<HostBypassPattern> {
        let mut bypasses = Vec::new();
        let desktop_app_path = self.desktop_app_path();

        if !desktop_app_path.exists() {
            return bypasses;
        }

        for entry in walkdir::WalkDir::new(&desktop_app_path)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !Self::is_rust_file(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                bypasses.extend(self.find_host_bypasses(path, &content));
            }
        }

        bypasses
    }

    /// Scan for domain logic in UI code.
    pub fn scan_domain_logic_in_ui(&self) -> Vec<DomainLogicViolation> {
        let mut violations = Vec::new();
        let desktop_app_path = self.desktop_app_path();

        if !desktop_app_path.exists() {
            return violations;
        }

        for entry in walkdir::WalkDir::new(&desktop_app_path)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !Self::is_rust_file(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                violations.extend(self.find_domain_logic(path, &content));
            }
        }

        violations
    }

    fn find_host_bypasses(&self, path: &Path, content: &str) -> Vec<HostBypassPattern> {
        content
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains("self.host."))
            .map(|(line_num, line)| HostBypassPattern {
                file: path.to_path_buf(),
                line: line_num + 1,
                pattern: line.trim().to_string(),
                suggested_action:
                    "Route this operation through Command_Spine instead of direct host access"
                        .to_string(),
            })
            .collect()
    }

    fn find_domain_logic(&self, path: &Path, content: &str) -> Vec<DomainLogicViolation> {
        let mut violations = Vec::new();
        let file_line_range = (1, content.lines().count());

        if content.contains("fn parse") || content.contains("impl Parser") {
            violations.push(DomainLogicViolation {
                file: path.to_path_buf(),
                violation_type: DomainLogicType::Parser,
                line_range: file_line_range,
                suggested_target_layer: "Extract parser to appropriate service layer".to_string(),
            });
        }

        if content.contains("fn validate") || content.contains("impl Validator") {
            violations.push(DomainLogicViolation {
                file: path.to_path_buf(),
                violation_type: DomainLogicType::Validator,
                line_range: file_line_range,
                suggested_target_layer: "Extract validator to appropriate service layer"
                    .to_string(),
            });
        }

        if content.contains("business_rule") || content.contains("domain_logic") {
            violations.push(DomainLogicViolation {
                file: path.to_path_buf(),
                violation_type: DomainLogicType::BusinessRule,
                line_range: file_line_range,
                suggested_target_layer: "Extract business rule to appropriate domain layer"
                    .to_string(),
            });
        }

        violations
    }
}
