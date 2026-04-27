use super::DiscoveryScanner;
use crate::models::RegistrationBlob;
use std::fs;
use std::path::Path;

impl DiscoveryScanner {
    /// Scan for large registration modules that should be decomposed.
    pub fn scan_registration_blobs(&self) -> Vec<RegistrationBlob> {
        let mut blobs = Vec::new();
        let command_spine_path = self.command_spine_path();

        if !command_spine_path.exists() {
            return blobs;
        }

        for entry in walkdir::WalkDir::new(&command_spine_path)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !Self::is_rust_file(path) || !self.is_registration_module(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                if let Some(blob) = self.check_registration_blob(path, &content) {
                    blobs.push(blob);
                }
            }
        }

        blobs
    }

    fn is_registration_module(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.contains("registration") || name.contains("register"))
    }

    fn check_registration_blob(&self, path: &Path, content: &str) -> Option<RegistrationBlob> {
        const BLOB_THRESHOLD: usize = 200;
        let line_count = content.lines().count();
        if line_count <= BLOB_THRESHOLD {
            return None;
        }

        let mixed_concerns = self.identify_mixed_concerns(content);
        let suggested_decomposition = if mixed_concerns.is_empty() {
            vec!["Consider splitting into smaller registration modules".to_string()]
        } else {
            vec![
                "Split into separate modules by concern".to_string(),
                format!("Identified concerns: {}", mixed_concerns.join(", ")),
            ]
        };

        Some(RegistrationBlob {
            file: path.to_path_buf(),
            line_count,
            mixed_concerns,
            suggested_decomposition,
        })
    }

    fn identify_mixed_concerns(&self, content: &str) -> Vec<String> {
        let mut concerns = Vec::new();
        if content.contains("register_command") {
            concerns.push("command_registration".to_string());
        }
        if content.contains("register_panel") {
            concerns.push("panel_registration".to_string());
        }
        if content.contains("register_action") {
            concerns.push("action_registration".to_string());
        }
        if content.contains("register_service") {
            concerns.push("service_registration".to_string());
        }
        concerns
    }
}
