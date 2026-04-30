use super::DiscoveryScanner;
use crate::models::{TestFileCandidate, TestType};
use std::fs;
use std::path::Path;

impl DiscoveryScanner {
    /// Scan the repository for test files that should be migrated.
    pub fn scan_test_files(&self) -> Vec<TestFileCandidate> {
        let mut candidates = Vec::new();

        for entry in self.walk_repo().flatten() {
            let path = entry.path();
            if !path.is_file() || !Self::is_rust_file(path) {
                continue;
            }

            if self.is_test_file(path) {
                if let Ok(content) = fs::read_to_string(path) {
                    candidates.push(TestFileCandidate {
                        path: path.to_path_buf(),
                        test_type: TestType::detect_from_file(path, &content),
                        target_suite: self.determine_target_suite(path),
                        dependencies: self.extract_dependencies(&content),
                    });
                }
            }
        }

        candidates
    }

    fn is_test_file(&self, path: &Path) -> bool {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        if file_name.ends_with(".test.rs") || file_name.ends_with("_tests.rs") {
            return self.is_in_src_tree(path);
        }

        if !self.is_in_src_tree(path) {
            return false;
        }

        if let Ok(content) = fs::read_to_string(path) {
            if content.contains("proptest!")
                || content.contains("prop_compose!")
                || content.contains("invariant!")
                || file_name.contains("smoke")
            {
                return true;
            }
        }

        false
    }

    fn determine_target_suite(&self, path: &Path) -> String {
        let path_str = path.to_string_lossy();

        if path_str.contains("l7.0-editor-command-spine") {
            "editor_canon_matrix/command_spine".to_string()
        } else if path_str.contains("l8.0-editor-shell") {
            "editor_canon_matrix/editor_shell".to_string()
        } else if path_str.contains("stratumx_editor_app") || path_str.contains("desktop_app") {
            "editor_app_matrix/desktop_app".to_string()
        } else if path_str.contains("l6.0-tool-session") {
            "tooling_canon_matrix/tool_session".to_string()
        } else if path_str.contains("editor-state-containers") {
            "editor_canon_matrix/state_containers".to_string()
        } else {
            "general".to_string()
        }
    }

    fn extract_dependencies(&self, content: &str) -> Vec<String> {
        content
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("use ") && !line.contains("crate::"))
            .map(ToString::to_string)
            .collect()
    }

    fn is_in_src_tree(&self, path: &Path) -> bool {
        path.ancestors().any(|ancestor| {
            ancestor
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "src")
        })
    }
}
