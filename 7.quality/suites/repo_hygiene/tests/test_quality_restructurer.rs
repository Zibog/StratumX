// Unit tests for Quality Restructurer
//
// Tests the functionality of organizing migrated tests into appropriate suite categories.

use repo_hygiene::{QualityRestructurer, TestFileCandidate, TestType};
use std::fs;
use tempfile::TempDir;

fn create_test_repo() -> TempDir {
    TempDir::new().unwrap()
}

#[test]
fn test_suite_structure_creation() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    // Create the suite structure
    restructurer.create_suite_structure().unwrap();

    // Verify editor_canon_matrix structure
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/unit")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/integration")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/property")
        .exists());

    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/editor_shell/unit")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/editor_shell/integration")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/editor_shell/property")
        .exists());

    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/state_containers/unit")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/state_containers/integration")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/state_containers/property")
        .exists());

    // Verify editor_app_matrix structure
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_app_matrix/desktop_app/unit")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_app_matrix/desktop_app/integration")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_app_matrix/desktop_app/property")
        .exists());

    // Verify tooling_canon_matrix structure
    assert!(temp_dir
        .path()
        .join("7.quality/suites/tooling_canon_matrix/tool_session/unit")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/tooling_canon_matrix/tool_session/integration")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/tooling_canon_matrix/tool_session/property")
        .exists());
}

#[test]
fn test_suite_structure_creation_handles_existing_directories() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    // Create some directories manually
    fs::create_dir_all(
        temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/command_spine/unit"),
    )
    .unwrap();

    // Create the suite structure (should not fail on existing directories)
    restructurer.create_suite_structure().unwrap();

    // Verify all directories still exist
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/unit")
        .exists());
    assert!(temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/integration")
        .exists());
}

#[test]
fn test_categorize_unit_test_from_command_spine() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("5.editor/l7.0-editor-command-spine/src/command_tests.rs"),
        test_type: TestType::Unit,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/command_spine/unit/command_tests.rs")
    );
}

#[test]
fn test_categorize_property_test_from_editor_shell() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("5.editor/l8.0-editor-shell/src/shell_property_tests.rs"),
        test_type: TestType::Property,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir.path().join(
            "7.quality/suites/editor_canon_matrix/editor_shell/property/shell_property_tests.rs"
        )
    );
}

#[test]
fn test_categorize_integration_test_from_desktop_app() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("6.apps/editor/stratumx_editor_app/src/desktop_app/panel_tests.rs"),
        test_type: TestType::Integration,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir
            .path()
            .join("7.quality/suites/editor_app_matrix/desktop_app/integration/panel_tests.rs")
    );
}

#[test]
fn test_categorize_test_from_tool_session() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("4.tooling/l6.0-tool-session/src/session_tests.rs"),
        test_type: TestType::Unit,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir
            .path()
            .join("7.quality/suites/tooling_canon_matrix/tool_session/unit/session_tests.rs")
    );
}

#[test]
fn test_categorize_test_from_state_containers() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("5.editor/editor-state-containers/src/state_tests.rs"),
        test_type: TestType::Unit,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/state_containers/unit/state_tests.rs")
    );
}

#[test]
fn test_categorize_invariant_test_goes_to_property() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("5.editor/l7.0-editor-command-spine/src/invariant_tests.rs"),
        test_type: TestType::Invariant,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/command_spine/property/invariant_tests.rs")
    );
}

#[test]
fn test_categorize_smoke_test_goes_to_integration() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("5.editor/l7.0-editor-command-spine/src/smoke_tests.rs"),
        test_type: TestType::Smoke,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate).unwrap();
    assert_eq!(
        result,
        temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/command_spine/integration/smoke_tests.rs")
    );
}

#[test]
fn test_categorize_test_already_in_suite_returns_error() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir
            .path()
            .join("7.quality/suites/editor_canon_matrix/command_spine/unit/existing_test.rs"),
        test_type: TestType::Unit,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Test already in quality suite"));
}

#[test]
fn test_categorize_test_unknown_source_returns_error() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let candidate = TestFileCandidate {
        path: temp_dir.path().join("unknown/path/test.rs"),
        test_type: TestType::Unit,
        target_suite: String::new(),
        dependencies: vec![],
    };

    let result = restructurer.categorize_test(&candidate);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Unable to determine suite mapping"));
}

#[test]
fn test_get_existing_suite_tests_empty_directory() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    let existing = restructurer.get_existing_suite_tests().unwrap();
    assert_eq!(existing.len(), 0);
}

#[test]
fn test_get_existing_suite_tests_finds_rust_files() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    // Create some test files in the suite
    let test_file_path = temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/unit/test1.rs");
    fs::create_dir_all(test_file_path.parent().unwrap()).unwrap();
    fs::write(&test_file_path, "// test content").unwrap();

    let test_file_path2 = temp_dir
        .path()
        .join("7.quality/suites/editor_app_matrix/desktop_app/integration/test2.rs");
    fs::create_dir_all(test_file_path2.parent().unwrap()).unwrap();
    fs::write(&test_file_path2, "// test content").unwrap();

    let existing = restructurer.get_existing_suite_tests().unwrap();
    assert_eq!(existing.len(), 2);
    assert!(existing.contains(&test_file_path));
    assert!(existing.contains(&test_file_path2));
}

#[test]
fn test_get_existing_suite_tests_ignores_non_rust_files() {
    let temp_dir = create_test_repo();
    let restructurer = QualityRestructurer::new(temp_dir.path().to_path_buf());

    // Create a Rust file and a non-Rust file
    let test_file_path = temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/unit/test.rs");
    fs::create_dir_all(test_file_path.parent().unwrap()).unwrap();
    fs::write(&test_file_path, "// test content").unwrap();

    let non_rust_file = temp_dir
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/unit/README.md");
    fs::write(&non_rust_file, "# README").unwrap();

    let existing = restructurer.get_existing_suite_tests().unwrap();
    assert_eq!(existing.len(), 1);
    assert!(existing.contains(&test_file_path));
    assert!(!existing.contains(&non_rust_file));
}
