// Quality Restructurer Module
//
// This module provides functionality for organizing migrated tests into appropriate
// suite categories under 7.quality/suites/. It creates the suite structure, categorizes
// tests by type, and preserves existing suite organization.

use crate::models::{TestFileCandidate, TestType};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Organizes migrated tests into appropriate suite categories
pub struct QualityRestructurer {
    repo_root: PathBuf,
}

impl QualityRestructurer {
    /// Create a new QualityRestructurer with the given repository root
    pub fn new(repo_root: PathBuf) -> Self {
        Self { repo_root }
    }

    /// Get the quality suites root directory
    fn suites_root(&self) -> PathBuf {
        self.repo_root.join("7.quality/suites")
    }

    /// Check if a test file is already in the quality suites directory
    pub fn is_in_existing_suite(&self, path: &Path) -> bool {
        path.starts_with(self.suites_root())
    }

    /// Create the complete suite structure with all subdirectories
    pub fn create_suite_structure(&self) -> Result<(), std::io::Error> {
        let suites_root = self.suites_root();

        // Define the suite categories and their subsystems
        let suite_categories = vec![
            (
                "editor_canon_matrix",
                vec!["command_spine", "editor_shell", "state_containers"],
            ),
            ("editor_app_matrix", vec!["desktop_app"]),
            ("tooling_canon_matrix", vec!["tool_session"]),
        ];

        // Create each suite category and its subsystem subdirectories
        for (suite_name, subsystems) in suite_categories {
            let suite_path = suites_root.join(suite_name);

            for subsystem in subsystems {
                let subsystem_path = suite_path.join(subsystem);

                // Create subdirectories for each test type
                for test_type in &["unit", "integration", "property"] {
                    let test_type_path = subsystem_path.join(test_type);
                    fs::create_dir_all(&test_type_path)?;
                }
            }
        }

        Ok(())
    }

    /// Determine the target suite path for a test file based on its type and source location
    pub fn categorize_test(&self, candidate: &TestFileCandidate) -> Result<PathBuf, String> {
        // Skip tests already in quality suites
        if self.is_in_existing_suite(&candidate.path) {
            return Err(format!(
                "Test already in quality suite: {}",
                candidate.path.display()
            ));
        }

        // Determine suite category and subsystem from source path
        let (suite_category, subsystem) = self.determine_suite_mapping(&candidate.path)?;

        // Determine test type subdirectory
        let test_type_dir = match candidate.test_type {
            TestType::Unit => "unit",
            TestType::Integration => "integration",
            TestType::Property => "property",
            TestType::Invariant => "property", // Invariants go with property tests
            TestType::Smoke => "integration",  // Smoke tests go with integration tests
        };

        // Construct the target path
        let target_path = self
            .suites_root()
            .join(suite_category)
            .join(subsystem)
            .join(test_type_dir)
            .join(candidate.path.file_name().unwrap());

        Ok(target_path)
    }

    /// Map source path to suite category and subsystem
    fn determine_suite_mapping(&self, source_path: &Path) -> Result<(&str, &str), String> {
        let path_str = source_path.to_string_lossy();

        // Map based on source directory
        if path_str.contains("5.editor/l7.0-editor-command-spine") {
            Ok(("editor_canon_matrix", "command_spine"))
        } else if path_str.contains("5.editor/l8.0-editor-shell") {
            Ok(("editor_canon_matrix", "editor_shell"))
        } else if path_str.contains("6.apps/editor/stratumx_editor_app") {
            Ok(("editor_app_matrix", "desktop_app"))
        } else if path_str.contains("4.tooling/l6.0-tool-session") {
            Ok(("tooling_canon_matrix", "tool_session"))
        } else if path_str.contains("5.editor/editor-state-containers") {
            Ok(("editor_canon_matrix", "state_containers"))
        } else {
            Err(format!(
                "Unable to determine suite mapping for path: {}",
                source_path.display()
            ))
        }
    }

    /// Get all test files currently in quality suites
    pub fn get_existing_suite_tests(&self) -> Result<HashSet<PathBuf>, std::io::Error> {
        let mut existing_tests = HashSet::new();
        let suites_root = self.suites_root();

        if !suites_root.exists() {
            return Ok(existing_tests);
        }

        // Walk through all suite directories
        for entry in walkdir::WalkDir::new(&suites_root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        existing_tests.insert(path.to_path_buf());
                    }
                }
            }
        }

        Ok(existing_tests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_restructurer() -> (QualityRestructurer, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());
        (restructurer, temp_dir)
    }

    #[test]
    fn test_is_in_existing_suite() {
        let (restructurer, temp_dir) = create_test_restructurer();
        let suite_path = temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/test.rs");
        let non_suite_path = temp_dir.path().join("5.editor/src/test.rs");

        assert!(restructurer.is_in_existing_suite(&suite_path));
        assert!(!restructurer.is_in_existing_suite(&non_suite_path));
    }

    #[test]
    fn test_determine_suite_mapping() {
        let (restructurer, temp_dir) = create_test_restructurer();

        let test_cases = vec![
            (
                temp_dir
                    .path()
                    .join("5.editor/l7.0-editor-command-spine/src/test.rs"),
                ("editor_canon_matrix", "command_spine"),
            ),
            (
                temp_dir
                    .path()
                    .join("5.editor/l8.0-editor-shell/src/test.rs"),
                ("editor_canon_matrix", "editor_shell"),
            ),
            (
                temp_dir
                    .path()
                    .join("6.apps/editor/stratumx_editor_app/src/test.rs"),
                ("editor_app_matrix", "desktop_app"),
            ),
            (
                temp_dir
                    .path()
                    .join("4.tooling/l6.0-tool-session/src/test.rs"),
                ("tooling_canon_matrix", "tool_session"),
            ),
            (
                temp_dir
                    .path()
                    .join("5.editor/editor-state-containers/src/test.rs"),
                ("editor_canon_matrix", "state_containers"),
            ),
        ];

        for (path, expected) in test_cases {
            let result = restructurer.determine_suite_mapping(&path);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}
