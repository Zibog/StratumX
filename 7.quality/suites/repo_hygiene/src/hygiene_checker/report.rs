use super::HygieneChecker;
use crate::models::HygieneReport;
use std::time::Instant;

pub fn run_all_checks(checker: &HygieneChecker) -> HygieneReport {
    let start_time = Instant::now();
    let mut all_violations = Vec::new();
    let mut passed_checks = 0;
    let mut failed_checks = 0;

    for violations in [
        checker.check_line_limits(),
        checker.check_test_files_in_src(),
        checker.check_todo_comments(),
        checker.check_allow_attributes(),
        checker.check_prohibitions(),
    ] {
        if violations.is_empty() {
            passed_checks += 1;
        } else {
            failed_checks += 1;
            all_violations.extend(violations);
        }
    }

    HygieneReport {
        violations: all_violations,
        passed_checks,
        failed_checks,
        execution_time: start_time.elapsed(),
    }
}
