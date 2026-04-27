// Unit tests for Discovery Scanner
//
// Tests the discovery scanner's ability to:
// - Detect test files by pattern and content
// - Classify test types correctly
// - Identify hygiene violations
// - Detect anti-patterns

use repo_hygiene::{DiscoveryScanner, TestType, WaiverRegistry};
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a temporary test repository with a given structure
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
// Test File Detection Tests
// ============================================================================

#[test]
fn test_detect_test_file_by_name_pattern_test_rs() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.test.rs",
        r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert!(test_files[0].path.ends_with("module.test.rs"));
}

#[test]
fn test_detect_test_file_by_name_pattern_tests_rs() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module_tests.rs",
        r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert!(test_files[0].path.ends_with("module_tests.rs"));
}

#[test]
fn test_detect_proptest_by_content() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/properties.rs",
        r#"
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(x in 0..100) {
        assert!(x < 100);
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert!(test_files[0].path.ends_with("properties.rs"));
    assert_eq!(test_files[0].test_type, TestType::Property);
}

#[test]
fn test_detect_proptest_by_prop_compose() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/generators.rs",
        r#"
use proptest::prelude::*;

prop_compose! {
    fn arb_user()(name in "\\w+", age in 0..100u8) -> User {
        User { name, age }
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(test_files[0].test_type, TestType::Property);
}

#[test]
fn test_detect_invariant_test() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/invariants.rs",
        r#"
invariant! {
    fn check_balance(state: &State) {
        assert!(state.balance >= 0);
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(test_files[0].test_type, TestType::Invariant);
}

#[test]
fn test_detect_smoke_test_by_name() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/smoke_test.rs",
        r#"
#[test]
fn smoke_test_basic() {
    assert_eq!(1 + 1, 2);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(test_files[0].test_type, TestType::Smoke);
}

#[test]
fn test_exclude_non_test_files() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.rs",
        r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 0);
}

#[test]
fn test_exclude_tests_in_quality_directory() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "7.quality/suites/test_suite/module.test.rs",
        r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    // Should not find test files in 7.quality/
    assert_eq!(test_files.len(), 0);
}

// ============================================================================
// Test Type Classification Tests
// ============================================================================

#[test]
fn test_classify_unit_test() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.test.rs",
        r#"
#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(test_files[0].test_type, TestType::Unit);
}

#[test]
fn test_classify_integration_test() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "tests/integration_test.rs",
        r#"
#[test]
fn test_integration() {
    assert_eq!(2 + 2, 4);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    // Note: This test file is in tests/ directory, not src/, so it won't be detected
    // as a migration candidate (it's already in the right place)
    assert_eq!(test_files.len(), 0);
}

// ============================================================================
// Target Suite Determination Tests
// ============================================================================

#[test]
fn test_determine_target_suite_command_spine() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "5.editor/l7.0-editor-command-spine/src/command.test.rs",
        r#"
#[test]
fn test_command() {
    assert_eq!(3 + 3, 6);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(
        test_files[0].target_suite,
        "editor_canon_matrix/command_spine"
    );
}

#[test]
fn test_determine_target_suite_editor_shell() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "5.editor/l8.0-editor-shell/src/shell.test.rs",
        r#"
#[test]
fn test_shell() {
    assert_eq!(4 + 4, 8);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(
        test_files[0].target_suite,
        "editor_canon_matrix/editor_shell"
    );
}

#[test]
fn test_determine_target_suite_desktop_app() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.test.rs",
        r#"
#[test]
fn test_panel() {
    assert_eq!(5 + 5, 10);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(test_files[0].target_suite, "editor_app_matrix/desktop_app");
}

#[test]
fn test_determine_target_suite_tool_session() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "4.tooling/l6.0-tool-session/src/session.test.rs",
        r#"
#[test]
fn test_session() {
    assert_eq!(6 + 6, 12);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(
        test_files[0].target_suite,
        "tooling_canon_matrix/tool_session"
    );
}

#[test]
fn test_determine_target_suite_state_containers() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "5.editor/editor-state-containers/src/container.test.rs",
        r#"
#[test]
fn test_container() {
    assert_eq!(7 + 7, 14);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let test_files = scanner.scan_test_files();

    assert_eq!(test_files.len(), 1);
    assert_eq!(
        test_files[0].target_suite,
        "editor_canon_matrix/state_containers"
    );
}

// ============================================================================
// Hygiene Violation Detection Tests
// ============================================================================

#[test]
fn test_detect_line_count_violation() {
    let repo = create_test_repo();

    // Create a file with more than 200 lines
    let mut content = String::new();
    for i in 0..250 {
        content.push_str(&format!("// Line {}\n", i));
    }

    create_file(&repo, "src/large_file.rs", &content);

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_hygiene_violations();

    assert_eq!(violations.len(), 1);
    assert!(matches!(
        violations[0].rule,
        repo_hygiene::HygieneRule::LineLimit {
            limit: 200,
            actual: 250
        }
    ));
}

#[test]
fn test_line_count_violation_respects_waiver() {
    let repo = create_test_repo();

    // Create a file with more than 200 lines
    let mut content = String::new();
    for i in 0..250 {
        content.push_str(&format!("// Line {}\n", i));
    }

    create_file(&repo, "src/large_file.rs", &content);

    // Create a waiver registry with this file waived
    let waiver_toml = r#"
[[line_limit_waivers]]
path = "src/large_file.rs"
justification = "Test waiver"
"#;

    let waiver_path = repo.path().join("waivers.toml");
    fs::write(&waiver_path, waiver_toml).expect("Failed to write waiver file");

    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Failed to load waiver registry");

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), waiver_registry);
    let violations = scanner.scan_hygiene_violations();

    // Should not report violation for waived file
    assert_eq!(violations.len(), 0);
}

#[test]
fn test_detect_test_file_in_src() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.test.rs",
        r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_hygiene_violations();

    assert!(violations
        .iter()
        .any(|v| matches!(v.rule, repo_hygiene::HygieneRule::TestFileInSrc)));
}

#[test]
fn test_detect_todo_comment() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.rs",
        r#"
pub fn add(a: i32, b: i32) -> i32 {
    // TODO: Optimize this function
    a + b
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_hygiene_violations();

    assert!(violations
        .iter()
        .any(|v| matches!(v.rule, repo_hygiene::HygieneRule::TodoComment)));

    let todo_violation = violations
        .iter()
        .find(|v| matches!(v.rule, repo_hygiene::HygieneRule::TodoComment))
        .unwrap();

    assert_eq!(todo_violation.line_number, Some(3));
}

#[test]
fn test_detect_allow_attribute() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.rs",
        r#"
#[allow(dead_code)]
pub fn unused_function() {
    println!("This is unused");
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_hygiene_violations();

    assert!(violations
        .iter()
        .any(|v| matches!(v.rule, repo_hygiene::HygieneRule::AllowAttribute { .. })));
}

#[test]
fn test_allow_attribute_respects_waiver() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "src/module.rs",
        r#"
#[allow(dead_code)]
pub fn unused_function() {
    println!("This is unused");
}
"#,
    );

    // Create a waiver registry with this file waived
    let waiver_toml = r#"
[[allow_attr_waivers]]
path = "src/module.rs"
justification = "Test waiver"
"#;

    let waiver_path = repo.path().join("waivers.toml");
    fs::write(&waiver_path, waiver_toml).expect("Failed to write waiver file");

    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Failed to load waiver registry");

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), waiver_registry);
    let violations = scanner.scan_hygiene_violations();

    // Should not report allow attribute violations for waived file
    assert!(!violations
        .iter()
        .any(|v| matches!(v.rule, repo_hygiene::HygieneRule::AllowAttribute { .. })));
}

// ============================================================================
// Anti-Pattern Detection Tests
// ============================================================================

#[test]
fn test_detect_host_bypass() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs",
        r#"
impl Panel {
    fn save_file(&self, path: &str, content: &str) {
        self.host.save_file(path, content);
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let bypasses = scanner.scan_host_bypasses();

    assert_eq!(bypasses.len(), 1);
    assert!(bypasses[0].pattern.contains("self.host.save_file"));
}

#[test]
fn test_detect_multiple_host_bypasses() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs",
        r#"
impl Panel {
    fn save_file(&self, path: &str, content: &str) {
        self.host.save_file(path, content);
    }
    
    fn open_file(&self, path: &str) -> String {
        self.host.open_file(path)
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let bypasses = scanner.scan_host_bypasses();

    assert_eq!(bypasses.len(), 2);
}

#[test]
fn test_detect_registration_blob() {
    let repo = create_test_repo();

    // Create a large registration file
    let mut content = String::from("// Registration module\n");
    for i in 0..250 {
        content.push_str(&format!("fn register_item_{}() {{}}\n", i));
    }

    create_file(
        &repo,
        "5.editor/l7.0-editor-command-spine/src/registration.rs",
        &content,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let blobs = scanner.scan_registration_blobs();

    assert_eq!(blobs.len(), 1);
    assert!(blobs[0].line_count > 200);
}

#[test]
fn test_detect_mixed_concerns_in_registration() {
    let repo = create_test_repo();

    let content = r#"
// Registration module with mixed concerns
fn register_command() {}
fn register_panel() {}
fn register_action() {}
fn register_service() {}
"#;

    // Make it long enough to trigger blob detection
    let mut long_content = content.to_string();
    for i in 0..200 {
        long_content.push_str(&format!("// Line {}\n", i));
    }

    create_file(
        &repo,
        "5.editor/l7.0-editor-command-spine/src/registration.rs",
        &long_content,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let blobs = scanner.scan_registration_blobs();

    assert_eq!(blobs.len(), 1);
    assert!(blobs[0].mixed_concerns.len() >= 4);
}

#[test]
fn test_detect_parser_in_ui() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs",
        r#"
impl Panel {
    fn parse_input(&self, input: &str) -> Result<Data, Error> {
        // Parser implementation
        Ok(Data::default())
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_domain_logic_in_ui();

    assert_eq!(violations.len(), 1);
    assert_eq!(
        violations[0].violation_type,
        repo_hygiene::DomainLogicType::Parser
    );
}

#[test]
fn test_detect_validator_in_ui() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs",
        r#"
impl Panel {
    fn validate_input(&self, input: &str) -> bool {
        // Validation logic
        true
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_domain_logic_in_ui();

    assert_eq!(violations.len(), 1);
    assert_eq!(
        violations[0].violation_type,
        repo_hygiene::DomainLogicType::Validator
    );
}

#[test]
fn test_detect_business_rule_in_ui() {
    let repo = create_test_repo();
    create_file(
        &repo,
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs",
        r#"
impl Panel {
    fn apply_business_rule(&self, data: &Data) -> Result<(), Error> {
        // Business rule implementation
        Ok(())
    }
}
"#,
    );

    let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
    let violations = scanner.scan_domain_logic_in_ui();

    assert_eq!(violations.len(), 1);
    assert_eq!(
        violations[0].violation_type,
        repo_hygiene::DomainLogicType::BusinessRule
    );
}
