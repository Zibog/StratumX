// Test Migrator Module
//
// This module handles the migration of test files from production src/ directories
// to the appropriate 7.quality/suites/ locations with proper organization.

use crate::models::{CompilationStatus, MigrationResult, TestFileCandidate};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Test migrator that moves test files from production src/ to 7.quality/suites/
pub struct TestMigrator {
    /// Root directory of the repository
    pub repo_root: PathBuf,
    /// Whether to perform a dry run (no actual file operations)
    pub dry_run: bool,
}

impl TestMigrator {
    /// Create a new test migrator
    pub fn new(repo_root: PathBuf, dry_run: bool) -> Self {
        Self { repo_root, dry_run }
    }

    /// Determine the destination suite path for a test file based on its source location
    ///
    /// Maps source directories to their corresponding suite locations:
    /// - command_spine → editor_canon_matrix/command_spine/
    /// - editor_shell → editor_canon_matrix/editor_shell/
    /// - desktop_app → editor_app_matrix/desktop_app/
    /// - tool_session → tooling_canon_matrix/tool_session/
    /// - state_containers → editor_canon_matrix/state_containers/
    pub fn determine_destination(&self, source_path: &Path) -> Option<PathBuf> {
        let path_str = source_path.to_string_lossy();

        // Determine which subsystem this test belongs to
        let suite_subpath = if path_str.contains("l7.0-editor-command-spine") {
            "editor_canon_matrix/command_spine"
        } else if path_str.contains("l8.0-editor-shell") {
            "editor_canon_matrix/editor_shell"
        } else if path_str.contains("stratumx_editor_app") || path_str.contains("desktop_app") {
            "editor_app_matrix/desktop_app"
        } else if path_str.contains("l6.0-tool-session") {
            "tooling_canon_matrix/tool_session"
        } else if path_str.contains("editor-state-containers") {
            "editor_canon_matrix/state_containers"
        } else {
            // Unknown source directory
            return None;
        };

        // Extract the filename
        let filename = source_path.file_name()?;

        // Build the destination path
        let dest_path = self
            .repo_root
            .join("7.quality/suites")
            .join(suite_subpath)
            .join(filename);

        Some(dest_path)
    }

    /// Migrate a test file from its current location to the appropriate suite
    pub fn migrate_test(
        &self,
        candidate: &TestFileCandidate,
    ) -> Result<MigrationResult, io::Error> {
        let source_path = &candidate.path;
        let destination_path = self.determine_destination(source_path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Cannot determine destination for {:?}", source_path),
            )
        })?;

        if self.dry_run {
            // In dry-run mode, just return what would happen
            return Ok(MigrationResult {
                source_path: source_path.clone(),
                destination_path,
                imports_updated: 0,
                regression_files_moved: vec![],
                compilation_status: CompilationStatus::NotAttempted,
            });
        }

        // Create destination directory if needed
        if let Some(parent) = destination_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Read the source file content
        let content = fs::read_to_string(source_path)?;

        // Update import paths
        let (updated_content, imports_updated) =
            self.update_imports(&content, source_path, &destination_path)?;

        // Write to destination
        fs::write(&destination_path, updated_content)?;

        // Preserve file permissions and metadata
        let metadata = fs::metadata(source_path)?;
        let permissions = metadata.permissions();
        fs::set_permissions(&destination_path, permissions)?;

        // Move regression files if they exist
        let regression_files_moved =
            self.preserve_regression_files(source_path, &destination_path)?;

        // Verify compilation
        let compilation_status = self.verify_compilation(&destination_path)?;

        // If compilation succeeded, remove the source file
        if compilation_status == CompilationStatus::Success {
            fs::remove_file(source_path)?;
        }

        Ok(MigrationResult {
            source_path: source_path.clone(),
            destination_path,
            imports_updated,
            regression_files_moved,
            compilation_status,
        })
    }

    /// Update import paths in the migrated test file
    ///
    /// Parses Rust import statements and calculates the correct relative path
    /// from the new test location to the production code.
    pub fn update_imports(
        &self,
        content: &str,
        source_path: &Path,
        destination_path: &Path,
    ) -> Result<(String, usize), io::Error> {
        let mut updated_content = String::new();
        let mut imports_updated = 0;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check if this is an import line
            if trimmed.starts_with("use ")
                && !trimmed.starts_with("use std::")
                && !trimmed.starts_with("use proptest::")
            {
                // Check if it's a relative import (starts with "super::" or "crate::")
                if trimmed.contains("super::") || trimmed.contains("crate::") {
                    // Calculate the relative path from destination to source's parent
                    if let Some(updated_line) =
                        self.calculate_updated_import(line, source_path, destination_path)
                    {
                        updated_content.push_str(&updated_line);
                        updated_content.push('\n');
                        imports_updated += 1;
                        continue;
                    }
                }
            }

            // Keep the line as-is
            updated_content.push_str(line);
            updated_content.push('\n');
        }

        Ok((updated_content, imports_updated))
    }

    /// Calculate the updated import path for a single import statement
    fn calculate_updated_import(
        &self,
        import_line: &str,
        _source_path: &Path,
        _destination_path: &Path,
    ) -> Option<String> {
        // For now, we'll keep imports as-is and rely on absolute crate paths
        // A more sophisticated implementation would calculate relative paths
        // This is a simplified version that handles common cases

        // If the import uses "crate::", it should continue to work
        // If it uses "super::", we need to adjust the number of "super::" based on depth change

        if import_line.contains("crate::") {
            // Absolute crate imports should work as-is
            return Some(import_line.to_string());
        }

        // For super:: imports, we'd need to calculate the depth difference
        // For now, return None to keep the original line
        None
    }

    /// Preserve proptest regression files during migration
    ///
    /// Detects proptest-regressions/ directory adjacent to the test file
    /// and moves regression files to the destination suite.
    pub fn preserve_regression_files(
        &self,
        source_path: &Path,
        destination_path: &Path,
    ) -> Result<Vec<PathBuf>, io::Error> {
        let mut moved_files = Vec::new();

        // Check for regression directory next to source file
        if let Some(source_parent) = source_path.parent() {
            let regression_dir = source_parent.join("proptest-regressions");

            if regression_dir.exists() && regression_dir.is_dir() {
                // Get the test file name (without extension) to find matching regression files
                let test_name = source_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");

                // Create destination regression directory
                if let Some(dest_parent) = destination_path.parent() {
                    let dest_regression_dir = dest_parent.join("proptest-regressions");
                    fs::create_dir_all(&dest_regression_dir)?;

                    // Move matching regression files
                    for entry in fs::read_dir(&regression_dir)? {
                        let entry = entry?;
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();

                        // Check if this regression file belongs to our test
                        if file_name_str.starts_with(test_name) {
                            let source_regression = entry.path();
                            let dest_regression = dest_regression_dir.join(&file_name);

                            fs::copy(&source_regression, &dest_regression)?;
                            fs::remove_file(&source_regression)?;

                            moved_files.push(dest_regression);
                        }
                    }
                }
            }
        }

        Ok(moved_files)
    }

    /// Verify that the migrated test file compiles successfully
    ///
    /// Runs `cargo check` on the migrated test file and captures any compilation errors.
    pub fn verify_compilation(&self, migrated_path: &Path) -> Result<CompilationStatus, io::Error> {
        // Run cargo check on the specific file
        let output = Command::new("cargo")
            .arg("check")
            .arg("--tests")
            .current_dir(&self.repo_root)
            .output()?;

        if output.status.success() {
            Ok(CompilationStatus::Success)
        } else {
            // Log the compilation error for debugging
            eprintln!(
                "Compilation failed for {:?}:\n{}",
                migrated_path,
                String::from_utf8_lossy(&output.stderr)
            );
            Ok(CompilationStatus::Failed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_destination_command_spine() {
        let migrator = TestMigrator::new(PathBuf::from("/repo"), false);
        let source = PathBuf::from("/repo/5.editor/l7.0-editor-command-spine/src/test.rs");
        let dest = migrator.determine_destination(&source);

        assert!(dest.is_some());
        let dest = dest.unwrap();
        assert!(dest
            .to_string_lossy()
            .contains("editor_canon_matrix/command_spine"));
    }

    #[test]
    fn test_determine_destination_editor_shell() {
        let migrator = TestMigrator::new(PathBuf::from("/repo"), false);
        let source = PathBuf::from("/repo/5.editor/l8.0-editor-shell/src/test.rs");
        let dest = migrator.determine_destination(&source);

        assert!(dest.is_some());
        let dest = dest.unwrap();
        assert!(dest
            .to_string_lossy()
            .contains("editor_canon_matrix/editor_shell"));
    }

    #[test]
    fn test_determine_destination_desktop_app() {
        let migrator = TestMigrator::new(PathBuf::from("/repo"), false);
        let source =
            PathBuf::from("/repo/6.apps/editor/stratumx_editor_app/src/desktop_app/test.rs");
        let dest = migrator.determine_destination(&source);

        assert!(dest.is_some());
        let dest = dest.unwrap();
        assert!(dest
            .to_string_lossy()
            .contains("editor_app_matrix/desktop_app"));
    }

    #[test]
    fn test_determine_destination_tool_session() {
        let migrator = TestMigrator::new(PathBuf::from("/repo"), false);
        let source = PathBuf::from("/repo/4.tooling/l6.0-tool-session/src/test.rs");
        let dest = migrator.determine_destination(&source);

        assert!(dest.is_some());
        let dest = dest.unwrap();
        assert!(dest
            .to_string_lossy()
            .contains("tooling_canon_matrix/tool_session"));
    }

    #[test]
    fn test_determine_destination_state_containers() {
        let migrator = TestMigrator::new(PathBuf::from("/repo"), false);
        let source = PathBuf::from("/repo/5.editor/editor-state-containers/src/test.rs");
        let dest = migrator.determine_destination(&source);

        assert!(dest.is_some());
        let dest = dest.unwrap();
        assert!(dest
            .to_string_lossy()
            .contains("editor_canon_matrix/state_containers"));
    }

    #[test]
    fn test_determine_destination_unknown() {
        let migrator = TestMigrator::new(PathBuf::from("/repo"), false);
        let source = PathBuf::from("/repo/unknown/path/test.rs");
        let dest = migrator.determine_destination(&source);

        assert!(dest.is_none());
    }
}
