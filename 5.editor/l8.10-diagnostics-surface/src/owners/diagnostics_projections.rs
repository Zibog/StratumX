//! Diagnostics projections — derived views and summaries.

use super::diagnostics_owner::DiagnosticsOwner;
use crate::model::diagnostics_types::{
    DiagnosticMessage, FailureCode, Severity, TraceId, TraceLineage,
};

/// Summary of diagnostics state
#[derive(Debug, Clone)]
pub struct DiagnosticsSummary {
    /// Count of messages by severity
    pub info_count: usize,
    /// Count of warning messages
    pub warning_count: usize,
    /// Count of error messages
    pub error_count: usize,
    /// Count of critical messages
    pub critical_count: usize,
    /// Total failure codes
    pub failure_code_count: usize,
    /// Total trace lineages
    pub trace_lineage_count: usize,
}

impl DiagnosticsOwner {
    /// Gets messages by severity
    pub fn get_messages_by_severity(&self, severity: Severity) -> Vec<&DiagnosticMessage> {
        self.messages
            .iter()
            .filter(|m| m.severity == severity)
            .collect()
    }

    /// Gets a summary of diagnostics state
    pub fn get_summary(&self) -> DiagnosticsSummary {
        let mut info_count = 0;
        let mut warning_count = 0;
        let mut error_count = 0;
        let mut critical_count = 0;

        for msg in &self.messages {
            match msg.severity {
                Severity::Info => info_count += 1,
                Severity::Warning => warning_count += 1,
                Severity::Error => error_count += 1,
                Severity::Critical => critical_count += 1,
            }
        }

        DiagnosticsSummary {
            info_count,
            warning_count,
            error_count,
            critical_count,
            failure_code_count: self.failure_codes.len(),
            trace_lineage_count: self.trace_lineage.len(),
        }
    }

    /// Gets all failure codes
    pub fn get_failure_codes(&self) -> &[FailureCode] {
        &self.failure_codes
    }

    /// Gets recovery action for a failure code
    pub fn get_recovery_action(&self, code: &FailureCode) -> Option<&String> {
        self.recovery_actions.get(code)
    }

    /// Gets trace lineage by ID
    pub fn get_trace_lineage(&self, trace_id: &TraceId) -> Option<&TraceLineage> {
        self.trace_lineage.get(trace_id)
    }
}
