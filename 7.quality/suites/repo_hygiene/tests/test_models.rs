// Unit tests for repository hygiene data models
//
// Tests enum variants, type construction, and basic functionality
// Requirements: 1.1-1.5

use std::path::PathBuf;
use std::time::Duration;

// Import the models module from the repo_hygiene crate
use repo_hygiene::models::*;

// ============================================================================
// HygieneRule Tests
// ============================================================================

#[test]
fn test_hygiene_rule_line_limit_variant() {
    let rule = HygieneRule::LineLimit {
        limit: 200,
        actual: 250,
    };

    match rule {
        HygieneRule::LineLimit { limit, actual } => {
            assert_eq!(limit, 200);
            assert_eq!(actual, 250);
        }
        _ => panic!("Expected LineLimit variant"),
    }
}

#[test]
fn test_hygiene_rule_test_file_in_src_variant() {
    let rule = HygieneRule::TestFileInSrc;
    assert!(matches!(rule, HygieneRule::TestFileInSrc));
}

#[test]
fn test_hygiene_rule_todo_comment_variant() {
    let rule = HygieneRule::TodoComment;
    assert!(matches!(rule, HygieneRule::TodoComment));
}

#[test]
fn test_hygiene_rule_allow_attribute_variant() {
    let rule = HygieneRule::AllowAttribute {
        attr: "dead_code".to_string(),
    };

    match rule {
        HygieneRule::AllowAttribute { attr } => {
            assert_eq!(attr, "dead_code");
        }
        _ => panic!("Expected AllowAttribute variant"),
    }
}

#[test]
fn test_hygiene_rule_duplicate_module_variant() {
    let rule = HygieneRule::DuplicateModule {
        other_location: PathBuf::from("src/other/module.rs"),
    };

    match rule {
        HygieneRule::DuplicateModule { other_location } => {
            assert_eq!(other_location, PathBuf::from("src/other/module.rs"));
        }
        _ => panic!("Expected DuplicateModule variant"),
    }
}

#[test]
fn test_hygiene_rule_equality() {
    let rule1 = HygieneRule::TodoComment;
    let rule2 = HygieneRule::TodoComment;
    assert_eq!(rule1, rule2);

    let rule3 = HygieneRule::TestFileInSrc;
    assert_ne!(rule1, rule3);
}

#[test]
fn test_hygiene_rule_clone() {
    let rule = HygieneRule::LineLimit {
        limit: 200,
        actual: 300,
    };
    let cloned = rule.clone();
    assert_eq!(rule, cloned);
}

// ============================================================================
// HygieneViolation Tests
// ============================================================================

#[test]
fn test_hygiene_violation_construction() {
    let violation = HygieneViolation {
        path: PathBuf::from("src/main.rs"),
        rule: HygieneRule::TodoComment,
        line_number: Some(42),
        context: "// TODO: fix this".to_string(),
        waived: false,
        remediation: "Remove TODO comment or create tracking issue".to_string(),
    };

    assert_eq!(violation.path, PathBuf::from("src/main.rs"));
    assert_eq!(violation.rule, HygieneRule::TodoComment);
    assert_eq!(violation.line_number, Some(42));
    assert_eq!(violation.context, "// TODO: fix this");
    assert!(!violation.waived);
    assert_eq!(
        violation.remediation,
        "Remove TODO comment or create tracking issue"
    );
}

#[test]
fn test_hygiene_violation_without_line_number() {
    let violation = HygieneViolation {
        path: PathBuf::from("src/lib.rs"),
        rule: HygieneRule::TestFileInSrc,
        line_number: None,
        context: "test_module.rs".to_string(),
        waived: false,
        remediation: "Move test file to tests/ directory".to_string(),
    };

    assert_eq!(violation.line_number, None);
}

#[test]
fn test_hygiene_violation_waived() {
    let violation = HygieneViolation {
        path: PathBuf::from("src/legacy.rs"),
        rule: HygieneRule::LineLimit {
            limit: 200,
            actual: 500,
        },
        line_number: None,
        context: "Legacy module".to_string(),
        waived: true,
        remediation: "Decompose into smaller modules".to_string(),
    };

    assert!(violation.waived);
}

#[test]
fn test_hygiene_violation_equality() {
    let v1 = HygieneViolation {
        path: PathBuf::from("src/main.rs"),
        rule: HygieneRule::TodoComment,
        line_number: Some(10),
        context: "TODO".to_string(),
        waived: false,
        remediation: "Fix".to_string(),
    };

    let v2 = v1.clone();
    assert_eq!(v1, v2);
}

// ============================================================================
// HygieneReport Tests
// ============================================================================

#[test]
fn test_hygiene_report_construction() {
    let violations = vec![HygieneViolation {
        path: PathBuf::from("src/a.rs"),
        rule: HygieneRule::TodoComment,
        line_number: Some(1),
        context: "TODO".to_string(),
        waived: false,
        remediation: "Fix".to_string(),
    }];

    let report = HygieneReport {
        violations: violations.clone(),
        passed_checks: 10,
        failed_checks: 1,
        execution_time: Duration::from_secs(5),
    };

    assert_eq!(report.violations.len(), 1);
    assert_eq!(report.passed_checks, 10);
    assert_eq!(report.failed_checks, 1);
    assert_eq!(report.execution_time, Duration::from_secs(5));
}

#[test]
fn test_hygiene_report_empty_violations() {
    let report = HygieneReport {
        violations: vec![],
        passed_checks: 15,
        failed_checks: 0,
        execution_time: Duration::from_millis(100),
    };

    assert!(report.violations.is_empty());
    assert_eq!(report.passed_checks, 15);
    assert_eq!(report.failed_checks, 0);
}

// ============================================================================
// TestType Tests
// ============================================================================

#[test]
fn test_test_type_variants() {
    let unit = TestType::Unit;
    let integration = TestType::Integration;
    let property = TestType::Property;
    let invariant = TestType::Invariant;
    let smoke = TestType::Smoke;

    assert!(matches!(unit, TestType::Unit));
    assert!(matches!(integration, TestType::Integration));
    assert!(matches!(property, TestType::Property));
    assert!(matches!(invariant, TestType::Invariant));
    assert!(matches!(smoke, TestType::Smoke));
}

#[test]
fn test_test_type_equality() {
    assert_eq!(TestType::Unit, TestType::Unit);
    assert_ne!(TestType::Unit, TestType::Integration);
}

#[test]
fn test_test_type_detect_property_test() {
    let path = PathBuf::from("src/test.rs");
    let content = r#"
        proptest! {
            fn test_something(x in 0..100) {
                assert!(x < 100);
            }
        }
    "#;

    let test_type = TestType::detect_from_file(&path, content);
    assert_eq!(test_type, TestType::Property);
}

#[test]
fn test_test_type_detect_property_test_with_compose() {
    let path = PathBuf::from("src/test.rs");
    let content = r#"
        prop_compose! {
            fn arb_value()(x in 0..100) -> Value {
                Value(x)
            }
        }
    "#;

    let test_type = TestType::detect_from_file(&path, content);
    assert_eq!(test_type, TestType::Property);
}

#[test]
fn test_test_type_detect_invariant_test() {
    let path = PathBuf::from("src/test.rs");
    let content = r#"
        invariant! {
            fn test_invariant() {
                assert_eq!(1 + 1, 2);
            }
        }
    "#;

    let test_type = TestType::detect_from_file(&path, content);
    assert_eq!(test_type, TestType::Invariant);
}

#[test]
fn test_test_type_detect_smoke_test() {
    let path = PathBuf::from("src/smoke_test.rs");
    let content = r#"
        #[test]
        fn test_basic() {
            assert_eq!(2 + 2, 4);
        }
    "#;

    let test_type = TestType::detect_from_file(&path, content);
    assert_eq!(test_type, TestType::Smoke);
}

#[test]
fn test_test_type_detect_integration_test() {
    let path = PathBuf::from("tests/integration_test.rs");
    let content = r#"
        #[test]
        fn test_integration() {
            assert_eq!(3 + 3, 6);
        }
    "#;

    let test_type = TestType::detect_from_file(&path, content);
    assert_eq!(test_type, TestType::Integration);
}

#[test]
fn test_test_type_detect_unit_test() {
    let path = PathBuf::from("src/module.rs");
    let content = r#"
        #[test]
        fn test_unit() {
            assert_eq!(4 + 4, 8);
        }
    "#;

    let test_type = TestType::detect_from_file(&path, content);
    assert_eq!(test_type, TestType::Unit);
}

// ============================================================================
// TestFileCandidate Tests
// ============================================================================

#[test]
fn test_test_file_candidate_construction() {
    let candidate = TestFileCandidate {
        path: PathBuf::from("src/test.rs"),
        test_type: TestType::Unit,
        target_suite: "editor_canon_matrix/command_spine".to_string(),
        dependencies: vec!["module_a".to_string(), "module_b".to_string()],
    };

    assert_eq!(candidate.path, PathBuf::from("src/test.rs"));
    assert_eq!(candidate.test_type, TestType::Unit);
    assert_eq!(candidate.target_suite, "editor_canon_matrix/command_spine");
    assert_eq!(candidate.dependencies.len(), 2);
}

#[test]
fn test_test_file_candidate_no_dependencies() {
    let candidate = TestFileCandidate {
        path: PathBuf::from("src/test.rs"),
        test_type: TestType::Property,
        target_suite: "editor_app_matrix/desktop_app".to_string(),
        dependencies: vec![],
    };

    assert!(candidate.dependencies.is_empty());
}

#[test]
fn test_test_file_candidate_equality() {
    let c1 = TestFileCandidate {
        path: PathBuf::from("src/test.rs"),
        test_type: TestType::Unit,
        target_suite: "suite".to_string(),
        dependencies: vec![],
    };

    let c2 = c1.clone();
    assert_eq!(c1, c2);
}

// ============================================================================
// CompilationStatus Tests
// ============================================================================

#[test]
fn test_compilation_status_variants() {
    let success = CompilationStatus::Success;
    let failed = CompilationStatus::Failed;
    let not_attempted = CompilationStatus::NotAttempted;

    assert!(matches!(success, CompilationStatus::Success));
    assert!(matches!(failed, CompilationStatus::Failed));
    assert!(matches!(not_attempted, CompilationStatus::NotAttempted));
}

#[test]
fn test_compilation_status_equality() {
    assert_eq!(CompilationStatus::Success, CompilationStatus::Success);
    assert_ne!(CompilationStatus::Success, CompilationStatus::Failed);
}

// ============================================================================
// MigrationResult Tests
// ============================================================================

#[test]
fn test_migration_result_construction() {
    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("tests/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![PathBuf::from("proptest-regressions/test.txt")],
        compilation_status: CompilationStatus::Success,
    };

    assert_eq!(result.source_path, PathBuf::from("src/test.rs"));
    assert_eq!(result.destination_path, PathBuf::from("tests/test.rs"));
    assert_eq!(result.imports_updated, 3);
    assert_eq!(result.regression_files_moved.len(), 1);
    assert_eq!(result.compilation_status, CompilationStatus::Success);
}

#[test]
fn test_migration_result_no_regression_files() {
    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("tests/test.rs"),
        imports_updated: 0,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::NotAttempted,
    };

    assert!(result.regression_files_moved.is_empty());
    assert_eq!(result.imports_updated, 0);
}

#[test]
fn test_migration_result_equality() {
    let r1 = MigrationResult {
        source_path: PathBuf::from("src/a.rs"),
        destination_path: PathBuf::from("tests/a.rs"),
        imports_updated: 1,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    let r2 = r1.clone();
    assert_eq!(r1, r2);
}

// ============================================================================
// HostBypassPattern Tests
// ============================================================================

#[test]
fn test_host_bypass_pattern_construction() {
    let bypass = HostBypassPattern {
        file: PathBuf::from("src/panel.rs"),
        line: 42,
        pattern: "self.host.save_file(path, content)".to_string(),
        suggested_action: "Route through Command_Spine SaveFile action".to_string(),
    };

    assert_eq!(bypass.file, PathBuf::from("src/panel.rs"));
    assert_eq!(bypass.line, 42);
    assert_eq!(bypass.pattern, "self.host.save_file(path, content)");
    assert_eq!(
        bypass.suggested_action,
        "Route through Command_Spine SaveFile action"
    );
}

#[test]
fn test_host_bypass_pattern_equality() {
    let b1 = HostBypassPattern {
        file: PathBuf::from("src/panel.rs"),
        line: 10,
        pattern: "self.host.open()".to_string(),
        suggested_action: "Fix".to_string(),
    };

    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

// ============================================================================
// RegistrationBlob Tests
// ============================================================================

#[test]
fn test_registration_blob_construction() {
    let blob = RegistrationBlob {
        file: PathBuf::from("src/registration.rs"),
        line_count: 500,
        mixed_concerns: vec!["routing".to_string(), "validation".to_string()],
        suggested_decomposition: vec![
            "Extract routing to routing.rs".to_string(),
            "Extract validation to validation.rs".to_string(),
        ],
    };

    assert_eq!(blob.file, PathBuf::from("src/registration.rs"));
    assert_eq!(blob.line_count, 500);
    assert_eq!(blob.mixed_concerns.len(), 2);
    assert_eq!(blob.suggested_decomposition.len(), 2);
}

#[test]
fn test_registration_blob_no_mixed_concerns() {
    let blob = RegistrationBlob {
        file: PathBuf::from("src/registration.rs"),
        line_count: 250,
        mixed_concerns: vec![],
        suggested_decomposition: vec!["Split into smaller modules".to_string()],
    };

    assert!(blob.mixed_concerns.is_empty());
}

#[test]
fn test_registration_blob_equality() {
    let b1 = RegistrationBlob {
        file: PathBuf::from("src/reg.rs"),
        line_count: 300,
        mixed_concerns: vec![],
        suggested_decomposition: vec![],
    };

    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

// ============================================================================
// DomainLogicType Tests
// ============================================================================

#[test]
fn test_domain_logic_type_variants() {
    let parser = DomainLogicType::Parser;
    let validator = DomainLogicType::Validator;
    let business_rule = DomainLogicType::BusinessRule;

    assert!(matches!(parser, DomainLogicType::Parser));
    assert!(matches!(validator, DomainLogicType::Validator));
    assert!(matches!(business_rule, DomainLogicType::BusinessRule));
}

#[test]
fn test_domain_logic_type_equality() {
    assert_eq!(DomainLogicType::Parser, DomainLogicType::Parser);
    assert_ne!(DomainLogicType::Parser, DomainLogicType::Validator);
}

// ============================================================================
// DomainLogicViolation Tests
// ============================================================================

#[test]
fn test_domain_logic_violation_construction() {
    let violation = DomainLogicViolation {
        file: PathBuf::from("src/ui/panel.rs"),
        violation_type: DomainLogicType::Parser,
        line_range: (100, 150),
        suggested_target_layer: "parser service layer".to_string(),
    };

    assert_eq!(violation.file, PathBuf::from("src/ui/panel.rs"));
    assert_eq!(violation.violation_type, DomainLogicType::Parser);
    assert_eq!(violation.line_range, (100, 150));
    assert_eq!(violation.suggested_target_layer, "parser service layer");
}

#[test]
fn test_domain_logic_violation_validator_type() {
    let violation = DomainLogicViolation {
        file: PathBuf::from("src/ui/form.rs"),
        violation_type: DomainLogicType::Validator,
        line_range: (50, 75),
        suggested_target_layer: "validation service".to_string(),
    };

    assert_eq!(violation.violation_type, DomainLogicType::Validator);
}

#[test]
fn test_domain_logic_violation_equality() {
    let v1 = DomainLogicViolation {
        file: PathBuf::from("src/ui.rs"),
        violation_type: DomainLogicType::BusinessRule,
        line_range: (10, 20),
        suggested_target_layer: "business layer".to_string(),
    };

    let v2 = v1.clone();
    assert_eq!(v1, v2);
}

// ============================================================================
// CleanupReport Tests
// ============================================================================

#[test]
fn test_cleanup_report_construction() {
    let host_bypasses = vec![HostBypassPattern {
        file: PathBuf::from("src/panel.rs"),
        line: 10,
        pattern: "self.host.save()".to_string(),
        suggested_action: "Fix".to_string(),
    }];

    let registration_blobs = vec![RegistrationBlob {
        file: PathBuf::from("src/reg.rs"),
        line_count: 300,
        mixed_concerns: vec![],
        suggested_decomposition: vec![],
    }];

    let domain_logic_violations = vec![DomainLogicViolation {
        file: PathBuf::from("src/ui.rs"),
        violation_type: DomainLogicType::Parser,
        line_range: (1, 10),
        suggested_target_layer: "service".to_string(),
    }];

    let report = CleanupReport::new(
        host_bypasses.clone(),
        registration_blobs.clone(),
        domain_logic_violations.clone(),
    );

    assert_eq!(report.host_bypasses.len(), 1);
    assert_eq!(report.registration_blobs.len(), 1);
    assert_eq!(report.domain_logic_violations.len(), 1);
    assert_eq!(report.total_issues, 3);
}

#[test]
fn test_cleanup_report_empty() {
    let report = CleanupReport::new(vec![], vec![], vec![]);

    assert!(report.host_bypasses.is_empty());
    assert!(report.registration_blobs.is_empty());
    assert!(report.domain_logic_violations.is_empty());
    assert_eq!(report.total_issues, 0);
}

#[test]
fn test_cleanup_report_total_issues_calculation() {
    let host_bypasses = vec![
        HostBypassPattern {
            file: PathBuf::from("a.rs"),
            line: 1,
            pattern: "bypass1".to_string(),
            suggested_action: "fix1".to_string(),
        },
        HostBypassPattern {
            file: PathBuf::from("b.rs"),
            line: 2,
            pattern: "bypass2".to_string(),
            suggested_action: "fix2".to_string(),
        },
    ];

    let registration_blobs = vec![RegistrationBlob {
        file: PathBuf::from("c.rs"),
        line_count: 300,
        mixed_concerns: vec![],
        suggested_decomposition: vec![],
    }];

    let report = CleanupReport::new(host_bypasses, registration_blobs, vec![]);

    assert_eq!(report.total_issues, 3);
}

// ============================================================================
// WaiverError Tests
// ============================================================================

#[test]
fn test_waiver_error_construction() {
    let error = WaiverError {
        path: PathBuf::from("src/legacy.rs"),
        rule: "line_limit".to_string(),
        error: "File no longer exists".to_string(),
    };

    assert_eq!(error.path, PathBuf::from("src/legacy.rs"));
    assert_eq!(error.rule, "line_limit");
    assert_eq!(error.error, "File no longer exists");
}

#[test]
fn test_waiver_error_equality() {
    let e1 = WaiverError {
        path: PathBuf::from("src/a.rs"),
        rule: "rule1".to_string(),
        error: "error1".to_string(),
    };

    let e2 = e1.clone();
    assert_eq!(e1, e2);
}
