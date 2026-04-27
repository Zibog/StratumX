// Property-based tests for migration logger
//
// These tests validate universal properties that should hold for all migration logging operations.

use proptest::prelude::*;
use repo_hygiene::{CompilationStatus, MigrationLogger, MigrationResult};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Generator for arbitrary MigrationResult
fn arb_migration_result() -> impl Strategy<Value = MigrationResult> {
    (
        prop::string::string_regex("[a-z_/]+\\.rs").unwrap(),
        prop::string::string_regex("7\\.quality/suites/[a-z_/]+\\.rs").unwrap(),
        0usize..20,
        prop::collection::vec(
            prop::string::string_regex("proptest-regressions/[a-z_]+\\.txt").unwrap(),
            0..5,
        ),
        prop::sample::select(vec![
            CompilationStatus::Success,
            CompilationStatus::Failed,
            CompilationStatus::NotAttempted,
        ]),
    )
        .prop_map(
            |(source, dest, imports, regression_files, status)| MigrationResult {
                source_path: PathBuf::from(source),
                destination_path: PathBuf::from(dest),
                imports_updated: imports,
                regression_files_moved: regression_files.into_iter().map(PathBuf::from).collect(),
                compilation_status: status,
            },
        )
}

// **Validates: Requirements 11.1, 11.2, 11.7**
// Property 28: Migration Log Completeness
//
// For any test file migration operation, the migration log should contain an entry
// with the source path, destination path, timestamp, and migration status.
#[test]
fn prop_migration_log_completeness() {
    proptest!(|(result in arb_migration_result())| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log the migration
        logger.log_migration(&result).unwrap();

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: Log must contain source path
        prop_assert!(
            content.contains(&result.source_path.to_string_lossy().to_string()),
            "Log must contain source path: {:?}",
            result.source_path
        );

        // Property: Log must contain destination path
        prop_assert!(
            content.contains(&result.destination_path.to_string_lossy().to_string()),
            "Log must contain destination path: {:?}",
            result.destination_path
        );

        // Property: Log must contain timestamp (UTC marker)
        prop_assert!(
            content.contains("UTC"),
            "Log must contain timestamp with UTC marker"
        );

        // Property: Log must contain imports updated count
        prop_assert!(
            content.contains(&format!("**Imports Updated**: {}", result.imports_updated)),
            "Log must contain imports updated count: {}",
            result.imports_updated
        );

        // Property: Log must contain compilation status
        let status_str = match result.compilation_status {
            CompilationStatus::Success => "Success",
            CompilationStatus::Failed => "Failed",
            CompilationStatus::NotAttempted => "Not Attempted",
        };
        prop_assert!(
            content.contains(&format!("**Status**: {}", status_str)),
            "Log must contain compilation status: {}",
            status_str
        );

        // Property: If regression files exist, they must be in the log
        if !result.regression_files_moved.is_empty() {
            for regression_file in &result.regression_files_moved {
                prop_assert!(
                    content.contains(&regression_file.to_string_lossy().to_string()),
                    "Log must contain regression file: {:?}",
                    regression_file
                );
            }
        } else {
            // If no regression files, log should say "None"
            prop_assert!(
                content.contains("**Regression Files**: None"),
                "Log must indicate no regression files"
            );
        }
    });
}

// Property: Multiple migrations should all be logged
#[test]
fn prop_multiple_migrations_all_logged() {
    proptest!(|(results in prop::collection::vec(arb_migration_result(), 1..10))| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log all migrations
        for result in &results {
            logger.log_migration(result).unwrap();
        }

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: All source paths must be in the log
        for result in &results {
            prop_assert!(
                content.contains(&result.source_path.to_string_lossy().to_string()),
                "Log must contain all source paths: {:?}",
                result.source_path
            );
        }
    });
}

// Property: Log file should always be valid after any number of operations
#[test]
fn prop_log_file_always_valid() {
    proptest!(|(results in prop::collection::vec(arb_migration_result(), 1..20))| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log all migrations
        for result in &results {
            logger.log_migration(result).unwrap();
        }

        // Property: Log file must exist
        prop_assert!(logger.log_path().exists(), "Log file must exist");

        // Property: Log file must be readable
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: Log file must have header
        prop_assert!(
            content.contains("# Phase 1 Migration Log"),
            "Log must have header"
        );

        // Property: Log file must have Test Migrations section
        prop_assert!(
            content.contains("## Test Migrations"),
            "Log must have Test Migrations section"
        );
    });
}

// Property: Logging should be idempotent for initialization
#[test]
fn prop_initialization_idempotent() {
    proptest!(|(n in 1usize..10)| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Initialize multiple times
        for _ in 0..n {
            logger.initialize().unwrap();
        }

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: Header should appear exactly once
        let header_count = content.matches("# Phase 1 Migration Log").count();
        prop_assert_eq!(
            header_count,
            1,
            "Header should appear exactly once, found {}",
            header_count
        );
    });
}

// Property: Log entries should preserve order
#[test]
fn prop_log_entries_preserve_order() {
    proptest!(|(results in prop::collection::vec(arb_migration_result(), 2..10))| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log all migrations
        for result in &results {
            logger.log_migration(result).unwrap();
        }

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: Source paths should appear in the same order they were logged
        let mut last_pos = 0;
        for result in &results {
            let source_str = result.source_path.to_string_lossy().to_string();
            if let Some(pos) = content[last_pos..].find(&source_str) {
                last_pos += pos + source_str.len();
            } else {
                prop_assert!(false, "Source path not found in order: {:?}", result.source_path);
            }
        }
    });
}

use repo_hygiene::{
    CleanupReport, DomainLogicType, DomainLogicViolation, HostBypassPattern, RegistrationBlob,
};

// Generator for arbitrary HostBypassPattern
fn arb_host_bypass() -> impl Strategy<Value = HostBypassPattern> {
    (
        prop::string::string_regex("src/[a-z_/]+\\.rs").unwrap(),
        1usize..1000,
        prop::string::string_regex("self\\.host\\.[a-z_]+\\(\\)").unwrap(),
        prop::string::string_regex("Route through [A-Z][a-z_]+").unwrap(),
    )
        .prop_map(|(file, line, pattern, action)| HostBypassPattern {
            file: PathBuf::from(file),
            line,
            pattern,
            suggested_action: action,
        })
}

// Generator for arbitrary RegistrationBlob
fn arb_registration_blob() -> impl Strategy<Value = RegistrationBlob> {
    (
        prop::string::string_regex("src/[a-z_/]+\\.rs").unwrap(),
        200usize..1000,
        prop::collection::vec(prop::string::string_regex("[A-Z][a-z]+").unwrap(), 0..5),
        prop::collection::vec(prop::string::string_regex("Split [a-z]+").unwrap(), 0..5),
    )
        .prop_map(
            |(file, line_count, concerns, decomposition)| RegistrationBlob {
                file: PathBuf::from(file),
                line_count,
                mixed_concerns: concerns,
                suggested_decomposition: decomposition,
            },
        )
}

// Generator for arbitrary DomainLogicViolation
fn arb_domain_logic_violation() -> impl Strategy<Value = DomainLogicViolation> {
    (
        prop::string::string_regex("src/[a-z_/]+\\.rs").unwrap(),
        prop::sample::select(vec![
            DomainLogicType::Parser,
            DomainLogicType::Validator,
            DomainLogicType::BusinessRule,
        ]),
        1usize..1000,
        1usize..100,
        prop::string::string_regex("[A-Z][a-z]+ Layer").unwrap(),
    )
        .prop_map(|(file, vtype, start, len, layer)| DomainLogicViolation {
            file: PathBuf::from(file),
            violation_type: vtype,
            line_range: (start, start + len),
            suggested_target_layer: layer,
        })
}

// Generator for arbitrary CleanupReport
fn arb_cleanup_report() -> impl Strategy<Value = CleanupReport> {
    (
        prop::collection::vec(arb_host_bypass(), 0..5),
        prop::collection::vec(arb_registration_blob(), 0..5),
        prop::collection::vec(arb_domain_logic_violation(), 0..5),
    )
        .prop_map(|(bypasses, blobs, violations)| CleanupReport::new(bypasses, blobs, violations))
}

// **Validates: Requirements 11.3-11.5**
// Property 29: Migration Log Includes All Operations
//
// For any cleanup operation (host bypass identification, registration blob decomposition,
// domain logic extraction), the migration log should contain an entry describing the
// operation and its status.
#[test]
fn prop_cleanup_log_includes_all_operations() {
    proptest!(|(report in arb_cleanup_report())| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log the cleanup report
        logger.log_cleanup(&report).unwrap();

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: Log must contain Cleanup Operations section
        prop_assert!(
            content.contains("## Cleanup Operations"),
            "Log must contain Cleanup Operations section"
        );

        // Property: All host bypasses must be logged
        if !report.host_bypasses.is_empty() {
            prop_assert!(
                content.contains("Host Bypasses Identified"),
                "Log must contain Host Bypasses section"
            );

            for bypass in &report.host_bypasses {
                // Check that the file and line are in the log
                let file_str = bypass.file.to_string_lossy().to_string();
                prop_assert!(
                    content.contains(&file_str),
                    "Log must contain host bypass file: {}",
                    file_str
                );
                prop_assert!(
                    content.contains(&bypass.pattern),
                    "Log must contain host bypass pattern: {}",
                    bypass.pattern
                );
            }
        }

        // Property: All registration blobs must be logged
        if !report.registration_blobs.is_empty() {
            prop_assert!(
                content.contains("Registration Blobs Identified"),
                "Log must contain Registration Blobs section"
            );

            for blob in &report.registration_blobs {
                let file_str = blob.file.to_string_lossy().to_string();
                prop_assert!(
                    content.contains(&file_str),
                    "Log must contain registration blob file: {}",
                    file_str
                );
                prop_assert!(
                    content.contains(&blob.line_count.to_string()),
                    "Log must contain registration blob line count: {}",
                    blob.line_count
                );
            }
        }

        // Property: All domain logic violations must be logged
        if !report.domain_logic_violations.is_empty() {
            prop_assert!(
                content.contains("Domain Logic Violations Identified"),
                "Log must contain Domain Logic Violations section"
            );

            for violation in &report.domain_logic_violations {
                let file_str = violation.file.to_string_lossy().to_string();
                prop_assert!(
                    content.contains(&file_str),
                    "Log must contain domain logic violation file: {}",
                    file_str
                );
            }
        }

        // Property: Total issues count must be in the log
        prop_assert!(
            content.contains(&format!("**Total Issues**: {}", report.total_issues)),
            "Log must contain total issues count: {}",
            report.total_issues
        );
    });
}

// Property: Multiple cleanup reports should all be logged
#[test]
fn prop_multiple_cleanup_reports_all_logged() {
    proptest!(|(reports in prop::collection::vec(arb_cleanup_report(), 1..5))| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log all cleanup reports
        for report in &reports {
            logger.log_cleanup(report).unwrap();
        }

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: Cleanup Operations section should appear exactly once
        let section_count = content.matches("## Cleanup Operations").count();
        prop_assert_eq!(
            section_count,
            1,
            "Cleanup Operations section should appear exactly once, found {}",
            section_count
        );

        // Property: All reports should be logged
        let cleanup_report_count = content.matches("### Cleanup Report").count();
        prop_assert_eq!(
            cleanup_report_count,
            reports.len(),
            "Should have {} cleanup reports, found {}",
            reports.len(),
            cleanup_report_count
        );
    });
}

// Property: Cleanup log should include remediation status
#[test]
fn prop_cleanup_log_includes_remediation_status() {
    proptest!(|(report in arb_cleanup_report())| {
        let temp_dir = TempDir::new().unwrap();
        let logger = MigrationLogger::new(temp_dir.path());

        // Log the cleanup report
        logger.log_cleanup(&report).unwrap();

        // Read the log file
        let content = fs::read_to_string(logger.log_path()).unwrap();

        // Property: All logged items should have a status
        if !report.host_bypasses.is_empty()
            || !report.registration_blobs.is_empty()
            || !report.domain_logic_violations.is_empty()
        {
            prop_assert!(
                content.contains("Flagged for manual refactoring"),
                "Log must contain remediation status"
            );
        }
    });
}
