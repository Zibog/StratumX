// Code Cleaner Module.

mod domain_logic;
mod host_bypass;
mod registration_blob;

use crate::models::CleanupReport;
use std::fs;
use std::path::{Path, PathBuf};

/// Code cleaner that identifies code quality issues requiring manual refactoring.
pub struct CodeCleaner {
    repo_root: PathBuf,
}

impl CodeCleaner {
    /// Create a new code cleaner for the given repository root.
    pub fn new(repo_root: PathBuf) -> Self {
        Self { repo_root }
    }

    /// Generate a comprehensive cleanup report.
    pub fn generate_cleanup_report(&self) -> CleanupReport {
        CleanupReport::new(
            self.identify_host_bypasses(),
            self.identify_registration_blobs(),
            self.identify_domain_logic_in_ui(),
        )
    }

    fn desktop_app_path(&self) -> PathBuf {
        self.repo_root
            .join("6.apps/editor/stratumx_editor_app/src/desktop_app")
    }

    fn command_spine_path(&self) -> PathBuf {
        self.repo_root
            .join("5.editor/l7.0-editor-command-spine/src")
    }

    fn scan_rust_files(&self, dir: &Path) -> std::io::Result<Vec<PathBuf>> {
        let mut rust_files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                rust_files.extend(self.scan_rust_files(&path)?);
            } else if path.extension().and_then(|segment| segment.to_str()) == Some("rs") {
                rust_files.push(path);
            }
        }
        Ok(rust_files)
    }
}

#[cfg(test)]
mod tests;
