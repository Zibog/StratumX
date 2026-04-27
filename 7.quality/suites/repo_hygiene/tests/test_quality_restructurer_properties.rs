// Property-based tests for Quality Restructurer
//
// Tests universal properties that should hold for test suite categorization and organization.

use proptest::prelude::*;
use repo_hygiene::{QualityRestructurer, TestFileCandidate, TestType};
use std::path::PathBuf;
use tempfile::TempDir;

// Strategy for generating test types
fn arb_test_type() -> impl Strategy<Value = TestType> {
    prop_oneof![
        Just(TestType::Unit),
        Just(TestType::Integration),
        Just(TestType::Property),
        Just(TestType::Invariant),
        Just(TestType::Smoke),
    ]
}

// Strategy for generating valid source paths
fn arb_source_path() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("5.editor/l7.0-editor-command-spine/src/test.rs".to_string()),
        Just("5.editor/l8.0-editor-shell/src/test.rs".to_string()),
        Just("6.apps/editor/stratumx_editor_app/src/test.rs".to_string()),
        Just("4.tooling/l6.0-tool-session/src/test.rs".to_string()),
        Just("5.editor/editor-state-containers/src/test.rs".to_string()),
    ]
}

// Strategy for generating test file candidates
fn arb_test_candidate(repo_root: PathBuf) -> impl Strategy<Value = TestFileCandidate> {
    (arb_source_path(), arb_test_type()).prop_map(move |(path, test_type)| TestFileCandidate {
        path: repo_root.join(path),
        test_type,
        target_suite: String::new(),
        dependencies: vec![],
    })
}

// **Validates: Requirements 5.2, 5.5**
// Feature: repo-sanitization-phase-1, Property 11: Test Suite Categorization
//
// For any migrated test file, the Quality_Restructurer should place it in the appropriate
// suite category based on its test type (unit, integration, property) and source subsystem.
#[test]
fn prop_test_suite_categorization() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();
    let restructurer = QualityRestructurer::new(repo_root.clone());

    proptest!(|(candidate in arb_test_candidate(repo_root.clone()))| {
        let result = restructurer.categorize_test(&candidate);

        // Should successfully categorize all valid source paths
        prop_assert!(result.is_ok());

        let target_path = result.unwrap();
        let target_str = target_path.to_string_lossy();

        // Verify the target path is in the quality suites directory
        prop_assert!(target_str.contains("7.quality/suites"));

        // Verify the test type subdirectory is correct
        match candidate.test_type {
            TestType::Unit => {
                prop_assert!(target_str.contains("\\unit\\") || target_str.contains("/unit/"));
            }
            TestType::Integration => {
                prop_assert!(target_str.contains("\\integration\\") || target_str.contains("/integration/"));
            }
            TestType::Property => {
                prop_assert!(target_str.contains("\\property\\") || target_str.contains("/property/"));
            }
            TestType::Invariant => {
                // Invariants go to property directory
                prop_assert!(target_str.contains("\\property\\") || target_str.contains("/property/"));
            }
            TestType::Smoke => {
                // Smoke tests go to integration directory
                prop_assert!(target_str.contains("\\integration\\") || target_str.contains("/integration/"));
            }
        }

        // Verify the suite category is correct based on source path
        let source_str = candidate.path.to_string_lossy();
        if source_str.contains("5.editor/l7.0-editor-command-spine") || source_str.contains("5.editor\\l7.0-editor-command-spine") {
            prop_assert!(target_str.contains("editor_canon_matrix") && (target_str.contains("command_spine")));
        } else if source_str.contains("5.editor/l8.0-editor-shell") || source_str.contains("5.editor\\l8.0-editor-shell") {
            prop_assert!(target_str.contains("editor_canon_matrix") && target_str.contains("editor_shell"));
        } else if source_str.contains("6.apps/editor/stratumx_editor_app") || source_str.contains("6.apps\\editor\\stratumx_editor_app") {
            prop_assert!(target_str.contains("editor_app_matrix") && target_str.contains("desktop_app"));
        } else if source_str.contains("4.tooling/l6.0-tool-session") || source_str.contains("4.tooling\\l6.0-tool-session") {
            prop_assert!(target_str.contains("tooling_canon_matrix") && target_str.contains("tool_session"));
        } else if source_str.contains("5.editor/editor-state-containers") || source_str.contains("5.editor\\editor-state-containers") {
            prop_assert!(target_str.contains("editor_canon_matrix") && target_str.contains("state_containers"));
        }

        // Verify the filename is preserved
        prop_assert_eq!(
            target_path.file_name(),
            candidate.path.file_name()
        );
    });
}

// **Validates: Requirements 5.6**
// Feature: repo-sanitization-phase-1, Property 12: Existing Suite Preservation
//
// For any test file already located in 7.quality/suites/, the migration process
// should not modify or move it.
#[test]
fn prop_existing_suite_preservation() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();
    let restructurer = QualityRestructurer::new(repo_root.clone());

    // Strategy for generating paths already in quality suites
    let arb_existing_suite_path = prop_oneof![
        Just("7.quality/suites/editor_canon_matrix/command_spine/unit/test.rs".to_string()),
        Just("7.quality/suites/editor_canon_matrix/editor_shell/integration/test.rs".to_string()),
        Just("7.quality/suites/editor_app_matrix/desktop_app/property/test.rs".to_string()),
        Just("7.quality/suites/tooling_canon_matrix/tool_session/unit/test.rs".to_string()),
        Just(
            "7.quality/suites/editor_canon_matrix/state_containers/integration/test.rs".to_string()
        ),
    ];

    proptest!(|(path_str in arb_existing_suite_path, test_type in arb_test_type())| {
        let candidate = TestFileCandidate {
            path: repo_root.join(path_str),
            test_type,
            target_suite: String::new(),
            dependencies: vec![],
        };

        // Should detect that the test is already in a suite
        prop_assert!(restructurer.is_in_existing_suite(&candidate.path));

        // Should return an error when trying to categorize
        let result = restructurer.categorize_test(&candidate);
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().contains("Test already in quality suite"));
    });
}
