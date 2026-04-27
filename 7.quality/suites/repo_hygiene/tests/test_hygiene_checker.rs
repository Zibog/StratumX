// Unit tests for HygieneChecker
//
// This module tests the hygiene checker functionality including:
// - Line limit checks
// - Test file in src detection
// - TODO comment detection
// - Allow attribute detection
// - Prohibition pattern detection
// - Unified check runner

use repo_hygiene::{HygieneChecker, HygieneRule, WaiverRegistry};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a temporary test repository with a basic structure
fn create_test_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path();

    // Create basic directory structure
    fs::create_dir_all(repo_root.join("src")).unwrap();
    fs::create_dir_all(repo_root.join("tests")).unwrap();

    temp_dir
}

/// Create a test file with specified content
fn create_test_file(repo_root: &std::path::Path, relative_path: &str, content: &str) {
    let file_path = repo_root.join(relative_path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(file_path, content).unwrap();
}

// ============================================================================
// Line Limit Tests (Requirement 6.1)
// ============================================================================

#[test]
fn test_line_limit_check_passes_for_file_under_limit() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Create a file with 100 lines (under limit)
    let content = (0..100)
        .map(|i| format!("// Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    create_test_file(repo_root, "src/small_file.rs", &content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_line_limits();

    assert_eq!(
        violations.len(),
        0,
        "Should not report violations for files under 200 lines"
    );
}

#[test]
fn test_line_limit_check_fails_for_file_over_limit() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Create a file with 250 lines (over limit)
    let content = (0..250)
        .map(|i| format!("// Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    create_test_file(repo_root, "src/large_file.rs", &content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_line_limits();

    assert_eq!(
        violations.len(),
        1,
        "Should report one violation for file over 200 lines"
    );

    let violation = &violations[0];
    assert_eq!(violation.path, PathBuf::from("src/large_file.rs"));

    match &violation.rule {
        HygieneRule::LineLimit { limit, actual } => {
            assert_eq!(*limit, 200);
            assert_eq!(*actual, 250);
        }
        _ => panic!("Expected LineLimit rule"),
    }

    assert!(!violation.waived);
    assert!(violation.remediation.contains("200 lines"));
}

#[test]
fn test_line_limit_check_respects_waivers() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Create a file with 250 lines (over limit)
    let content = (0..250)
        .map(|i| format!("// Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    create_test_file(repo_root, "src/waived_file.rs", &content);

    // Create waiver registry with this file waived
    let waiver_toml = r#"
[[line_limit_waivers]]
path = "src/waived_file.rs"
justification = "Legacy file scheduled for refactoring"
"#;

    let waiver_path = repo_root.join("waivers.toml");
    fs::write(&waiver_path, waiver_toml).unwrap();

    let waiver_registry = WaiverRegistry::load_from_file(&waiver_path).unwrap();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_line_limits();

    assert_eq!(
        violations.len(),
        0,
        "Should not report violations for waived files"
    );
}

// ============================================================================
// Test File in Src Tests (Requirement 6.2)
// ============================================================================

#[test]
fn test_detects_test_file_with_test_rs_suffix() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    create_test_file(repo_root, "src/module.test.rs", "// test file");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_test_files_in_src();

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].path, PathBuf::from("src/module.test.rs"));
    assert!(matches!(violations[0].rule, HygieneRule::TestFileInSrc));
}

#[test]
fn test_detects_test_file_with_tests_rs_suffix() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    create_test_file(repo_root, "src/module_tests.rs", "// test file");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_test_files_in_src();

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].path, PathBuf::from("src/module_tests.rs"));
}

#[test]
fn test_does_not_detect_regular_files_as_tests() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    create_test_file(repo_root, "src/module.rs", "// regular file");
    create_test_file(repo_root, "src/lib.rs", "// library file");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_test_files_in_src();

    assert_eq!(
        violations.len(),
        0,
        "Should not detect regular files as test files"
    );
}

#[test]
fn test_does_not_flag_tests_in_tests_directory() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Test files in tests/ directory are OK
    create_test_file(repo_root, "tests/integration_tests.rs", "// test file");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_test_files_in_src();

    assert_eq!(
        violations.len(),
        0,
        "Should not flag test files in tests/ directory"
    );
}

// ============================================================================
// TODO Comment Tests (Requirement 6.3)
// ============================================================================

#[test]
fn test_detects_todo_comments() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
fn example() {
    // TODO: implement this
    unimplemented!()
}
"#;
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_todo_comments();

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].path, PathBuf::from("src/module.rs"));
    assert!(matches!(violations[0].rule, HygieneRule::TodoComment));
    assert_eq!(violations[0].line_number, Some(3));
    assert!(violations[0].context.contains("TODO"));
}

#[test]
fn test_detects_todo_case_insensitive() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
// todo: fix this
// Todo: also this
// TODO: and this
"#;
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_todo_comments();

    assert_eq!(violations.len(), 3, "Should detect TODO in any case");
}

#[test]
fn test_extracts_todo_context() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = "// TODO: implement error handling";
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_todo_comments();

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].context, "// TODO: implement error handling");
}

// ============================================================================
// Allow Attribute Tests (Requirement 6.4)
// ============================================================================

#[test]
fn test_detects_allow_attributes() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
#[allow(dead_code)]
fn unused_function() {}
"#;
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_allow_attributes();

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].path, PathBuf::from("src/module.rs"));

    match &violations[0].rule {
        HygieneRule::AllowAttribute { attr } => {
            assert!(attr.contains("allow(dead_code)"));
        }
        _ => panic!("Expected AllowAttribute rule"),
    }
}

#[test]
fn test_allow_attribute_check_respects_waivers() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
#[allow(improper_ctypes)]
extern "C" fn ffi_function() {}
"#;
    create_test_file(repo_root, "src/ffi.rs", content);

    // Create waiver registry
    let waiver_toml = r#"
[[allow_attr_waivers]]
path = "src/ffi.rs"
justification = "FFI requires improper_ctypes allow"
"#;

    let waiver_path = repo_root.join("waivers.toml");
    fs::write(&waiver_path, waiver_toml).unwrap();

    let waiver_registry = WaiverRegistry::load_from_file(&waiver_path).unwrap();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_allow_attributes();

    assert_eq!(
        violations.len(),
        0,
        "Should not report violations for waived files"
    );
}

// ============================================================================
// Prohibition Tests (Requirements 8.1-8.6)
// ============================================================================

#[test]
fn test_detects_static_mut() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = "static mut GLOBAL_STATE: i32 = 0;";
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_prohibitions();

    let static_mut_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.rule, HygieneRule::StaticMut))
        .collect();

    assert_eq!(static_mut_violations.len(), 1);
    assert!(static_mut_violations[0].remediation.contains("Mutex"));
}

#[test]
fn test_detects_execute_bridge_functions() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
pub fn execute_command() {}
fn execute_action() {}
"#;
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_prohibitions();

    let execute_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.rule, HygieneRule::ExecuteBridge))
        .collect();

    assert_eq!(execute_violations.len(), 2);
}

#[test]
fn test_detects_host_bypasses() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
impl Panel {
    fn save(&self) {
        self.host.save_file(path, content);
    }
}
"#;
    create_test_file(repo_root, "src/panel.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_prohibitions();

    let host_bypass_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.rule, HygieneRule::HostBypass))
        .collect();

    assert_eq!(host_bypass_violations.len(), 1);
    assert!(host_bypass_violations[0]
        .remediation
        .contains("Command_Spine"));
}

#[test]
fn test_detects_hardcoded_paths() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = r#"
let path1 = "/home/user/file.txt";
let path2 = "C:\\Users\\user\\file.txt";
let path3 = "/Users/user/file.txt";
"#;
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_prohibitions();

    let path_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.rule, HygieneRule::HardcodedPath))
        .collect();

    assert_eq!(path_violations.len(), 3);
}

#[test]
fn test_detects_fake_truth_patterns() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = "let default_fake_value = 42;";
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_prohibitions();

    let fake_truth_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.rule, HygieneRule::FakeTruth))
        .collect();

    assert_eq!(fake_truth_violations.len(), 1);
}

#[test]
fn test_prohibition_violations_include_remediation() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    let content = "static mut GLOBAL: i32 = 0;";
    create_test_file(repo_root, "src/module.rs", content);

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let violations = checker.check_prohibitions();

    assert!(!violations.is_empty());
    assert!(
        !violations[0].remediation.is_empty(),
        "Remediation should not be empty"
    );
}

// ============================================================================
// Unified Check Runner Tests (Requirements 6.10-6.12, 7.5-7.6)
// ============================================================================

#[test]
fn test_run_all_checks_aggregates_violations() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Create files with multiple types of violations
    let large_file = (0..250)
        .map(|i| format!("// Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    create_test_file(repo_root, "src/large.rs", &large_file);

    create_test_file(repo_root, "src/module.test.rs", "// test file");

    create_test_file(repo_root, "src/todo.rs", "// TODO: fix this");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let report = checker.run_all_checks();

    assert!(
        report.violations.len() >= 3,
        "Should aggregate violations from all checks"
    );
    assert!(report.failed_checks > 0);
}

#[test]
fn test_run_all_checks_tracks_passed_and_failed() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Create a clean file
    create_test_file(repo_root, "src/clean.rs", "fn example() {}");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let report = checker.run_all_checks();

    let total_checks = report.passed_checks + report.failed_checks;
    assert_eq!(total_checks, 5, "Should run 5 check types");
}

#[test]
fn test_run_all_checks_tracks_execution_time() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    create_test_file(repo_root, "src/module.rs", "fn example() {}");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let report = checker.run_all_checks();

    // Execution time should be tracked
    assert!(
        report.execution_time.as_secs() < 60,
        "Should complete within 60 seconds"
    );
}

#[test]
fn test_run_all_checks_with_no_violations() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    // Create a clean file
    create_test_file(repo_root, "src/clean.rs", "fn example() {}");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let report = checker.run_all_checks();

    assert_eq!(report.violations.len(), 0);
    assert_eq!(report.failed_checks, 0);
    assert_eq!(report.passed_checks, 5);
}

#[test]
fn test_violation_report_includes_all_required_fields() {
    let temp_dir = create_test_repo();
    let repo_root = temp_dir.path();

    create_test_file(repo_root, "src/module.rs", "// TODO: implement");

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

    let report = checker.run_all_checks();

    assert!(!report.violations.is_empty());

    let violation = &report.violations[0];
    assert!(
        !violation.path.as_os_str().is_empty(),
        "Path should not be empty"
    );
    assert!(!violation.context.is_empty(), "Context should not be empty");
    assert!(
        !violation.remediation.is_empty(),
        "Remediation should not be empty"
    );
    assert!(
        violation.line_number.is_some(),
        "Line number should be present for TODO"
    );
}
