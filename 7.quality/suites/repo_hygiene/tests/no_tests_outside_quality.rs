// Gate: No tests outside 7.quality

use std::fs;
use std::path::{Path, PathBuf};

fn scan_for_test_dirs(root: &Path, exclude_dirs: &[&str]) -> Vec<PathBuf> {
    let mut violations = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if exclude_dirs.contains(&file_name) {
                continue;
            }

            if path.is_dir() {
                if file_name == "tests" {
                    // Skip evidence tests in docs
                    if !path.to_str().unwrap().contains("evidence") {
                        violations.push(path.clone());
                    }
                }
                violations.extend(scan_for_test_dirs(&path, exclude_dirs));
            }
        }
    }

    violations
}

fn scan_for_test_attrs(root: &Path, exclude_dirs: &[&str]) -> Vec<(PathBuf, usize)> {
    let mut violations = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if exclude_dirs.contains(&file_name) {
                continue;
            }

            if path.is_dir() {
                violations.extend(scan_for_test_attrs(&path, exclude_dirs));
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for (line_num, line) in content.lines().enumerate() {
                        let trimmed = line.trim();
                        if trimmed == "#[test]"
                            || trimmed == "#[cfg(test)]"
                            || trimmed.starts_with("mod tests")
                        {
                            violations.push((path.clone(), line_num + 1));
                        }
                    }
                }
            }
        }
    }

    violations
}

#[test]
fn no_tests_directories_outside_quality() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let exclude = &["7.quality", "target", ".git", "node_modules"];
    let violations = scan_for_test_dirs(workspace_root, exclude);

    if !violations.is_empty() {
        eprintln!("\n❌ Found tests/ directories outside 7.quality:");
        for path in &violations {
            eprintln!(
                "  - {}",
                path.strip_prefix(workspace_root).unwrap().display()
            );
        }
        panic!(
            "Tests must be in 7.quality, found {} violations",
            violations.len()
        );
    }
}

#[test]
fn no_test_attributes_outside_quality() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let exclude = &["7.quality", "target", ".git", "node_modules"];
    let violations = scan_for_test_attrs(workspace_root, exclude);

    if !violations.is_empty() {
        eprintln!("\n❌ Found #[test] or #[cfg(test)] outside 7.quality:");
        for (path, line) in &violations {
            eprintln!(
                "  - {}:{}",
                path.strip_prefix(workspace_root).unwrap().display(),
                line
            );
        }
        panic!(
            "Test attributes must be in 7.quality, found {} violations",
            violations.len()
        );
    }
}
