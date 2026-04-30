//! Property 21: Quality Suite Centralization
//! Property 31: Quality Suite Execution from Root
//!
//! **Validates: Requirements 10.1, 10.3, 14.5**

use std::fs;
use std::path::{Path, PathBuf};

/// Helper function to get workspace root
fn get_workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root")
        .to_path_buf()
}

/// Check for heavy tests (integration, property, matrix, hygiene, benchmark) outside 7.quality
fn find_misplaced_heavy_tests(repo_root: &Path) -> Vec<PathBuf> {
    let mut misplaced = Vec::new();

    // Directories to scan (exclude 7.quality)
    let scan_dirs = vec![
        repo_root.join("2.engine"),
        repo_root.join("3.sdk"),
        repo_root.join("4.tooling"),
        repo_root.join("5.editor"),
        repo_root.join("6.apps"),
    ];

    for dir in scan_dirs {
        if !dir.exists() {
            continue;
        }

        // Scan for integration tests (tests/ directories)
        scan_for_tests_directories(&dir, &mut misplaced);

        // Scan for property-based tests (proptest/quickcheck usage)
        scan_for_property_tests(&dir, &mut misplaced);

        // Scan for benchmark tests (benches/ directories)
        scan_for_benches_directories(&dir, &mut misplaced);
    }

    misplaced
}

/// Scan for tests/ directories
fn scan_for_tests_directories(dir: &Path, misplaced: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip target and hidden directories
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if name_str == "target" || name_str.starts_with('.') {
                        continue;
                    }
                }

                // Check if this is a tests/ directory
                if path.file_name() == Some(std::ffi::OsStr::new("tests")) {
                    // Check if it contains any test files
                    if let Ok(test_entries) = fs::read_dir(&path) {
                        for test_entry in test_entries.flatten() {
                            let test_path = test_entry.path();
                            if test_path.extension() == Some(std::ffi::OsStr::new("rs")) {
                                misplaced.push(test_path);
                            }
                        }
                    }
                } else {
                    // Recurse into subdirectories
                    scan_for_tests_directories(&path, misplaced);
                }
            }
        }
    }
}

/// Scan for property-based tests (proptest/quickcheck usage)
fn scan_for_property_tests(dir: &Path, misplaced: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip target and hidden directories
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if name_str == "target" || name_str.starts_with('.') {
                        continue;
                    }
                }
                // Recurse into subdirectories
                scan_for_property_tests(&path, misplaced);
            } else if path.extension() == Some(std::ffi::OsStr::new("rs")) {
                // Read file content
                if let Ok(content) = fs::read_to_string(&path) {
                    // Check for proptest/quickcheck usage
                    let has_proptest =
                        content.contains("use proptest::") || content.contains("proptest!");
                    let has_quickcheck =
                        content.contains("use quickcheck::") || content.contains("quickcheck!");

                    if has_proptest || has_quickcheck {
                        misplaced.push(path);
                    }
                }
            }
        }
    }
}

/// Scan for benches/ directories
fn scan_for_benches_directories(dir: &Path, misplaced: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip target and hidden directories
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if name_str == "target" || name_str.starts_with('.') {
                        continue;
                    }
                }

                // Check if this is a benches/ directory
                if path.file_name() == Some(std::ffi::OsStr::new("benches")) {
                    // Check if it contains any benchmark files
                    if let Ok(bench_entries) = fs::read_dir(&path) {
                        for bench_entry in bench_entries.flatten() {
                            let bench_path = bench_entry.path();
                            if bench_path.extension() == Some(std::ffi::OsStr::new("rs")) {
                                misplaced.push(bench_path);
                            }
                        }
                    }
                } else {
                    // Recurse into subdirectories
                    scan_for_benches_directories(&path, misplaced);
                }
            }
        }
    }
}

/// Check if stratumx_quality_tasks is accessible from workspace
fn is_quality_tasks_accessible(repo_root: &Path) -> bool {
    let quality_tasks_path = repo_root.join("7.quality/tasks/stratumx_quality_tasks/Cargo.toml");
    quality_tasks_path.exists()
}

// ============================================================================
// Property 21: Quality Suite Centralization
// ============================================================================

#[test]
fn property_21_quality_suite_centralization() {
    // **Validates: Requirements 10.1**
    //
    // For any heavy test (integration test, property test, matrix test, hygiene test),
    // it must be located in 7.quality packages, not in other layers.

    let repo_root = get_workspace_root();
    let misplaced_tests = find_misplaced_heavy_tests(&repo_root);

    assert!(
        misplaced_tests.is_empty(),
        "Found {} misplaced heavy test(s) outside 7.quality:\n{}",
        misplaced_tests.len(),
        misplaced_tests
            .iter()
            .map(|p| format!("  - {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

// ============================================================================
// Property 31: Quality Suite Execution from Root
// ============================================================================

#[test]
fn property_31_quality_suite_execution_from_root() {
    // **Validates: Requirements 10.3, 14.5**
    //
    // For any quality suite in 7.quality, it must be executable from the repository root
    // using standard root commands without requiring manual directory changes.

    let repo_root = get_workspace_root();

    // Verify stratumx_quality_tasks is accessible from workspace
    assert!(
        is_quality_tasks_accessible(&repo_root),
        "stratumx_quality_tasks must be accessible from workspace root"
    );

    // Verify root command scripts exist
    let verify_script = repo_root.join("tools/verify.ps1");
    let full_script = repo_root.join("tools/full.ps1");
    let smoke_script = repo_root.join("tools/smoke.ps1");

    assert!(
        verify_script.exists(),
        "tools/verify.ps1 must exist for running quality suites from root"
    );
    assert!(
        full_script.exists(),
        "tools/full.ps1 must exist for running quality suites from root"
    );
    assert!(
        smoke_script.exists(),
        "tools/smoke.ps1 must exist for running quality suites from root"
    );

    // Verify scripts use correct invocation pattern (cargo run -p stratumx_quality_tasks)
    if let Ok(verify_content) = fs::read_to_string(&verify_script) {
        assert!(
            verify_content.contains("stratumx_quality_tasks"),
            "tools/verify.ps1 must invoke stratumx_quality_tasks"
        );
    }

    if let Ok(full_content) = fs::read_to_string(&full_script) {
        assert!(
            full_content.contains("stratumx_quality_tasks"),
            "tools/full.ps1 must invoke stratumx_quality_tasks"
        );
    }

    if let Ok(smoke_content) = fs::read_to_string(&smoke_script) {
        assert!(
            smoke_content.contains("stratumx_quality_tasks"),
            "tools/smoke.ps1 must invoke stratumx_quality_tasks"
        );
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[test]
fn test_quality_suites_directory_exists() {
    let suites_dir = std::path::Path::new("7.quality/suites");
    assert!(suites_dir.exists(), "7.quality/suites directory must exist");
}

#[test]
fn test_quality_suites_are_comprehensive() {
    let suites_dir = std::path::Path::new("7.quality/suites");
    let suite_count = std::fs::read_dir(suites_dir)
        .expect("Failed to read suites directory")
        .filter(|e| {
            e.as_ref()
                .map(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .unwrap_or(false)
        })
        .count();

    assert!(
        suite_count >= 20,
        "Should have at least 20 quality suites, found {}",
        suite_count
    );
}

#[test]
fn test_no_tests_dir_in_sdk() {
    // Verify SDK packages don't have tests/ directories
    let sdk_dir = std::path::Path::new("3.sdk");
    if let Ok(entries) = std::fs::read_dir(sdk_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let tests_dir = entry.path().join("tests");
                assert!(
                    !tests_dir.exists(),
                    "SDK package {:?} should not have tests/ directory",
                    entry.file_name()
                );
            }
        }
    }
}

#[test]
fn test_get_workspace_root() {
    let root = get_workspace_root();
    assert!(root.exists(), "Workspace root must exist");
    assert!(
        root.join("Cargo.toml").exists(),
        "Workspace root must contain Cargo.toml"
    );
}

#[test]
fn test_find_misplaced_heavy_tests() {
    let repo_root = get_workspace_root();
    let misplaced = find_misplaced_heavy_tests(&repo_root);
    // This test just verifies the function runs without panicking
    // The actual assertion is in property_21_quality_suite_centralization
    println!("Found {} misplaced tests", misplaced.len());
}
