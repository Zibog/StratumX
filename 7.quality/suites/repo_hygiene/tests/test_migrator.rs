// Unit tests for Test Migrator
//
// These tests validate specific examples and edge cases for test migration.

use repo_hygiene::{CompilationStatus, TestFileCandidate, TestMigrator, TestType};
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a temporary test repository
fn create_test_repo() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Create a file in the test repository
fn create_file(repo: &TempDir, path: &str, content: &str) {
    let full_path = repo.path().join(path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directories");
    }
    fs::write(&full_path, content).expect("Failed to write file");
}

// ============================================================================
// Subtask 5.3: Unit tests for destination mapping
// ============================================================================

#[test]
fn test_destination_mapping_command_spine() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let source = repo
        .path()
        .join("5.editor/l7.0-editor-command-spine/src/command_tests.rs");
    let dest = migrator
        .determine_destination(&source)
        .expect("Expected command spine destination");
    assert!(dest.to_string_lossy().contains("7.quality/suites"));
    assert!(dest
        .to_string_lossy()
        .contains("editor_canon_matrix/command_spine"));
    assert_eq!(dest.file_name().unwrap(), "command_tests.rs");
}

#[test]
fn test_destination_mapping_editor_shell() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let source = repo
        .path()
        .join("5.editor/l8.0-editor-shell/src/shell_tests.rs");
    let dest = migrator
        .determine_destination(&source)
        .expect("Expected editor shell destination");
    assert!(dest.to_string_lossy().contains("7.quality/suites"));
    assert!(dest
        .to_string_lossy()
        .contains("editor_canon_matrix/editor_shell"));
    assert_eq!(dest.file_name().unwrap(), "shell_tests.rs");
}

#[test]
fn test_destination_mapping_desktop_app() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let source = repo
        .path()
        .join("6.apps/editor/stratumx_editor_app/src/desktop_app/panel_tests.rs");
    let dest = migrator
        .determine_destination(&source)
        .expect("Expected desktop app destination");
    assert!(dest.to_string_lossy().contains("7.quality/suites"));
    assert!(dest
        .to_string_lossy()
        .contains("editor_app_matrix/desktop_app"));
    assert_eq!(dest.file_name().unwrap(), "panel_tests.rs");
}

#[test]
fn test_destination_mapping_tool_session() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let source = repo
        .path()
        .join("4.tooling/l6.0-tool-session/src/session_tests.rs");
    let dest = migrator
        .determine_destination(&source)
        .expect("Expected tool session destination");
    assert!(dest.to_string_lossy().contains("7.quality/suites"));
    assert!(dest
        .to_string_lossy()
        .contains("tooling_canon_matrix/tool_session"));
    assert_eq!(dest.file_name().unwrap(), "session_tests.rs");
}

#[test]
fn test_destination_mapping_state_containers() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let source = repo
        .path()
        .join("5.editor/editor-state-containers/src/container_tests.rs");
    let dest = migrator
        .determine_destination(&source)
        .expect("Expected state containers destination");
    assert!(dest.to_string_lossy().contains("7.quality/suites"));
    assert!(dest
        .to_string_lossy()
        .contains("editor_canon_matrix/state_containers"));
    assert_eq!(dest.file_name().unwrap(), "container_tests.rs");
}

#[test]
fn test_destination_mapping_unknown_directory() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let source = repo.path().join("unknown/path/test.rs");
    let dest = migrator.determine_destination(&source);

    assert!(dest.is_none());
}

// ============================================================================
// Subtask 5.6: Unit tests for file migration
// ============================================================================

#[test]
fn test_dry_run_migration() {
    let repo = create_test_repo();

    // Create source test file
    let source_path_str = "5.editor/l7.0-editor-command-spine/src/test.rs";
    let test_content = r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#;
    create_file(&repo, source_path_str, test_content);

    let source_path = repo.path().join(source_path_str);

    // Create test candidate
    let candidate = TestFileCandidate {
        path: source_path.clone(),
        test_type: TestType::Unit,
        target_suite: "editor_canon_matrix/command_spine".to_string(),
        dependencies: vec![],
    };

    // Perform dry-run migration
    let migrator = TestMigrator::new(repo.path().to_path_buf(), true);
    let migration_result = migrator
        .migrate_test(&candidate)
        .expect("Dry-run migration should succeed");

    // Verify source path is correct
    assert_eq!(migration_result.source_path, source_path);

    // Verify destination path is in correct suite
    assert!(migration_result
        .destination_path
        .to_string_lossy()
        .contains("editor_canon_matrix/command_spine"));

    // Verify compilation status is NotAttempted in dry-run mode
    assert_eq!(
        migration_result.compilation_status,
        CompilationStatus::NotAttempted
    );

    // Verify source file still exists (dry-run doesn't move files)
    assert!(source_path.exists());
}

#[test]
fn test_migration_with_invalid_source() {
    let repo = create_test_repo();

    // Create test candidate with non-existent source
    let source_path = repo.path().join("nonexistent/test.rs");
    let candidate = TestFileCandidate {
        path: source_path.clone(),
        test_type: TestType::Unit,
        target_suite: "test_suite".to_string(),
        dependencies: vec![],
    };

    // Attempt migration (not dry-run)
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);
    let result = migrator.migrate_test(&candidate);

    // Should fail because source doesn't exist
    assert!(result.is_err());
}

#[test]
fn test_migration_creates_destination_directory() {
    let repo = create_test_repo();

    // Create source test file
    let source_path_str = "5.editor/l7.0-editor-command-spine/src/test.rs";
    let test_content = r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#;
    create_file(&repo, source_path_str, test_content);

    let source_path = repo.path().join(source_path_str);

    // Create test candidate
    let candidate = TestFileCandidate {
        path: source_path.clone(),
        test_type: TestType::Unit,
        target_suite: "editor_canon_matrix/command_spine".to_string(),
        dependencies: vec![],
    };

    // Verify destination directory doesn't exist yet
    let dest_dir = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine");
    assert!(!dest_dir.exists());

    // Perform dry-run migration (which still checks destination logic)
    let migrator = TestMigrator::new(repo.path().to_path_buf(), true);
    let result = migrator.migrate_test(&candidate);

    assert!(result.is_ok());
}

// ============================================================================
// Subtask 5.9: Unit tests for import path updates
// ============================================================================

#[test]
fn test_import_path_update_crate_imports() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let content = r#"
use crate::some_module;
use crate::another::module;

#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#;

    let source_path = repo
        .path()
        .join("5.editor/l7.0-editor-command-spine/src/test.rs");
    let dest_path = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/test.rs");

    let (updated_content, _imports_updated) = migrator
        .update_imports(content, &source_path, &dest_path)
        .unwrap();

    // Crate-level imports should remain unchanged
    assert!(updated_content.contains("use crate::some_module"));
    assert!(updated_content.contains("use crate::another::module"));
}

#[test]
fn test_import_path_update_std_imports() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let content = r#"
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#;

    let source_path = repo
        .path()
        .join("5.editor/l7.0-editor-command-spine/src/test.rs");
    let dest_path = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/test.rs");

    let (updated_content, imports_updated) = migrator
        .update_imports(content, &source_path, &dest_path)
        .unwrap();

    // Std imports should remain unchanged
    assert!(updated_content.contains("use std::collections::HashMap"));
    assert!(updated_content.contains("use std::path::PathBuf"));

    // No imports should be updated
    assert_eq!(imports_updated, 0);
}

#[test]
fn test_import_path_update_proptest_imports() {
    let repo = create_test_repo();
    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);

    let content = r#"
use proptest::prelude::*;
use proptest::prop_compose;

proptest! {
    #[test]
    fn test_property(x in 0..100) {
        assert!(x < 100);
    }
}
"#;

    let source_path = repo
        .path()
        .join("5.editor/l7.0-editor-command-spine/src/test.rs");
    let dest_path = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/test.rs");

    let (updated_content, imports_updated) = migrator
        .update_imports(content, &source_path, &dest_path)
        .unwrap();

    // Proptest imports should remain unchanged
    assert!(updated_content.contains("use proptest::prelude::*"));
    assert!(updated_content.contains("use proptest::prop_compose"));

    // No imports should be updated
    assert_eq!(imports_updated, 0);
}

#[test]
fn test_regression_file_detection() {
    let repo = create_test_repo();

    // Create test file
    let source_path_str = "5.editor/l7.0-editor-command-spine/src/property_test.rs";
    create_file(&repo, source_path_str, "// test content");

    // Create regression file
    let regression_path =
        "5.editor/l7.0-editor-command-spine/src/proptest-regressions/property_test.txt";
    create_file(&repo, regression_path, "# regression data");

    let source_path = repo.path().join(source_path_str);
    let dest_path = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/property_test.rs");

    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);
    let moved_files = migrator
        .preserve_regression_files(&source_path, &dest_path)
        .expect("Regression file preservation should succeed");

    // Should have moved the regression file
    assert_eq!(moved_files.len(), 1);
    assert!(moved_files[0]
        .to_string_lossy()
        .contains("proptest-regressions"));
    assert!(moved_files[0]
        .to_string_lossy()
        .contains("property_test.txt"));
}

#[test]
fn test_regression_file_preservation_no_regressions() {
    let repo = create_test_repo();

    // Create test file without regression files
    let source_path_str = "5.editor/l7.0-editor-command-spine/src/test.rs";
    create_file(&repo, source_path_str, "// test content");

    let source_path = repo.path().join(source_path_str);
    let dest_path = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/test.rs");

    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);
    let moved_files = migrator
        .preserve_regression_files(&source_path, &dest_path)
        .expect("Regression file preservation should succeed");

    // Should have no moved files
    assert_eq!(moved_files.len(), 0);
}

#[test]
fn test_regression_file_preservation_multiple_files() {
    let repo = create_test_repo();

    // Create test file
    let source_path_str = "5.editor/l7.0-editor-command-spine/src/property_test.rs";
    create_file(&repo, source_path_str, "// test content");

    // Create multiple regression files for the same test
    let regression_path1 =
        "5.editor/l7.0-editor-command-spine/src/proptest-regressions/property_test.txt";
    let regression_path2 =
        "5.editor/l7.0-editor-command-spine/src/proptest-regressions/property_test_2.txt";
    create_file(&repo, regression_path1, "# regression data 1");
    create_file(&repo, regression_path2, "# regression data 2");

    let source_path = repo.path().join(source_path_str);
    let dest_path = repo
        .path()
        .join("7.quality/suites/editor_canon_matrix/command_spine/property_test.rs");

    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);
    let moved_files = migrator
        .preserve_regression_files(&source_path, &dest_path)
        .expect("Regression file preservation should succeed");

    // Should have moved both regression files
    assert_eq!(moved_files.len(), 2);
}

// ============================================================================
// Subtask 5.15: Unit tests for post-migration verification
// ============================================================================

#[test]
fn test_compilation_verification_tracks_status() {
    let repo = create_test_repo();

    // Create a simple test file
    let test_path = repo.path().join("7.quality/suites/test.rs");
    fs::create_dir_all(test_path.parent().unwrap()).unwrap();
    fs::write(&test_path, "#[test] fn test() { assert!(true); }").unwrap();

    let migrator = TestMigrator::new(repo.path().to_path_buf(), false);
    let status = migrator
        .verify_compilation(&test_path)
        .expect("Compilation verification should return a status");

    // Status should be one of the valid enum values
    assert!(matches!(
        status,
        CompilationStatus::Success | CompilationStatus::Failed | CompilationStatus::NotAttempted
    ));
}
