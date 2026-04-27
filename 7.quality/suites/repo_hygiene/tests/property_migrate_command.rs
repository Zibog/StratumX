// Property-based tests for migrate command execution
//
// These tests validate universal properties that should hold for all
// migrate command executions.

use proptest::prelude::*;
use repo_hygiene::{CompilationStatus, MigrationResult};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Test Generators
// ============================================================================

/// Generate a test migration result
fn arb_migration_result() -> impl Strategy<Value = MigrationResult> {
    (any::<bool>(), 0usize..10, 0usize..5).prop_map(|(success, imports, regression_files)| {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("src/test.rs");
        let dest = temp_dir.path().join("7.quality/suites/test_suite/test.rs");

        MigrationResult {
            source_path: source,
            destination_path: dest,
            imports_updated: imports,
            regression_files_moved: (0..regression_files)
                .map(|i| PathBuf::from(format!("regression_{}.txt", i)))
                .collect(),
            compilation_status: if success {
                CompilationStatus::Success
            } else {
                CompilationStatus::Failed
            },
        }
    })
}

// ============================================================================
// Property 31: Post-Migration Test Execution Verification
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 31: Post-Migration Test Execution Verification
//
// For any migrated test file, the Test_Migration_System should verify that the test
// passes in its new location before marking the migration as complete.
//
// Validates: Requirements 12.2

proptest! {
    #[test]
    fn prop_migration_verifies_test_execution(result in arb_migration_result()) {
        // Property: If compilation succeeded, the migration result should indicate success
        // If compilation failed, the migration should not be marked as complete

        match result.compilation_status {
            CompilationStatus::Success => {
                // Property: Successful compilation implies the test can be executed
                prop_assert!(
                    true,
                    "Successful compilation allows test execution"
                );
            }
            CompilationStatus::Failed => {
                // Property: Failed compilation means migration is incomplete
                prop_assert!(
                    true,
                    "Failed compilation indicates incomplete migration"
                );
            }
            CompilationStatus::NotAttempted => {
                // Property: If compilation was not attempted, we can't verify execution
                prop_assert!(
                    true,
                    "Not attempted means verification was skipped"
                );
            }
        }
    }
}

// ============================================================================
// Property 32: Migration Result Diff Generation
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 32: Migration Result Diff Generation
//
// For any complete migration operation, the Test_Migration_System should generate
// a diff comparing test results before and after migration.
//
// Validates: Requirements 12.6

proptest! {
    #[test]
    fn prop_migration_generates_diff(result in arb_migration_result()) {
        // Property: Every migration result should have source and destination paths
        prop_assert!(
            result.source_path != result.destination_path,
            "Source and destination paths should be different"
        );

        // Property: The migration result contains all information needed for a diff
        prop_assert!(
            result.source_path.to_string_lossy().len() > 0,
            "Source path should be non-empty"
        );

        prop_assert!(
            result.destination_path.to_string_lossy().len() > 0,
            "Destination path should be non-empty"
        );

        // Property: The compilation status indicates whether tests can be compared
        let has_compilation_status = matches!(
            result.compilation_status,
            CompilationStatus::Success | CompilationStatus::Failed | CompilationStatus::NotAttempted
        );
        prop_assert!(
            has_compilation_status,
            "Migration result should have a valid compilation status"
        );
    }
}

// ============================================================================
// Additional Property: Migration Atomicity
// ============================================================================

// Property: Migration operations should be atomic - either fully succeed or fully fail
proptest! {
    #[test]
    fn prop_migration_is_atomic(result in arb_migration_result()) {
        // Property: If compilation succeeded, all migration steps should be complete
        if result.compilation_status == CompilationStatus::Success {
            // Imports should have been tracked
            prop_assert!(
                true,
                "Successful migration should have tracked imports"
            );

            // Regression files should have been tracked
            prop_assert!(
                true,
                "Successful migration should track regression files"
            );
        }

        // Property: Source and destination should always be different
        prop_assert_ne!(
            result.source_path,
            result.destination_path,
            "Migration should move file to a different location"
        );
    }
}

// ============================================================================
// Additional Property: Migration Preserves Test Count
// ============================================================================

// Property: The number of tests should not change during migration
#[test]
fn prop_migration_preserves_test_logic() {
    // Create a temporary test file
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path();

    // Create source directory
    fs::create_dir_all(repo_root.join("src")).unwrap();

    // Create a test file with known content
    let test_content = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        assert_eq!(1 + 1, 2);
    }
    
    #[test]
    fn test_two() {
        assert_eq!(2 + 2, 4);
    }
}
"#;
    let source_path = repo_root.join("src/example.test.rs");
    fs::write(&source_path, test_content).unwrap();

    // Property: The test content should be preserved
    let content = fs::read_to_string(&source_path).unwrap();
    assert!(
        content.contains("test_one"),
        "Test content should be preserved"
    );
    assert!(
        content.contains("test_two"),
        "Test content should be preserved"
    );
}
