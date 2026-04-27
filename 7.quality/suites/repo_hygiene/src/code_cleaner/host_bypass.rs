use super::CodeCleaner;
use crate::models::HostBypassPattern;
use std::fs;
use std::path::Path;

impl CodeCleaner {
    /// Identify all host bypass patterns in Desktop_App panels.
    pub fn identify_host_bypasses(&self) -> Vec<HostBypassPattern> {
        let mut bypasses = Vec::new();
        let desktop_app_path = self.desktop_app_path();
        if !desktop_app_path.exists() {
            return bypasses;
        }

        if let Ok(entries) = self.scan_rust_files(&desktop_app_path) {
            for file_path in entries {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    bypasses.extend(self.find_host_bypasses_in_file(&file_path, &content));
                }
            }
        }

        bypasses
    }

    fn find_host_bypasses_in_file(
        &self,
        file_path: &Path,
        content: &str,
    ) -> Vec<HostBypassPattern> {
        let mut bypasses = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            if line.contains("self.host.") {
                if let Some(pattern) = self.extract_host_pattern(line) {
                    bypasses.push(HostBypassPattern {
                        file: file_path.to_path_buf(),
                        line: line_num + 1,
                        pattern: pattern.clone(),
                        suggested_action: self.suggest_host_bypass_remediation(&pattern),
                    });
                }
            }
        }
        bypasses
    }

    pub(crate) fn extract_host_pattern(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find("self.host.") {
            let rest = &line[start..];
            if let Some(paren_start) = rest.find('(') {
                let mut depth = 0;
                let mut end_pos = paren_start;
                for (i, ch) in rest[paren_start..].chars().enumerate() {
                    if ch == '(' {
                        depth += 1;
                    } else if ch == ')' {
                        depth -= 1;
                        if depth == 0 {
                            end_pos = paren_start + i + 1;
                            break;
                        }
                    }
                }
                Some(rest[..end_pos].trim().to_string())
            } else {
                let end = rest.find(';').unwrap_or(rest.len());
                Some(rest[..end].trim().to_string())
            }
        } else {
            None
        }
    }

    pub(crate) fn suggest_host_bypass_remediation(&self, pattern: &str) -> String {
        if pattern.contains("dialog") {
            "Route through Command_Spine dialog action".to_string()
        } else if pattern.contains("save_file") || pattern.contains("write_file") {
            "Route through Command_Spine SaveFile action".to_string()
        } else if pattern.contains("open_file") || pattern.contains("read_file") {
            "Route through Command_Spine OpenFile action".to_string()
        } else {
            format!(
                "Route through Command_Spine instead of direct host access: {}",
                pattern
            )
        }
    }
}
