use crate::waiver_registry::WaiverRegistry;
use std::path::{Path, PathBuf};

mod hygiene_checks;
mod registration_scan;
mod test_discovery;
mod ui_pattern_scan;

/// Main discovery scanner that traverses the repository.
pub struct DiscoveryScanner {
    repo_root: PathBuf,
    waiver_registry: WaiverRegistry,
}

impl DiscoveryScanner {
    /// Create a new discovery scanner.
    pub fn new(repo_root: PathBuf, waiver_registry: WaiverRegistry) -> Self {
        Self {
            repo_root,
            waiver_registry,
        }
    }

    fn walk_repo(&self) -> impl Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>> + '_ {
        let quality_path = self.repo_root.join("7.quality");
        walkdir::WalkDir::new(&self.repo_root)
            .into_iter()
            .filter_entry(move |entry| {
                let path = entry.path();
                if path == self.repo_root {
                    return true;
                }

                let file_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                if file_name.starts_with('.') && file_name != "." {
                    return false;
                }
                if file_name == "target" {
                    return false;
                }
                if path.starts_with(&quality_path) && path != quality_path {
                    return false;
                }

                true
            })
    }

    fn desktop_app_path(&self) -> PathBuf {
        self.repo_root
            .join("6.apps/editor/stratumx_editor_app/src/desktop_app")
    }

    fn command_spine_path(&self) -> PathBuf {
        self.repo_root
            .join("5.editor/l7.0-editor-command-spine/src")
    }

    fn is_rust_file(path: &Path) -> bool {
        path.extension().is_some_and(|ext| ext == "rs")
    }
}
