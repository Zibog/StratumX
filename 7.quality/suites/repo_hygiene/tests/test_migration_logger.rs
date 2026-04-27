// Unit tests for migration logger

use repo_hygiene::{
    CleanupReport, CompilationStatus, DomainLogicType, DomainLogicViolation, HostBypassPattern,
    MigrationLogger, MigrationResult, RegistrationBlob,
};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_log_migration_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    assert!(logger.log_path().exists());
}

#[test]
fn test_log_migration_includes_source_path() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("src/test.rs"));
}

#[test]
fn test_log_migration_includes_destination_path() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("7.quality/suites/test.rs"));
}

#[test]
fn test_log_migration_includes_timestamp() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    // Check for timestamp format (year should be present)
    assert!(content.contains("UTC"));
}

#[test]
fn test_log_migration_includes_imports_updated() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 5,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("**Imports Updated**: 5"));
}

#[test]
fn test_log_migration_includes_regression_files() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![
            PathBuf::from("proptest-regressions/test.txt"),
            PathBuf::from("proptest-regressions/test2.txt"),
        ],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("proptest-regressions/test.txt"));
    assert!(content.contains("proptest-regressions/test2.txt"));
}

#[test]
fn test_log_migration_no_regression_files() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("**Regression Files**: None"));
}

#[test]
fn test_log_migration_includes_compilation_status_success() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("**Status**: Success"));
}

#[test]
fn test_log_migration_includes_compilation_status_failed() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result = MigrationResult {
        source_path: PathBuf::from("src/test.rs"),
        destination_path: PathBuf::from("7.quality/suites/test.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Failed,
    };

    logger.log_migration(&result).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("**Status**: Failed"));
}

#[test]
fn test_log_migration_appends_multiple_entries() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let result1 = MigrationResult {
        source_path: PathBuf::from("src/test1.rs"),
        destination_path: PathBuf::from("7.quality/suites/test1.rs"),
        imports_updated: 3,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    let result2 = MigrationResult {
        source_path: PathBuf::from("src/test2.rs"),
        destination_path: PathBuf::from("7.quality/suites/test2.rs"),
        imports_updated: 5,
        regression_files_moved: vec![],
        compilation_status: CompilationStatus::Success,
    };

    logger.log_migration(&result1).unwrap();
    logger.log_migration(&result2).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("src/test1.rs"));
    assert!(content.contains("src/test2.rs"));
}

#[test]
fn test_log_cleanup_creates_cleanup_section() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let report = CleanupReport::new(vec![], vec![], vec![]);

    logger.log_cleanup(&report).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("## Cleanup Operations"));
}

#[test]
fn test_log_cleanup_includes_host_bypasses() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let bypass = HostBypassPattern {
        file: PathBuf::from("src/app.rs"),
        line: 42,
        pattern: "self.host.save_file()".to_string(),
        suggested_action: "Route through Command_Spine".to_string(),
    };

    let report = CleanupReport::new(vec![bypass], vec![], vec![]);

    logger.log_cleanup(&report).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("Host Bypasses Identified"));
    assert!(content.contains("src/app.rs:42"));
    assert!(content.contains("self.host.save_file()"));
}

#[test]
fn test_log_cleanup_includes_registration_blobs() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let blob = RegistrationBlob {
        file: PathBuf::from("src/registration.rs"),
        line_count: 250,
        mixed_concerns: vec!["UI".to_string(), "Logic".to_string()],
        suggested_decomposition: vec!["Split UI".to_string()],
    };

    let report = CleanupReport::new(vec![], vec![blob], vec![]);

    logger.log_cleanup(&report).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("Registration Blobs Identified"));
    assert!(content.contains("src/registration.rs"));
    assert!(content.contains("250"));
}

#[test]
fn test_log_cleanup_includes_domain_logic_violations() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let violation = DomainLogicViolation {
        file: PathBuf::from("src/panel.rs"),
        violation_type: DomainLogicType::Parser,
        line_range: (10, 20),
        suggested_target_layer: "Service Layer".to_string(),
    };

    let report = CleanupReport::new(vec![], vec![], vec![violation]);

    logger.log_cleanup(&report).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("Domain Logic Violations Identified"));
    assert!(content.contains("src/panel.rs:10-20"));
    assert!(content.contains("Parser"));
}

#[test]
fn test_log_cleanup_includes_total_issues() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    let bypass = HostBypassPattern {
        file: PathBuf::from("src/app.rs"),
        line: 42,
        pattern: "self.host.save_file()".to_string(),
        suggested_action: "Route through Command_Spine".to_string(),
    };

    let blob = RegistrationBlob {
        file: PathBuf::from("src/registration.rs"),
        line_count: 250,
        mixed_concerns: vec![],
        suggested_decomposition: vec![],
    };

    let report = CleanupReport::new(vec![bypass], vec![blob], vec![]);

    logger.log_cleanup(&report).unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("**Total Issues**: 2"));
}

#[test]
fn test_initialize_creates_directory() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    logger.initialize().unwrap();

    assert!(logger.log_path().parent().unwrap().exists());
}

#[test]
fn test_initialize_creates_header() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    logger.initialize().unwrap();

    let content = fs::read_to_string(logger.log_path()).unwrap();
    assert!(content.contains("# Phase 1 Migration Log"));
    assert!(content.contains("## Test Migrations"));
}

#[test]
fn test_initialize_idempotent() {
    let temp_dir = TempDir::new().unwrap();
    let logger = MigrationLogger::new(temp_dir.path());

    logger.initialize().unwrap();
    let content1 = fs::read_to_string(logger.log_path()).unwrap();

    logger.initialize().unwrap();
    let content2 = fs::read_to_string(logger.log_path()).unwrap();

    // Should not duplicate the header
    assert_eq!(content1, content2);
}
