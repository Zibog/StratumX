// Property-based tests for HygieneChecker
//
// This module contains property tests that validate universal correctness
// properties of the hygiene checker across a wide range of inputs.

use proptest::prelude::*;
use repo_hygiene::{HygieneChecker, HygieneRule, WaiverRegistry};
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a temporary test repository
fn create_test_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path();
    fs::create_dir_all(repo_root.join("src")).unwrap();
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
// Property Test Generators
// ============================================================================

/// Generate arbitrary Rust code content with prohibited patterns
fn arb_code_with_prohibitions() -> impl Strategy<Value = (String, Vec<&'static str>)> {
    prop::collection::vec(
        prop_oneof![
            Just((
                "static mut GLOBAL: i32 = 0;".to_string(),
                vec!["static mut"]
            )),
            Just(("pub fn execute_command() {}".to_string(), vec!["execute_*"])),
            Just((
                "self.host.save_file(path);".to_string(),
                vec!["host bypass"]
            )),
            Just((
                "let path = \"/home/user/file.txt\";".to_string(),
                vec!["hardcoded path"]
            )),
            Just((
                "let path = \"C:\\\\Users\\\\file.txt\";".to_string(),
                vec!["hardcoded path"]
            )),
            Just((
                "let default_fake_value = 42;".to_string(),
                vec!["fake truth"]
            )),
            Just(("fn normal_function() {}".to_string(), vec![])),
        ],
        1..10,
    )
    .prop_map(|patterns| {
        let code = patterns
            .iter()
            .map(|(c, _)| c.clone())
            .collect::<Vec<_>>()
            .join("\n");
        let expected: Vec<&'static str> = patterns
            .iter()
            .flat_map(|(_, p)| p.iter().copied())
            .collect();
        (code, expected)
    })
}

/// Generate arbitrary line counts
fn arb_line_count() -> impl Strategy<Value = usize> {
    prop_oneof![
        (1..=200usize).prop_map(|n| n),   // Under limit
        (201..=500usize).prop_map(|n| n), // Over limit
    ]
}

/// Generate arbitrary TODO comment variations
fn arb_todo_comment() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("// TODO: implement this".to_string()),
        Just("// todo: fix this".to_string()),
        Just("// Todo: refactor".to_string()),
        Just("/* TODO: cleanup */".to_string()),
    ]
}

/// Generate arbitrary allow attributes
fn arb_allow_attribute() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("#[allow(dead_code)]".to_string()),
        Just("#[allow(unused_variables)]".to_string()),
        Just("#[allow(clippy::all)]".to_string()),
        Just("#[allow(improper_ctypes)]".to_string()),
    ]
}

// ============================================================================
// Property 20: Prohibition Pattern Detection
// Feature: repo-sanitization-phase-1, Property 20: Prohibition Pattern Detection
// Validates: Requirements 8.1-8.6
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_prohibition_detection_finds_all_patterns(
        (code, expected_patterns) in arb_code_with_prohibitions()
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        create_test_file(repo_root, "src/test.rs", &code);

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let violations = checker.check_prohibitions();

        // Property: For any code containing prohibited patterns,
        // the checker should detect at least as many violations as expected patterns
        prop_assert!(
            violations.len() >= expected_patterns.len(),
            "Expected at least {} violations for patterns {:?}, but found {}",
            expected_patterns.len(),
            expected_patterns,
            violations.len()
        );

        // Property: Each violation should have non-empty remediation
        for violation in &violations {
            prop_assert!(
                !violation.remediation.is_empty(),
                "Violation should include remediation guidance"
            );
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_prohibition_violations_include_line_numbers(
        pattern in prop_oneof![
            Just("static mut GLOBAL: i32 = 0;"),
            Just("pub fn execute_command() {}"),
            Just("self.host.save_file(path);"),
        ]
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        let code = format!("// Line 1\n{}\n// Line 3", pattern);
        create_test_file(repo_root, "src/test.rs", &code);

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let violations = checker.check_prohibitions();

        // Property: All prohibition violations should include line numbers
        prop_assert!(!violations.is_empty(), "Should detect at least one violation");

        for violation in &violations {
            prop_assert!(
                violation.line_number.is_some(),
                "Prohibition violation should include line number"
            );

            // Line number should be 2 (where the pattern is)
            prop_assert_eq!(
                violation.line_number,
                Some(2),
                "Line number should point to the violation"
            );
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_prohibition_violations_include_context(
        pattern in prop_oneof![
            Just("static mut GLOBAL: i32 = 0;"),
            Just("self.host.save_file(path);"),
        ]
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        create_test_file(repo_root, "src/test.rs", pattern);

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let violations = checker.check_prohibitions();

        // Property: All violations should include context (the offending line)
        prop_assert!(!violations.is_empty());

        for violation in &violations {
            prop_assert!(
                !violation.context.is_empty(),
                "Violation should include context"
            );

            prop_assert!(
                violation.context.contains(pattern.trim()),
                "Context should contain the offending pattern"
            );
        }
    }
}

// ============================================================================
// Property 19: Violation Report Completeness
// Feature: repo-sanitization-phase-1, Property 19: Violation Report Completeness
// Validates: Requirements 6.10-6.11
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_violation_report_includes_all_required_fields(
        line_count in arb_line_count(),
        todo_comment in arb_todo_comment(),
        allow_attr in arb_allow_attribute()
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        // Create file with line limit violation
        if line_count > 200 {
            let content = (0..line_count)
                .map(|i| format!("// Line {}", i))
                .collect::<Vec<_>>()
                .join("\n");
            create_test_file(repo_root, "src/large.rs", &content);
        }

        // Create file with TODO comment
        create_test_file(repo_root, "src/todo.rs", &todo_comment);

        // Create file with allow attribute
        create_test_file(repo_root, "src/allow.rs", &allow_attr);

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let report = checker.run_all_checks();

        // Property: For any hygiene violation detected, the report should include
        // file path, rule name, and context
        for violation in &report.violations {
            prop_assert!(
                !violation.path.as_os_str().is_empty(),
                "Violation should include file path"
            );

            prop_assert!(
                !violation.context.is_empty(),
                "Violation should include context"
            );

            prop_assert!(
                !violation.remediation.is_empty(),
                "Violation should include remediation guidance"
            );

            // Line number should be present for TODO and allow attribute violations
            match violation.rule {
                HygieneRule::TodoComment | HygieneRule::AllowAttribute { .. } => {
                    prop_assert!(
                        violation.line_number.is_some(),
                        "TODO and allow attribute violations should include line number"
                    );
                }
                _ => {}
            }
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_report_tracks_passed_and_failed_checks(
        has_violation in prop::bool::ANY
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        if has_violation {
            // Create a file with a violation
            create_test_file(repo_root, "src/test.rs", "// TODO: implement");
        } else {
            // Create a clean file
            create_test_file(repo_root, "src/test.rs", "fn example() {}");
        }

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let report = checker.run_all_checks();

        // Property: The sum of passed and failed checks should equal total checks
        let total_checks = report.passed_checks + report.failed_checks;
        prop_assert_eq!(
            total_checks,
            5,
            "Should run exactly 5 check types"
        );

        // Property: If violations exist, failed_checks should be > 0
        if !report.violations.is_empty() {
            prop_assert!(
                report.failed_checks > 0,
                "Failed checks should be > 0 when violations exist"
            );
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_report_execution_time_is_non_negative(
        file_count in 1..10usize
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        // Create multiple files
        for i in 0..file_count {
            create_test_file(
                repo_root,
                &format!("src/file{}.rs", i),
                "fn example() {}"
            );
        }

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let report = checker.run_all_checks();

        // Property: Execution time should always be non-negative
        prop_assert!(
            report.execution_time.as_secs() < 3600,
            "Execution time should be reasonable (less than 1 hour)"
        );
    }
}

// ============================================================================
// Additional Property Tests for Line Limits
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_line_limit_check_correctness(line_count in arb_line_count()) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        let content = (0..line_count)
            .map(|i| format!("// Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        create_test_file(repo_root, "src/test.rs", &content);

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let violations = checker.check_line_limits();

        // Property: Files over 200 lines should be reported, files under should not
        if line_count > 200 {
            prop_assert_eq!(
                violations.len(),
                1,
                "Should report violation for file with {} lines",
                line_count
            );

            match &violations[0].rule {
                HygieneRule::LineLimit { limit, actual } => {
                    prop_assert_eq!(*limit, 200);
                    prop_assert_eq!(*actual, line_count);
                }
                _ => prop_assert!(false, "Expected LineLimit rule"),
            }
        } else {
            prop_assert_eq!(
                violations.len(),
                0,
                "Should not report violation for file with {} lines",
                line_count
            );
        }
    }
}

// ============================================================================
// Additional Property Tests for TODO Comments
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_todo_detection_is_case_insensitive(
        todo_variant in prop_oneof![
            Just("TODO"),
            Just("todo"),
            Just("Todo"),
            Just("tOdO"),
        ]
    ) {
        let temp_dir = create_test_repo();
        let repo_root = temp_dir.path();

        let content = format!("// {}: implement this", todo_variant);
        create_test_file(repo_root, "src/test.rs", &content);

        let waiver_registry = WaiverRegistry::new();
        let checker = HygieneChecker::new(repo_root.to_path_buf(), waiver_registry);

        let violations = checker.check_todo_comments();

        // Property: TODO detection should be case-insensitive
        prop_assert_eq!(
            violations.len(),
            1,
            "Should detect TODO in any case: {}",
            todo_variant
        );
    }
}
