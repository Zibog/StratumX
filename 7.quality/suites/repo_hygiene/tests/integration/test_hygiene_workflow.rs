// Integration test for hygiene check workflow
// Requirements: 6.1-6.12, 7.1-7.7, 8.1-8.8

use crate::common::*;
use repo_hygiene::{HygieneChecker, HygieneRule, WaiverRegistry};

#[test]
fn test_full_hygiene_check_workflow() {
    // Create mock repository with known violations
    let repo = create_repo_with_violations();

    // Initialize waiver registry (empty for this test)
    let waiver_registry = WaiverRegistry::new();

    // Create hygiene checker
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Run all checks
    let report = checker.run_all_checks();

    // Verify violations were detected
    assert!(!report.violations.is_empty(), "Should detect violations");
    assert!(report.failed_checks > 0, "Should have failed checks");

    // Verify specific violation types were detected
    let has_line_limit_violation = report
        .violations
        .iter()
        .any(|v| matches!(v.rule, HygieneRule::LineLimit { .. }));
    assert!(
        has_line_limit_violation,
        "Should detect line limit violations"
    );

    let has_todo_violation = report
        .violations
        .iter()
        .any(|v| matches!(v.rule, HygieneRule::TodoComment));
    assert!(has_todo_violation, "Should detect TODO comment violations");

    let has_allow_violation = report
        .violations
        .iter()
        .any(|v| matches!(v.rule, HygieneRule::AllowAttribute { .. }));
    assert!(
        has_allow_violation,
        "Should detect #[allow(...)] violations"
    );

    let has_static_mut_violation = report
        .violations
        .iter()
        .any(|v| matches!(v.rule, HygieneRule::StaticMut));
    assert!(
        has_static_mut_violation,
        "Should detect static mut violations"
    );
}

#[test]
fn test_line_limit_check() {
    let repo = create_repo_with_violations();
    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Check line limits
    let violations = checker.check_line_limits();

    // Should detect the 250-line file
    assert!(
        !violations.is_empty(),
        "Should detect line limit violations"
    );

    let long_file_violation = violations
        .iter()
        .find(|v| v.path.to_str().unwrap().contains("long_file.rs"));
    assert!(
        long_file_violation.is_some(),
        "Should detect long_file.rs violation"
    );

    if let Some(violation) = long_file_violation {
        match &violation.rule {
            HygieneRule::LineLimit { limit, actual } => {
                assert_eq!(*limit, 200, "Limit should be 200 lines");
                assert!(*actual > 200, "Actual line count should exceed limit");
            }
            _ => panic!("Expected LineLimit rule"),
        }
    }
}

#[test]
fn test_todo_comment_check() {
    let repo = create_repo_with_violations();
    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Check TODO comments
    let violations = checker.check_todo_comments();

    // Should detect the TODO comment
    assert!(!violations.is_empty(), "Should detect TODO comments");

    let todo_violation = violations
        .iter()
        .find(|v| v.path.to_str().unwrap().contains("with_todo.rs"));
    assert!(
        todo_violation.is_some(),
        "Should detect with_todo.rs violation"
    );

    if let Some(violation) = todo_violation {
        assert!(
            violation.line_number.is_some(),
            "Should include line number for TODO"
        );
        assert!(
            violation.context.contains("TODO"),
            "Context should include TODO comment"
        );
    }
}

#[test]
fn test_allow_attribute_check() {
    let repo = create_repo_with_violations();
    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Check #[allow(...)] attributes
    let violations = checker.check_allow_attributes();

    // Should detect the #[allow(dead_code)] attribute
    assert!(
        !violations.is_empty(),
        "Should detect #[allow(...)] attributes"
    );

    let allow_violation = violations
        .iter()
        .find(|v| v.path.to_str().unwrap().contains("with_allow.rs"));
    assert!(
        allow_violation.is_some(),
        "Should detect with_allow.rs violation"
    );

    if let Some(violation) = allow_violation {
        match &violation.rule {
            HygieneRule::AllowAttribute { attr } => {
                assert!(
                    attr.contains("dead_code"),
                    "Should identify the specific allow attribute"
                );
            }
            _ => panic!("Expected AllowAttribute rule"),
        }
    }
}

#[test]
fn test_prohibition_check() {
    let repo = create_repo_with_violations();
    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Check prohibitions
    let violations = checker.check_prohibitions();

    // Should detect static mut
    assert!(!violations.is_empty(), "Should detect prohibited patterns");

    let static_mut_violation = violations
        .iter()
        .find(|v| v.path.to_str().unwrap().contains("with_static_mut.rs"));
    assert!(
        static_mut_violation.is_some(),
        "Should detect static mut violation"
    );

    if let Some(violation) = static_mut_violation {
        assert!(
            matches!(violation.rule, HygieneRule::StaticMut),
            "Should be StaticMut rule"
        );
        assert!(
            !violation.remediation.is_empty(),
            "Should provide remediation guidance"
        );
    }
}

#[test]
fn test_hygiene_report_format() {
    let repo = create_repo_with_violations();
    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Run all checks
    let report = checker.run_all_checks();

    // Verify report structure
    assert!(
        !report.violations.is_empty(),
        "Report should contain violations"
    );
    assert!(
        report.failed_checks > 0,
        "Report should track failed checks"
    );
    assert!(
        report.execution_time.as_secs() < 30,
        "Should complete within 30 seconds"
    );

    // Verify each violation has required fields
    for violation in &report.violations {
        assert!(
            violation.path.exists() || !violation.path.to_str().unwrap().is_empty(),
            "Violation should have valid path"
        );
        assert!(
            !violation.context.is_empty() || violation.line_number.is_some(),
            "Violation should have context or line number"
        );
        assert!(
            !violation.remediation.is_empty(),
            "Violation should have remediation guidance"
        );
    }
}

#[test]
fn test_hygiene_check_with_no_violations() {
    // Create a clean repository
    let repo = MockRepo::new();
    repo.create_dir("2.engine/l1-foundation/src");

    // Create a clean file (under 200 lines, no violations)
    repo.create_file(
        "2.engine/l1-foundation/src/clean_file.rs",
        r#"
pub fn clean_function() {
    println!("This is a clean file");
}

pub fn another_function() {
    println!("No violations here");
}
"#,
    );

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    // Run all checks
    let report = checker.run_all_checks();

    // Should have no violations
    assert_eq!(
        report.violations.len(),
        0,
        "Clean repository should have no violations"
    );
    assert_eq!(report.failed_checks, 0, "Should have no failed checks");
    assert!(report.passed_checks > 0, "Should have some passed checks");
}

#[test]
fn test_violation_context_extraction() {
    let repo = MockRepo::new();
    repo.create_dir("2.engine/l1-foundation/src");

    // Create file with TODO in specific context
    repo.create_file(
        "2.engine/l1-foundation/src/context_test.rs",
        r#"
pub fn function_one() {
    // TODO: Refactor this function
    println!("Function one");
}

pub fn function_two() {
    // This is fine
    println!("Function two");
}
"#,
    );

    let waiver_registry = WaiverRegistry::new();
    let checker = HygieneChecker::new(repo.root_path().to_path_buf(), waiver_registry);

    let violations = checker.check_todo_comments();

    assert_eq!(violations.len(), 1, "Should detect exactly one TODO");

    let violation = &violations[0];
    assert!(
        violation.context.contains("TODO: Refactor this function"),
        "Context should include the TODO comment text"
    );
    assert_eq!(
        violation.line_number,
        Some(3),
        "Should identify correct line number"
    );
}
