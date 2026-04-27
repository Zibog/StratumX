use super::{WaiverEntry, WaiverRegistry};
use crate::models::WaiverError;
use std::path::{Path, PathBuf};

pub fn validate_waivers(registry: &WaiverRegistry, repo_root: &Path) -> Vec<WaiverError> {
    let mut errors = Vec::new();
    validate_entries(
        &registry.line_limit_waivers,
        repo_root,
        "LineLimit",
        &mut errors,
    );
    validate_entries(
        &registry.allow_attr_waivers,
        repo_root,
        "AllowAttribute",
        &mut errors,
    );
    validate_entries(
        &registry.test_in_src_waivers,
        repo_root,
        "TestFileInSrc",
        &mut errors,
    );
    validate_entries(
        &registry.execute_bridge_waivers,
        repo_root,
        "ExecuteBridge",
        &mut errors,
    );
    errors
}

fn validate_entries(
    entries: &[WaiverEntry],
    repo_root: &Path,
    rule: &str,
    errors: &mut Vec<WaiverError>,
) {
    for entry in entries {
        let full_path = repo_root.join(&entry.path);
        if !full_path.exists() {
            errors.push(WaiverError {
                path: PathBuf::from(&entry.path),
                rule: rule.to_string(),
                error: format!("File does not exist: {}", entry.path),
            });
        }
    }
}
