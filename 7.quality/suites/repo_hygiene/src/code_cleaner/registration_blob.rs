use super::CodeCleaner;
use crate::models::RegistrationBlob;
use std::fs;
use std::path::Path;

impl CodeCleaner {
    /// Identify all registration blobs in Command_Spine.
    pub fn identify_registration_blobs(&self) -> Vec<RegistrationBlob> {
        let mut blobs = Vec::new();
        let command_spine_path = self.command_spine_path();
        if !command_spine_path.exists() {
            return blobs;
        }

        if let Ok(entries) = self.scan_rust_files(&command_spine_path) {
            for file_path in entries {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Some(blob) = self.check_registration_blob(&file_path, &content) {
                        blobs.push(blob);
                    }
                }
            }
        }

        blobs
    }

    fn check_registration_blob(&self, file_path: &Path, content: &str) -> Option<RegistrationBlob> {
        let filename = file_path.file_name()?.to_str()?;
        if !filename.contains("register") && !filename.contains("registration") {
            return None;
        }

        let line_count = content.lines().count();
        if line_count <= 200 {
            return None;
        }

        let mixed_concerns = self.detect_mixed_concerns(content);
        let suggested_decomposition = self.suggest_decomposition(&mixed_concerns);

        Some(RegistrationBlob {
            file: file_path.to_path_buf(),
            line_count,
            mixed_concerns,
            suggested_decomposition,
        })
    }

    pub(crate) fn detect_mixed_concerns(&self, content: &str) -> Vec<String> {
        let mut concerns = Vec::new();
        if content.contains("register_command") || content.contains("Command::") {
            concerns.push("Command registration".to_string());
        }
        if content.contains("register_panel") || content.contains("Panel::") {
            concerns.push("Panel registration".to_string());
        }
        if content.contains("register_tool") || content.contains("Tool::") {
            concerns.push("Tool registration".to_string());
        }
        if content.contains("register_action") || content.contains("Action::") {
            concerns.push("Action registration".to_string());
        }
        if content.contains("register_service") || content.contains("Service::") {
            concerns.push("Service registration".to_string());
        }
        concerns
    }

    fn suggest_decomposition(&self, mixed_concerns: &[String]) -> Vec<String> {
        if mixed_concerns.len() > 1 {
            let mut suggestions =
                vec!["Split into separate registration modules by concern".to_string()];
            suggestions.extend(
                mixed_concerns
                    .iter()
                    .map(|concern| format!("Create dedicated module for {}", concern)),
            );
            suggestions
        } else {
            vec!["Break down into smaller, focused registration functions".to_string()]
        }
    }
}
