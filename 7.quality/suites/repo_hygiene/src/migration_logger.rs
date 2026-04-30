// Migration Logger Module
//
// This module handles logging of all migration and cleanup operations to a markdown file.
// It provides a comprehensive audit trail of Phase 1 sanitization activities.

use crate::models::{
    CleanupReport, CompilationStatus, DomainLogicType, DomainLogicViolation, HostBypassPattern,
    MigrationResult, RegistrationBlob,
};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Logger for migration and cleanup operations
pub struct MigrationLogger {
    /// Path to the migration log file
    log_path: PathBuf,
}

impl MigrationLogger {
    /// Create a new migration logger
    ///
    /// The log file will be created at `7.quality/docs/phase1_migration_log.md`
    pub fn new(repo_root: &Path) -> Self {
        let log_path = repo_root.join("7.quality/docs/phase1_migration_log.md");
        Self { log_path }
    }

    /// Initialize the log file with header if it doesn't exist
    pub fn initialize(&self) -> Result<(), io::Error> {
        // Create the docs directory if it doesn't exist
        if let Some(parent) = self.log_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // If the log file doesn't exist, create it with a header
        if !self.log_path.exists() {
            let header = "# Phase 1 Migration Log\n\n\
                         This log documents all test migrations and cleanup operations performed during Phase 1 of repository sanitization.\n\n\
                         ## Test Migrations\n\n";
            fs::write(&self.log_path, header)?;
        }

        Ok(())
    }

    /// Log a test migration operation
    pub fn log_migration(&self, result: &MigrationResult) -> Result<(), io::Error> {
        self.initialize()?;

        let timestamp = Self::format_timestamp(SystemTime::now());
        let status = Self::format_compilation_status(&result.compilation_status);

        let mut entry = format!("\n### {}\n\n", timestamp);
        entry.push_str(&format!("**Source**: `{}`\n", result.source_path.display()));
        entry.push_str(&format!(
            "**Destination**: `{}`\n",
            result.destination_path.display()
        ));
        entry.push_str(&format!(
            "**Imports Updated**: {}\n",
            result.imports_updated
        ));

        if result.regression_files_moved.is_empty() {
            entry.push_str("**Regression Files**: None\n");
        } else {
            entry.push_str("**Regression Files**: ");
            for (i, file) in result.regression_files_moved.iter().enumerate() {
                if i > 0 {
                    entry.push_str(", ");
                }
                entry.push_str(&format!("`{}`", file.display()));
            }
            entry.push('\n');
        }

        entry.push_str(&format!("**Status**: {}\n", status));

        self.append_to_log(&entry)
    }

    /// Log cleanup operations
    pub fn log_cleanup(&self, report: &CleanupReport) -> Result<(), io::Error> {
        self.initialize()?;

        // Check if we need to add the cleanup operations section header
        let content = fs::read_to_string(&self.log_path)?;
        if !content.contains("## Cleanup Operations") {
            let section_header = "\n## Cleanup Operations\n\n";
            self.append_to_log(section_header)?;
        }

        let timestamp = Self::format_timestamp(SystemTime::now());
        let mut entry = format!("\n### Cleanup Report - {}\n\n", timestamp);

        // Log host bypasses
        if !report.host_bypasses.is_empty() {
            entry.push_str("#### Host Bypasses Identified\n\n");
            for (i, bypass) in report.host_bypasses.iter().enumerate() {
                entry.push_str(&format!("{}. ", i + 1));
                entry.push_str(&Self::format_host_bypass(bypass));
                entry.push('\n');
            }
            entry.push('\n');
        }

        // Log registration blobs
        if !report.registration_blobs.is_empty() {
            entry.push_str("#### Registration Blobs Identified\n\n");
            for (i, blob) in report.registration_blobs.iter().enumerate() {
                entry.push_str(&format!("{}. ", i + 1));
                entry.push_str(&Self::format_registration_blob(blob));
                entry.push('\n');
            }
            entry.push('\n');
        }

        // Log domain logic violations
        if !report.domain_logic_violations.is_empty() {
            entry.push_str("#### Domain Logic Violations Identified\n\n");
            for (i, violation) in report.domain_logic_violations.iter().enumerate() {
                entry.push_str(&format!("{}. ", i + 1));
                entry.push_str(&Self::format_domain_logic_violation(violation));
                entry.push('\n');
            }
            entry.push('\n');
        }

        entry.push_str(&format!("**Total Issues**: {}\n", report.total_issues));

        self.append_to_log(&entry)
    }

    /// Append content to the log file
    fn append_to_log(&self, content: &str) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        file.write_all(content.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    /// Format a timestamp for display
    fn format_timestamp(time: SystemTime) -> String {
        // Convert to a human-readable format
        // For simplicity, we'll use the debug format
        // In production, you might want to use chrono or time crate
        match time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Self::unix_to_datetime(duration.as_secs()),
            Err(_) => "Unknown Time".to_string(),
        }
    }

    /// Convert Unix timestamp to ISO 8601 datetime string
    fn unix_to_datetime(secs: u64) -> String {
        // Simple conversion without external dependencies
        // This is a basic implementation; for production use chrono
        let days_since_epoch = secs / 86400;
        let remaining_secs = secs % 86400;
        let hours = remaining_secs / 3600;
        let minutes = (remaining_secs % 3600) / 60;
        let seconds = remaining_secs % 60;

        // Calculate year, month, day (simplified, doesn't account for leap years perfectly)
        let mut year = 1970;
        let mut days = days_since_epoch;

        // Rough approximation
        while days >= 365 {
            if Self::is_leap_year(year) && days >= 366 {
                days -= 366;
                year += 1;
            } else if !Self::is_leap_year(year) && days >= 365 {
                days -= 365;
                year += 1;
            } else {
                break;
            }
        }

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            year,
            1 + (days / 30).min(11), // Rough month approximation
            1 + (days % 30),
            hours,
            minutes,
            seconds
        )
    }

    /// Check if a year is a leap year
    fn is_leap_year(year: u64) -> bool {
        (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
    }

    /// Format compilation status for display
    fn format_compilation_status(status: &CompilationStatus) -> &'static str {
        match status {
            CompilationStatus::Success => "Success",
            CompilationStatus::Failed => "Failed",
            CompilationStatus::NotAttempted => "Not Attempted",
        }
    }

    /// Format a host bypass pattern for display
    fn format_host_bypass(bypass: &HostBypassPattern) -> String {
        format!(
            "**File**: `{}:{}`\n   **Pattern**: `{}`\n   **Remediation**: {}\n   **Status**: Flagged for manual refactoring",
            bypass.file.display(),
            bypass.line,
            bypass.pattern,
            bypass.suggested_action
        )
    }

    /// Format a registration blob for display
    fn format_registration_blob(blob: &RegistrationBlob) -> String {
        let mut result = format!(
            "**File**: `{}`\n   **Line Count**: {}\n",
            blob.file.display(),
            blob.line_count
        );

        if !blob.mixed_concerns.is_empty() {
            result.push_str("   **Mixed Concerns**: ");
            for (i, concern) in blob.mixed_concerns.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(concern);
            }
            result.push('\n');
        }

        if !blob.suggested_decomposition.is_empty() {
            result.push_str("   **Suggested Decomposition**:\n");
            for suggestion in &blob.suggested_decomposition {
                result.push_str(&format!("   - {}\n", suggestion));
            }
        }

        result.push_str("   **Status**: Flagged for manual refactoring");
        result
    }

    /// Format a domain logic violation for display
    fn format_domain_logic_violation(violation: &DomainLogicViolation) -> String {
        let violation_type = match violation.violation_type {
            DomainLogicType::Parser => "Parser",
            DomainLogicType::Validator => "Validator",
            DomainLogicType::BusinessRule => "Business Rule",
        };

        format!(
            "**File**: `{}:{}-{}`\n   **Type**: {}\n   **Suggested Target Layer**: {}\n   **Status**: Flagged for manual refactoring",
            violation.file.display(),
            violation.line_range.0,
            violation.line_range.1,
            violation_type,
            violation.suggested_target_layer
        )
    }

    /// Get the path to the log file
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_format_compilation_status() {
        assert_eq!(
            MigrationLogger::format_compilation_status(&CompilationStatus::Success),
            "Success"
        );
        assert_eq!(
            MigrationLogger::format_compilation_status(&CompilationStatus::Failed),
            "Failed"
        );
        assert_eq!(
            MigrationLogger::format_compilation_status(&CompilationStatus::NotAttempted),
            "Not Attempted"
        );
    }

    #[test]
    fn test_is_leap_year() {
        assert!(MigrationLogger::is_leap_year(2000));
        assert!(MigrationLogger::is_leap_year(2004));
        assert!(!MigrationLogger::is_leap_year(1900));
        assert!(!MigrationLogger::is_leap_year(2001));
    }

    #[test]
    fn test_format_host_bypass() {
        let bypass = HostBypassPattern {
            file: PathBuf::from("src/app.rs"),
            line: 42,
            pattern: "self.host.save_file()".to_string(),
            suggested_action: "Route through Command_Spine".to_string(),
        };

        let formatted = MigrationLogger::format_host_bypass(&bypass);
        assert!(formatted.contains("src/app.rs:42"));
        assert!(formatted.contains("self.host.save_file()"));
        assert!(formatted.contains("Route through Command_Spine"));
    }

    #[test]
    fn test_format_registration_blob() {
        let blob = RegistrationBlob {
            file: PathBuf::from("src/registration.rs"),
            line_count: 250,
            mixed_concerns: vec!["UI".to_string(), "Logic".to_string()],
            suggested_decomposition: vec!["Split UI".to_string(), "Extract Logic".to_string()],
        };

        let formatted = MigrationLogger::format_registration_blob(&blob);
        assert!(formatted.contains("src/registration.rs"));
        assert!(formatted.contains("250"));
        assert!(formatted.contains("UI"));
        assert!(formatted.contains("Logic"));
    }

    #[test]
    fn test_format_domain_logic_violation() {
        let violation = DomainLogicViolation {
            file: PathBuf::from("src/panel.rs"),
            violation_type: DomainLogicType::Parser,
            line_range: (10, 20),
            suggested_target_layer: "Service Layer".to_string(),
        };

        let formatted = MigrationLogger::format_domain_logic_violation(&violation);
        assert!(formatted.contains("src/panel.rs:10-20"));
        assert!(formatted.contains("Parser"));
        assert!(formatted.contains("Service Layer"));
    }
}
