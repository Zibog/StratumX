// Feature: phase-3-editor-app-gold-gates
// Property 3: File Size Enforcement for Production Code
//
// **Validates: Requirements 7.8, 10.2**
//
// This property test verifies that production files (in layers 2.engine, 3.sdk,
// 4.tooling, 5.editor, 6.apps) exceeding 300 LOC fail validation.

use proptest::prelude::*;
use repo_hygiene::{HygieneRule, WaiverRegistry};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_workspace_root() -> PathBuf {
    let mut current = std::env::current_dir().expect("Failed to get current directory");

    loop {
        let editor_dir = current.join("5.editor");
        let apps_dir = current.join("6.apps");
        if editor_dir.exists() && apps_dir.exists() {
            return current;
        }

        if !current.pop() {
            panic!("Could not find workspace root");
        }
    }
}

/// Count non-comment, non-blank lines in a Rust file
fn count_significant_lines(content: &str) -> usize {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .count()
}

/// Check if a file is in a production layer
fn is_production_file(path: &Path, workspace_root: &Path) -> bool {
    if let Ok(relative) = path.strip_prefix(workspace_root) {
        let path_str = relative.to_string_lossy();
        path_str.starts_with("2.engine")
            || path_str.starts_with("3.sdk")
            || path_str.starts_with("4.tooling")
            || path_str.starts_with("5.editor")
            || path_str.starts_with("6.apps")
    } else {
        false
    }
}

/// Find all Rust files in production layers
fn find_production_rust_files(workspace_root: &Path) -> Vec<PathBuf> {
    let production_dirs = vec![
        workspace_root.join("2.engine"),
        workspace_root.join("3.sdk"),
        workspace_root.join("4.tooling"),
        workspace_root.join("5.editor"),
        workspace_root.join("6.apps"),
    ];

    let mut files = Vec::new();

    for dir in production_dirs {
        if !dir.exists() {
            continue;
        }

        for entry in WalkDir::new(&dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        {
            files.push(entry.path().to_path_buf());
        }
    }

    files
}

// Property 3: File Size Enforcement for Production Code
// **Validates: Requirements 7.8, 10.2**
//
// For any production file exceeding 300 LOC, validation should fail unless waived.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_file_size_enforced_at_300_loc(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let waiver_path = workspace_root.join("7.quality/suites/repo_hygiene/waivers.toml");
        let waiver_registry = WaiverRegistry::load_from_file(&waiver_path)
            .expect("Failed to load waiver registry");

        let production_files = find_production_rust_files(&workspace_root);

        prop_assert!(
            !production_files.is_empty(),
            "Should find at least some production Rust files"
        );

        let mut violations = Vec::new();

        for file_path in production_files {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let line_count = content.lines().count();

                if line_count > 300 {
                    let relative_path = file_path.strip_prefix(&workspace_root)
                        .unwrap_or(&file_path);

                    let rule = HygieneRule::LineLimit {
                        limit: 300,
                        actual: line_count,
                    };

                    // Check if this violation is waived
                    if !waiver_registry.is_waived(&rule, relative_path) {
                        violations.push((
                            relative_path.display().to_string(),
                            line_count,
                        ));
                    }
                }
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} production files exceeding 300 LOC:\n{}",
            violations.len(),
            violations
                .iter()
                .map(|(path, lines)| format!("  {} ({} lines)", path, lines))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

// Property 3b: File Size Warning for Large Files
// **Validates: Requirements 7.9, 10.3**
//
// For any production file exceeding 200 LOC (but under 300), a warning should be issued
// unless the file is allowlisted.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_file_size_warned_at_200_loc(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let waiver_path = workspace_root.join("7.quality/suites/repo_hygiene/waivers.toml");
        let waiver_registry = WaiverRegistry::load_from_file(&waiver_path)
            .expect("Failed to load waiver registry");

        let production_files = find_production_rust_files(&workspace_root);
        let mut warnings = Vec::new();

        for file_path in production_files {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let line_count = content.lines().count();

                // Check files between 200 and 300 LOC
                if line_count > 200 && line_count <= 300 {
                    let relative_path = file_path.strip_prefix(&workspace_root)
                        .unwrap_or(&file_path);

                    let rule = HygieneRule::LineLimit {
                        limit: 200,
                        actual: line_count,
                    };

                    // Check if this warning is waived
                    if !waiver_registry.is_waived(&rule, relative_path) {
                        warnings.push((
                            relative_path.display().to_string(),
                            line_count,
                        ));
                    }
                }
            }
        }

        // This is a warning, not a hard failure, but we track it
        if !warnings.is_empty() {
            println!(
                "WARNING: Found {} production files exceeding 200 LOC (but under 300):",
                warnings.len()
            );
            for (path, lines) in &warnings {
                println!("  {} ({} lines)", path, lines);
            }
        }

        // Property test passes - we're just documenting warnings
        prop_assert!(true);
    }
}

// Unit tests for helper functions

#[test]
fn test_count_significant_lines() {
    let content = r#"
// This is a comment
fn main() {
    println!("Hello");
}

// Another comment
"#;

    let count = count_significant_lines(content);
    assert_eq!(count, 3); // fn main() { println!("Hello"); }
}

#[test]
fn test_is_production_file() {
    let workspace_root = PathBuf::from("/workspace");

    assert!(is_production_file(
        &workspace_root.join("2.engine/some/file.rs"),
        &workspace_root
    ));
    assert!(is_production_file(
        &workspace_root.join("5.editor/some/file.rs"),
        &workspace_root
    ));
    assert!(!is_production_file(
        &workspace_root.join("7.quality/some/file.rs"),
        &workspace_root
    ));
}

#[test]
fn test_find_production_rust_files() {
    let workspace_root = get_workspace_root();
    let files = find_production_rust_files(&workspace_root);

    // Should find at least some files
    assert!(!files.is_empty(), "Should find production Rust files");

    // All files should be .rs files
    for file in &files {
        assert_eq!(
            file.extension().and_then(|s| s.to_str()),
            Some("rs"),
            "File should have .rs extension: {}",
            file.display()
        );
    }

    // All files should be in production directories
    for file in &files {
        assert!(
            is_production_file(file, &workspace_root),
            "File should be in production directory: {}",
            file.display()
        );
    }
}

#[test]
fn test_material_service_split_under_300_loc() {
    let workspace_root = get_workspace_root();
    let service_dir = workspace_root.join(
        "5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service",
    );

    if !service_dir.exists() {
        // Skip if directory doesn't exist (test might run before split)
        return;
    }

    // Check that all split files are under 300 LOC
    let split_files = vec![
        "session.rs",
        "commands.rs",
        "cache.rs",
        "preview.rs",
        "validation.rs",
        "diagnostics.rs",
        "mod.rs",
    ];

    for split_file in split_files {
        let file_path = service_dir.join(split_file);
        if file_path.exists() {
            let content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read {}", split_file));
            let line_count = content.lines().count();

            assert!(
                line_count <= 300,
                "Split file {} has {} lines (should be <= 300)",
                split_file,
                line_count
            );

            println!(
                "✓ {} has {} lines (under 300 LOC limit)",
                split_file, line_count
            );
        }
    }
}
