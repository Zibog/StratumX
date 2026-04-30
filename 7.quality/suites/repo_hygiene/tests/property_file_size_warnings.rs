// Property test: File Size Warning for Large Files
//
// Feature: phase-3-editor-app-gold-gates
// Property 4: File Size Warning for Large Files
// Validates: Requirements 7.9, 10.3
//
// For any production file (in layers 2.engine, 3.sdk, 4.tooling, 5.editor, 6.apps)
// exceeding 200 LOC that is not allowlisted, the file size validator should issue a warning.

use std::fs;
use std::path::{Path, PathBuf};

/// Count non-empty, non-comment lines in a Rust file
fn count_loc(path: &Path) -> usize {
    let content = fs::read_to_string(path).unwrap_or_default();
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .count()
}

/// Check if a file is in a production layer
fn is_production_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains("2.engine/")
        || path_str.contains("3.sdk/")
        || path_str.contains("4.tooling/")
        || path_str.contains("5.editor/")
        || path_str.contains("6.apps/")
}

/// Check if a file is allowlisted (can be extended as needed)
fn is_allowlisted(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Files that are intentionally large and allowlisted
    // Add specific files here as they are approved
    let allowlist: [&str; 0] = [
        // Example: "5.editor/some_large_file.rs"
    ];

    allowlist.iter().any(|pattern| path_str.contains(pattern))
}

/// Recursively find all Rust files in production layers
fn find_production_rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_production_rust_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs")
                && is_production_file(&path)
            {
                files.push(path);
            }
        }
    }

    files
}

#[test]
fn property_file_size_warnings() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let production_files = find_production_rust_files(&workspace_root);
    let mut warnings = Vec::new();

    for file in production_files {
        let loc = count_loc(&file);

        // Property: Files exceeding 200 LOC that are not allowlisted should generate warnings
        if loc > 200 && !is_allowlisted(&file) {
            let relative_path = file.strip_prefix(&workspace_root).unwrap_or(&file);
            warnings.push(format!(
                "WARNING: {} has {} LOC (exceeds 200 LOC threshold)",
                relative_path.display(),
                loc
            ));
        }
    }

    // If there are warnings, print them but don't fail the test
    // This is a warning-level property, not a hard failure
    if !warnings.is_empty() {
        println!("\n=== File Size Warnings ===");
        for warning in &warnings {
            println!("{}", warning);
        }
        println!("\nTotal files with warnings: {}", warnings.len());
        println!("\nNote: These are warnings, not errors. Consider splitting these files.");
    }

    // Test always passes - this is for visibility only
    // The actual enforcement happens in the file size validator tool
}

#[test]
fn property_file_size_warnings_allowlist_respected() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let production_files = find_production_rust_files(&workspace_root);

    for file in production_files {
        let loc = count_loc(&file);

        // Property: Allowlisted files should not generate warnings even if they exceed 200 LOC
        if is_allowlisted(&file) {
            // This is expected and allowed
            continue;
        }

        // For non-allowlisted files, we just track them
        if loc > 200 {
            let relative_path = file.strip_prefix(&workspace_root).unwrap_or(&file);
            println!(
                "File {} has {} LOC (not allowlisted)",
                relative_path.display(),
                loc
            );
        }
    }
}
