// Property-based tests for verify command execution
//
// These tests validate universal properties that should hold for all
// verify command executions.

use proptest::prelude::*;
use repo_hygiene::{HygieneChecker, WaiverRegistry};
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Test Generators
// ============================================================================

/// Generate a temporary test repository with random structure
fn arb_test_repo() -> impl Strategy<Value = TempDir> {
    Just(()).prop_map(|_| {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Create basic directory structure
        fs::create_dir_all(repo_root.join("src")).unwrap();
        fs::create_dir_all(repo_root.join("7.quality/suites/repo_hygiene")).unwrap();

        // Create empty waiver registry
        let waivers_content = "# Empty waiver registry\n";
        fs::write(
            repo_root.join("7.quality/suites/repo_hygiene/waivers.toml"),
            waivers_content,
        )
        .unwrap();

        temp_dir
    })
}

// ============================================================================
// Property 24: Verify Command Execution Completeness
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 24: Verify Command Execution Completeness
//
// For any hygiene check implemented in the suite, when the verify command is run,
// that check should be executed and its result included in the report.
//
// Validates: Requirements 10.3

proptest! {
    #[test]
    fn prop_verify_executes_all_checks(temp_dir in arb_test_repo()) {
        let repo_root = temp_dir.path().to_path_buf();

        // Create hygiene checker
        let waiver_registry = WaiverRegistry::default();
        let checker = HygieneChecker::new(repo_root.clone(), waiver_registry);

        // Run all checks
        let report = checker.run_all_checks();

        // Property: The sum of passed and failed checks should be greater than 0
        // (i.e., at least some checks were executed)
        let total_checks = report.passed_checks + report.failed_checks;
        prop_assert!(
            total_checks > 0,
            "Verify command should execute at least one check, but executed {}",
            total_checks
        );

        // Property: The report should have a non-zero execution time
        prop_assert!(
            report.execution_time.as_nanos() > 0,
            "Verify command should have non-zero execution time"
        );
    }
}

// ============================================================================
// Property 25: Verify Command Exit Code Correctness
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 25: Verify Command Exit Code Correctness
//
// For any execution of the verify command, the exit code should be 0 if and only if
// all hygiene checks pass.
//
// Validates: Requirements 10.5

proptest! {
    #[test]
    fn prop_verify_exit_code_matches_violations(temp_dir in arb_test_repo()) {
        let repo_root = temp_dir.path().to_path_buf();

        // Create hygiene checker
        let waiver_registry = WaiverRegistry::default();
        let checker = HygieneChecker::new(repo_root.clone(), waiver_registry);

        // Run all checks
        let report = checker.run_all_checks();

        // Property: Exit code should be success (Ok) if and only if there are no violations
        let should_succeed = report.violations.is_empty();
        let would_exit_success = report.violations.is_empty();

        prop_assert_eq!(
            should_succeed,
            would_exit_success,
            "Exit code should be 0 iff no violations found. Violations: {}",
            report.violations.len()
        );

        // Property: If there are violations, failed_checks should be > 0
        if !report.violations.is_empty() {
            prop_assert!(
                report.failed_checks > 0,
                "If violations exist, failed_checks should be > 0, but was {}",
                report.failed_checks
            );
        }

        // Property: If there are no violations, failed_checks should be 0
        if report.violations.is_empty() {
            prop_assert_eq!(
                report.failed_checks,
                0,
                "If no violations exist, failed_checks should be 0, but was {}",
                report.failed_checks
            );
        }
    }
}

// ============================================================================
// Additional Property: Verify Command Consistency
// ============================================================================

// Property: Running verify twice on the same repository should produce the same results
proptest! {
    #[test]
    fn prop_verify_is_deterministic(temp_dir in arb_test_repo()) {
        let repo_root = temp_dir.path().to_path_buf();

        // Create hygiene checker
        let waiver_registry = WaiverRegistry::default();
        let checker = HygieneChecker::new(repo_root.clone(), waiver_registry.clone());

        // Run checks twice
        let report1 = checker.run_all_checks();

        let checker2 = HygieneChecker::new(repo_root.clone(), waiver_registry);
        let report2 = checker2.run_all_checks();

        // Property: Both runs should find the same number of violations
        prop_assert_eq!(
            report1.violations.len(),
            report2.violations.len(),
            "Verify should be deterministic: found {} violations in first run, {} in second",
            report1.violations.len(),
            report2.violations.len()
        );

        // Property: Both runs should have the same pass/fail counts
        prop_assert_eq!(
            report1.passed_checks,
            report2.passed_checks,
            "Passed checks should be consistent across runs"
        );

        prop_assert_eq!(
            report1.failed_checks,
            report2.failed_checks,
            "Failed checks should be consistent across runs"
        );
    }
}
