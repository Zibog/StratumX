// Property-based and unit tests for waiver registry
//
// This module tests the waiver registry system including:
// - TOML parsing and validation
// - Waiver lookup and checking
// - Stale waiver detection
// - Property-based tests for correctness properties

use proptest::prelude::*;
use repo_hygiene::{HygieneRule, WaiverEntry, WaiverRegistry};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ============================================================================
// Property Test Generators
// ============================================================================

/// Generate a valid waiver entry with non-empty justification
fn arb_waiver_entry() -> impl Strategy<Value = WaiverEntry> {
    ("[a-z0-9_]+(/[a-z0-9_]+)*\\.rs", "[A-Za-z0-9][A-Za-z0-9 ]+").prop_map(
        |(path, justification)| WaiverEntry {
            path,
            justification,
        },
    )
}

/// Generate a waiver entry with empty justification (invalid)
fn arb_invalid_waiver_entry() -> impl Strategy<Value = WaiverEntry> {
    "[a-z0-9_]+(/[a-z0-9_]+)*\\.rs".prop_map(|path| WaiverEntry {
        path,
        justification: String::new(),
    })
}

/// Generate a valid waiver registry
fn arb_waiver_registry() -> impl Strategy<Value = WaiverRegistry> {
    (
        prop::collection::vec(arb_waiver_entry(), 0..5),
        prop::collection::vec(arb_waiver_entry(), 0..5),
        prop::collection::vec(arb_waiver_entry(), 0..5),
        prop::collection::vec(arb_waiver_entry(), 0..5),
    )
        .prop_map(
            |(
                line_limit_waivers,
                allow_attr_waivers,
                test_in_src_waivers,
                execute_bridge_waivers,
            )| WaiverRegistry {
                line_limit_waivers,
                allow_attr_waivers,
                test_in_src_waivers,
                execute_bridge_waivers,
            },
        )
}

// ============================================================================
// Property Tests
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 21: Waiver Entry Validation
// For any entry in the Waiver_Registry, if the referenced file does not exist,
// the Hygiene_Checker should report a waiver validation error.
// Validates: Requirements 9.6
proptest! {
    #[test]
    fn prop_waiver_entry_validation(registry in arb_waiver_registry()) {
        // Create a temporary directory (empty, so no files exist)
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Validate waivers - all should fail since files don't exist
        let errors = registry.validate_waivers(repo_root);

        // Every waiver entry should produce an error
        let expected_errors = registry.line_limit_waivers.len()
            + registry.allow_attr_waivers.len()
            + registry.test_in_src_waivers.len()
            + registry.execute_bridge_waivers.len();
        prop_assert_eq!(errors.len(), expected_errors);

        // Each error should reference a non-existent file
        for error in &errors {
            let full_path = repo_root.join(&error.path);
            prop_assert!(!full_path.exists(), "Error should be for non-existent file");
        }
    }
}

// Feature: repo-sanitization-phase-1, Property 21: Waiver Entry Validation (positive case)
// For any entry in the Waiver_Registry, if the referenced file exists,
// no validation error should be reported.
proptest! {
    #[test]
    fn prop_waiver_entry_validation_existing_files(registry in arb_waiver_registry()) {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Create all files referenced in the registry
        for entry in &registry.line_limit_waivers {
            let file_path = repo_root.join(&entry.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&file_path, "// test file").unwrap();
        }

        for entry in &registry.allow_attr_waivers {
            let file_path = repo_root.join(&entry.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&file_path, "// test file").unwrap();
        }

        for entry in &registry.test_in_src_waivers {
            let file_path = repo_root.join(&entry.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&file_path, "// test file").unwrap();
        }

        for entry in &registry.execute_bridge_waivers {
            let file_path = repo_root.join(&entry.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&file_path, "// test file").unwrap();
        }

        // Validate waivers - should have no errors
        let errors = registry.validate_waivers(repo_root);
        prop_assert_eq!(errors.len(), 0, "No errors expected when all files exist");
    }
}

// Feature: repo-sanitization-phase-1, Property 22: Waiver Mechanism Correctness
// For any file listed in the Waiver_Registry for a specific rule, the Hygiene_Checker
// should not report violations of that rule for that file.
// Validates: Requirements 9.5
proptest! {
    #[test]
    fn prop_waiver_mechanism_correctness(
        waived_files in prop::collection::vec(arb_waiver_entry(), 1..10),
        non_waived_files in prop::collection::vec("[a-z0-9_]+(/[a-z0-9_]+)*\\.rs", 1..10)
    ) {
        // Create registry with waived files
        let registry = WaiverRegistry {
            line_limit_waivers: waived_files.clone(),
            allow_attr_waivers: Vec::new(),
            test_in_src_waivers: Vec::new(),
            execute_bridge_waivers: Vec::new(),
        };

        let rule = HygieneRule::LineLimit {
            limit: 200,
            actual: 250,
        };

        // All waived files should return true for is_waived
        for entry in &waived_files {
            let path = Path::new(&entry.path);
            prop_assert!(
                registry.is_waived(&rule, path),
                "Waived file '{}' should be waived",
                entry.path
            );
        }

        // Non-waived files should return false
        for file_path in &non_waived_files {
            // Skip if this file happens to be in the waived list
            if waived_files.iter().any(|w| w.path == *file_path) {
                continue;
            }

            let path = Path::new(file_path);
            prop_assert!(
                !registry.is_waived(&rule, path),
                "Non-waived file '{}' should not be waived",
                file_path
            );
        }
    }
}

// Feature: repo-sanitization-phase-1, Property 23: Waiver Entry Format Validation
// For any entry in the Waiver_Registry without a justification field, the registry
// loader should reject the entry as invalid.
// Validates: Requirements 9.4
proptest! {
    #[test]
    fn prop_waiver_format_validation(
        valid_entries in prop::collection::vec(arb_waiver_entry(), 0..5),
        invalid_entries in prop::collection::vec(arb_invalid_waiver_entry(), 1..5)
    ) {
        let temp_dir = TempDir::new().unwrap();
        let waiver_file = temp_dir.path().join("waivers.toml");

        // Create TOML with mix of valid and invalid entries
        let mut toml_content = String::new();

        // Add valid entries
        for entry in &valid_entries {
            toml_content.push_str(&format!(
                "[[line_limit_waivers]]\npath = \"{}\"\njustification = \"{}\"\n\n",
                entry.path, entry.justification
            ));
        }

        // Add at least one invalid entry (empty justification)
        for entry in &invalid_entries {
            toml_content.push_str(&format!(
                "[[line_limit_waivers]]\npath = \"{}\"\njustification = \"\"\n\n",
                entry.path
            ));
        }

        fs::write(&waiver_file, toml_content).unwrap();

        // Loading should fail due to invalid entries
        let result = WaiverRegistry::load_from_file(&waiver_file);
        prop_assert!(
            result.is_err(),
            "Registry with empty justification should fail to load"
        );

        let error_msg = result.unwrap_err();
        prop_assert!(
            error_msg.contains("missing justification"),
            "Error message should mention missing justification, got: {}",
            error_msg
        );
    }
}

// Feature: repo-sanitization-phase-1, Property 23: Waiver Entry Format Validation (positive case)
// For any entry in the Waiver_Registry with a non-empty justification field, the registry
// loader should accept the entry as valid.
proptest! {
    #[test]
    fn prop_waiver_format_validation_valid(
        entries in prop::collection::vec(arb_waiver_entry(), 1..10)
    ) {
        let temp_dir = TempDir::new().unwrap();
        let waiver_file = temp_dir.path().join("waivers.toml");

        // Create TOML with only valid entries
        let mut toml_content = String::new();
        for entry in &entries {
            toml_content.push_str(&format!(
                "[[line_limit_waivers]]\npath = \"{}\"\njustification = \"{}\"\n\n",
                entry.path, entry.justification
            ));
        }

        fs::write(&waiver_file, toml_content).unwrap();

        // Loading should succeed
        let result = WaiverRegistry::load_from_file(&waiver_file);
        prop_assert!(
            result.is_ok(),
            "Registry with valid entries should load successfully"
        );

        let registry = result.unwrap();
        prop_assert_eq!(
            registry.line_limit_waivers.len(),
            entries.len(),
            "All entries should be loaded"
        );
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[test]
fn test_load_from_file_valid_toml() {
    let temp_dir = TempDir::new().unwrap();
    let waiver_file = temp_dir.path().join("waivers.toml");

    let toml_content = r#"
[[line_limit_waivers]]
path = "src/large_file.rs"
justification = "Legacy module scheduled for refactoring"

[[allow_attr_waivers]]
path = "src/ffi_bridge.rs"
justification = "FFI requires #[allow(improper_ctypes)]"
"#;

    fs::write(&waiver_file, toml_content).unwrap();

    let registry = WaiverRegistry::load_from_file(&waiver_file).unwrap();

    assert_eq!(registry.line_limit_waivers.len(), 1);
    assert_eq!(registry.line_limit_waivers[0].path, "src/large_file.rs");
    assert_eq!(
        registry.line_limit_waivers[0].justification,
        "Legacy module scheduled for refactoring"
    );

    assert_eq!(registry.allow_attr_waivers.len(), 1);
    assert_eq!(registry.allow_attr_waivers[0].path, "src/ffi_bridge.rs");
}

#[test]
fn test_load_from_file_missing_justification() {
    let temp_dir = TempDir::new().unwrap();
    let waiver_file = temp_dir.path().join("waivers.toml");

    let toml_content = r#"
[[line_limit_waivers]]
path = "src/large_file.rs"
justification = ""
"#;

    fs::write(&waiver_file, toml_content).unwrap();

    let result = WaiverRegistry::load_from_file(&waiver_file);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("missing justification"));
}

#[test]
fn test_load_from_file_invalid_toml() {
    let temp_dir = TempDir::new().unwrap();
    let waiver_file = temp_dir.path().join("waivers.toml");

    let toml_content = "this is not valid TOML [[[";
    fs::write(&waiver_file, toml_content).unwrap();

    let result = WaiverRegistry::load_from_file(&waiver_file);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse"));
}

#[test]
fn test_load_from_file_nonexistent() {
    let result = WaiverRegistry::load_from_file(Path::new("/nonexistent/waivers.toml"));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to read"));
}

#[test]
fn test_is_waived_line_limit() {
    let mut registry = WaiverRegistry::new();
    registry.line_limit_waivers.push(WaiverEntry {
        path: "src/large_file.rs".to_string(),
        justification: "Test justification".to_string(),
    });

    let rule = HygieneRule::LineLimit {
        limit: 200,
        actual: 250,
    };

    assert!(registry.is_waived(&rule, Path::new("src/large_file.rs")));
    assert!(!registry.is_waived(&rule, Path::new("src/other_file.rs")));
}

#[test]
fn test_is_waived_allow_attribute() {
    let mut registry = WaiverRegistry::new();
    registry.allow_attr_waivers.push(WaiverEntry {
        path: "src/ffi_bridge.rs".to_string(),
        justification: "FFI requires allow attribute".to_string(),
    });

    let rule = HygieneRule::AllowAttribute {
        attr: "improper_ctypes".to_string(),
    };

    assert!(registry.is_waived(&rule, Path::new("src/ffi_bridge.rs")));
    assert!(!registry.is_waived(&rule, Path::new("src/other_file.rs")));
}

#[test]
fn test_is_waived_unsupported_rule() {
    let mut registry = WaiverRegistry::new();
    registry.line_limit_waivers.push(WaiverEntry {
        path: "src/file.rs".to_string(),
        justification: "Test".to_string(),
    });

    // TodoComment rule doesn't support waivers
    let rule = HygieneRule::TodoComment;
    assert!(!registry.is_waived(&rule, Path::new("src/file.rs")));
}

#[test]
fn test_validate_waivers_stale_entries() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path();

    let mut registry = WaiverRegistry::new();
    registry.line_limit_waivers.push(WaiverEntry {
        path: "src/nonexistent.rs".to_string(),
        justification: "Test".to_string(),
    });
    registry.allow_attr_waivers.push(WaiverEntry {
        path: "src/also_nonexistent.rs".to_string(),
        justification: "Test".to_string(),
    });

    let errors = registry.validate_waivers(repo_root);

    assert_eq!(errors.len(), 2);
    assert!(errors[0].error.contains("does not exist"));
    assert!(errors[1].error.contains("does not exist"));
}

#[test]
fn test_validate_waivers_valid_entries() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path();

    // Create test files
    let src_dir = repo_root.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    fs::write(src_dir.join("file1.rs"), "// test").unwrap();
    fs::write(src_dir.join("file2.rs"), "// test").unwrap();

    let mut registry = WaiverRegistry::new();
    registry.line_limit_waivers.push(WaiverEntry {
        path: "src/file1.rs".to_string(),
        justification: "Test".to_string(),
    });
    registry.allow_attr_waivers.push(WaiverEntry {
        path: "src/file2.rs".to_string(),
        justification: "Test".to_string(),
    });

    let errors = registry.validate_waivers(repo_root);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_get_waivers_for_rule() {
    let mut registry = WaiverRegistry::new();
    registry.line_limit_waivers.push(WaiverEntry {
        path: "src/file1.rs".to_string(),
        justification: "Justification 1".to_string(),
    });
    registry.line_limit_waivers.push(WaiverEntry {
        path: "src/file2.rs".to_string(),
        justification: "Justification 2".to_string(),
    });

    let rule = HygieneRule::LineLimit {
        limit: 200,
        actual: 250,
    };
    let waivers = registry.get_waivers_for_rule(&rule);

    assert_eq!(waivers.len(), 2);
    assert_eq!(
        waivers.get(&PathBuf::from("src/file1.rs")).unwrap(),
        "Justification 1"
    );
    assert_eq!(
        waivers.get(&PathBuf::from("src/file2.rs")).unwrap(),
        "Justification 2"
    );
}
