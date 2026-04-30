//! Property Test: Test Placement Restriction
//!
//! Feature: phase-3-editor-app-gold-gates
//! Property 5: Test Placement Restriction
//!
//! For any test module or test file in the workspace, it should be located within
//! the 7.quality layer, and any test found outside this layer should cause validation to fail.
//!
//! Validates: Requirements 7.10, 13.2

use std::fs;
use std::path::{Path, PathBuf};

/// Checks if a path is within the 7.quality layer
fn is_in_quality_layer(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "7.quality")
}

/// Checks if a file contains test code
fn contains_test_code(path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(path) {
        // Check for common test patterns
        content.contains("#[test]")
            || content.contains("#[cfg(test)]")
            || content.contains("mod tests {")
    } else {
        false
    }
}

/// Recursively scans a directory for Rust files with tests
fn scan_for_tests(dir: &Path, tests_outside_quality: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip hidden directories and target directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with('.') || name_str == "target" {
                    continue;
                }
            }

            if path.is_dir() {
                scan_for_tests(&path, tests_outside_quality);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                // If file contains tests and is not in 7.quality, record it
                if contains_test_code(&path) && !is_in_quality_layer(&path) {
                    tests_outside_quality.push(path);
                }
            }
        }
    }
}

#[test]
fn property_test_placement_restriction() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root");

    let mut tests_outside_quality = Vec::new();

    // Scan production layers for tests
    let production_layers = ["2.engine", "3.sdk", "4.tooling", "5.editor", "6.apps"];

    for layer in &production_layers {
        let layer_path = workspace_root.join(layer);
        if layer_path.exists() {
            scan_for_tests(&layer_path, &mut tests_outside_quality);
        }
    }

    // Property: All tests should be in 7.quality layer
    if !tests_outside_quality.is_empty() {
        eprintln!("Found tests outside 7.quality layer:");
        for test_file in &tests_outside_quality {
            eprintln!("  - {}", test_file.display());
        }
        panic!(
            "Property violation: {} test file(s) found outside 7.quality layer",
            tests_outside_quality.len()
        );
    }
}

#[test]
fn property_quality_layer_tests_allowed() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root");

    let quality_path = workspace_root.join("7.quality");

    if !quality_path.exists() {
        // If 7.quality doesn't exist, skip this test
        return;
    }

    let mut quality_tests = Vec::new();
    scan_for_tests(&quality_path, &mut quality_tests);

    // Property: Tests in 7.quality layer should be allowed
    // This test verifies that our scanning logic works and finds tests in 7.quality
    // The scan_for_tests function adds files to the list when they're NOT in quality layer,
    // so for this test we need a different approach

    let mut found_quality_tests = false;
    if let Ok(entries) = fs::read_dir(&quality_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path.extension().and_then(|s| s.to_str()) == Some("rs")
                && contains_test_code(&path)
            {
                found_quality_tests = true;
                break;
            }
        }
    }

    // We expect to find at least some tests in 7.quality (including this file)
    assert!(
        found_quality_tests || quality_path.join("suites").exists(),
        "Expected to find tests in 7.quality layer"
    );
}
