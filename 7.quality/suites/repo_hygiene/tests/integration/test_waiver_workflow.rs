// Integration test for waiver workflow
// Requirements: 9.1-9.7

use crate::common::*;
use repo_hygiene::{HygieneChecker, HygieneRule, WaiverRegistry};

#[test]
fn test_waiver_workflow_with_exemptions() {
    // Create mock repository with violations and waivers
    let repo = create_repo_with_waivers();

    // Load waiver registry
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Create hygiene checker with waiver registry
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Run all checks
    let report = checker.run_all_checks();

    // Verify waived file is not reported
    let waived_file_violation = report
        .violations
        .iter()
        .find(|v| v.path.to_str().unwrap().contains("waived_file.rs"));

    assert!(
        waived_file_violation.is_none(),
        "Waived file should not be reported as violation"
    );
}

#[test]
fn test_waiver_registry_loading() {
    let repo = create_repo_with_waivers();
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");

    // Load waiver registry
    let result = WaiverRegistry::load_from_file(&waiver_path);
    assert!(result.is_ok(), "Should successfully load waiver registry");

    let registry = result.unwrap();

    // Verify waiver entries were loaded
    assert!(
        !registry.line_limit_waivers.is_empty(),
        "Should have line limit waivers"
    );

    // Verify justification is present
    let waiver = &registry.line_limit_waivers[0];
    assert!(
        !waiver.justification.is_empty(),
        "Waiver should have justification"
    );
    assert!(
        waiver.justification.contains("Legacy") || waiver.justification.contains("refactoring"),
        "Justification should explain why waiver is needed"
    );
}

#[test]
fn test_waiver_registry_validation() {
    let repo = create_repo_with_waivers();
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");

    let registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Validate waivers
    let errors = registry.validate_waivers(repo.root_path());

    // Should have no errors since the waived file exists
    assert_eq!(
        errors.len(),
        0,
        "Should have no validation errors for existing files"
    );
}

#[test]
fn test_waiver_registry_detects_stale_entries() {
    let repo = MockRepo::new();
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create waiver registry with non-existent file
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[line_limit_waivers]]
path = "non/existent/file.rs"
justification = "This file doesn't exist"
"#,
    );

    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Validate waivers
    let errors = registry.validate_waivers(repo.root_path());

    // Should detect stale waiver
    assert!(!errors.is_empty(), "Should detect stale waiver entries");
    assert!(
        errors[0].error.contains("does not exist"),
        "Error should indicate file doesn't exist"
    );
}

#[test]
fn test_waiver_mechanism_correctness() {
    let repo = MockRepo::new();
    repo.create_dir("2.engine/l1-foundation/src");
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create a file that violates line limit
    let mut long_content = String::new();
    for i in 0..250 {
        long_content.push_str(&format!("// Line {}\n", i));
    }
    repo.create_file("2.engine/l1-foundation/src/long_file.rs", &long_content);

    // Create waiver for this file
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[line_limit_waivers]]
path = "2.engine/l1-foundation/src/long_file.rs"
justification = "Test waiver"
"#,
    );

    // Load waiver registry
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Check if file is waived
    let rule = HygieneRule::LineLimit {
        limit: 200,
        actual: 250,
    };
    let file_path = std::path::Path::new("2.engine/l1-foundation/src/long_file.rs");

    assert!(
        waiver_registry.is_waived(&rule, file_path),
        "File should be waived for line limit rule"
    );
}

#[test]
fn test_non_waived_violations_still_reported() {
    let repo = MockRepo::new();
    repo.create_dir("2.engine/l1-foundation/src");
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create two files that violate line limit
    let mut long_content = String::new();
    for i in 0..250 {
        long_content.push_str(&format!("// Line {}\n", i));
    }
    repo.create_file("2.engine/l1-foundation/src/waived_file.rs", &long_content);
    repo.create_file(
        "2.engine/l1-foundation/src/not_waived_file.rs",
        &long_content,
    );

    // Create waiver for only one file
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[line_limit_waivers]]
path = "2.engine/l1-foundation/src/waived_file.rs"
justification = "Test waiver"
"#,
    );

    // Load waiver registry
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Create hygiene checker
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Check line limits
    let violations = checker.check_line_limits();

    // Should report only the non-waived file
    assert!(
        !violations.is_empty(),
        "Should detect non-waived violations"
    );

    let has_waived_file = violations
        .iter()
        .any(|v| v.path.file_name().and_then(|name| name.to_str()) == Some("waived_file.rs"));
    assert!(!has_waived_file, "Waived file should not be reported");

    let has_not_waived_file = violations
        .iter()
        .any(|v| v.path.file_name().and_then(|name| name.to_str()) == Some("not_waived_file.rs"));
    assert!(has_not_waived_file, "Non-waived file should be reported");
}

#[test]
fn test_waiver_entry_format_validation() {
    let repo = MockRepo::new();
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create waiver registry with missing justification
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[line_limit_waivers]]
path = "some/file.rs"
justification = ""
"#,
    );

    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let result = WaiverRegistry::load_from_file(&waiver_path);

    // Should fail to load due to empty justification
    assert!(
        result.is_err(),
        "Should reject waiver with empty justification"
    );
    assert!(
        result.unwrap_err().contains("missing justification"),
        "Error should mention missing justification"
    );
}

#[test]
fn test_waiver_for_allow_attributes() {
    let repo = MockRepo::new();
    repo.create_dir("2.engine/l1-foundation/src");
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create file with #[allow(...)] attribute
    repo.create_file(
        "2.engine/l1-foundation/src/with_allow.rs",
        r#"
#[allow(dead_code)]
pub fn unused_function() {
    println!("Never called");
}
"#,
    );

    // Create waiver for allow attribute
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[allow_attr_waivers]]
path = "2.engine/l1-foundation/src/with_allow.rs"
justification = "FFI bridge requires allow attribute"
"#,
    );

    // Load waiver registry
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Create hygiene checker
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Check allow attributes
    let violations = checker.check_allow_attributes();

    // Should not report the waived file
    let has_violation = violations
        .iter()
        .any(|v| v.path.to_str().unwrap().contains("with_allow.rs"));
    assert!(
        !has_violation,
        "Waived allow attribute should not be reported"
    );
}

#[test]
fn test_multiple_waivers_same_file() {
    let repo = MockRepo::new();
    repo.create_dir("2.engine/l1-foundation/src");
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create file with multiple violations
    let mut long_content = String::from("#[allow(dead_code)]\n");
    for i in 0..250 {
        long_content.push_str(&format!("// Line {}\n", i));
    }
    repo.create_file(
        "2.engine/l1-foundation/src/multi_violation.rs",
        &long_content,
    );

    // Create waivers for both violations
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[line_limit_waivers]]
path = "2.engine/l1-foundation/src/multi_violation.rs"
justification = "Legacy file"

[[allow_attr_waivers]]
path = "2.engine/l1-foundation/src/multi_violation.rs"
justification = "Required for FFI"
"#,
    );

    // Load waiver registry
    let waiver_path = repo.root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("Should load waiver registry");

    // Create hygiene checker
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Run all checks
    let report = checker.run_all_checks();

    // Should not report any violations for this file
    let has_violation = report
        .violations
        .iter()
        .any(|v| v.path.to_str().unwrap().contains("multi_violation.rs"));
    assert!(
        !has_violation,
        "File with multiple waivers should not be reported"
    );
}
