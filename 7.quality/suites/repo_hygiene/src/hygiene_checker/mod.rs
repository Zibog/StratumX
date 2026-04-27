// Hygiene Checker Module.

mod allow_attributes;
mod line_limits;
mod prohibitions;
mod report;
mod test_in_src;
mod todo_comments;

use crate::models::HygieneReport;
use crate::waiver_registry::WaiverRegistry;
use std::path::PathBuf;
use stratumx_repo_hygiene_support::production_rust_files;

/// Main hygiene checker that enforces code quality rules.
pub struct HygieneChecker {
    repo_root: PathBuf,
    waiver_registry: WaiverRegistry,
}

impl HygieneChecker {
    /// Create a new hygiene checker.
    pub fn new(repo_root: PathBuf, waiver_registry: WaiverRegistry) -> Self {
        Self {
            repo_root,
            waiver_registry,
        }
    }

    fn walk_production_code(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let files = production_rust_files(&self.repo_root);
        if files.is_empty() {
            Ok(self.walk_fixture_repo(&self.repo_root))
        } else {
            Ok(files)
        }
    }

    fn relative_path<'a>(&'a self, entry: &'a std::path::Path) -> &'a std::path::Path {
        entry.strip_prefix(&self.repo_root).unwrap_or(entry)
    }

    fn walk_fixture_repo(&self, dir: &std::path::Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return files,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    if name.starts_with('.') || name == "target" || name == "7.quality" {
                        continue;
                    }
                }
                files.extend(self.walk_fixture_repo(&path));
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs")
                && path.ancestors().any(|ancestor| {
                    ancestor.file_name().and_then(|name| name.to_str()) == Some("src")
                })
            {
                files.push(path);
            }
        }

        files
    }

    /// Run all hygiene checks and generate a comprehensive report.
    pub fn run_all_checks(&self) -> HygieneReport {
        report::run_all_checks(self)
    }
}
