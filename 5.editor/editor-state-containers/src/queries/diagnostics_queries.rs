//! Diagnostics query types

use super::ReadModel;
use crate::owners::diagnostics_owner::DiagnosticsOwner;

#[derive(Debug, Clone)]
pub struct DiagnosticsSummaryView {
    pub error_count: usize,
    pub warning_count: usize,
    pub total_messages: usize,
}

impl ReadModel<DiagnosticsOwner> for DiagnosticsSummaryView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        let error_count = owner
            .diagnostics
            .iter()
            .filter(|d| matches!(d.severity, crate::Severity::Error))
            .count();
        let warning_count = owner
            .diagnostics
            .iter()
            .filter(|d| matches!(d.severity, crate::Severity::Warning))
            .count();
        Self {
            error_count,
            warning_count,
            total_messages: error_count + warning_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticsMessagesView {
    pub messages: Vec<crate::DiagnosticMessage>,
}

impl ReadModel<DiagnosticsOwner> for DiagnosticsMessagesView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        Self {
            messages: owner.messages.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticsFailureCodesView {
    pub failure_count: usize,
    pub failure_codes: Vec<crate::FailureCode>,
}

impl ReadModel<DiagnosticsOwner> for DiagnosticsFailureCodesView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        Self {
            failure_count: owner.failure_codes.len(),
            failure_codes: owner.failure_codes.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticsTraceLineageView {
    pub trace_count: usize,
    pub traces: Vec<crate::TraceLineage>,
}

impl ReadModel<DiagnosticsOwner> for DiagnosticsTraceLineageView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        Self {
            trace_count: owner.trace_lineage.len(),
            traces: owner.trace_lineage.values().cloned().collect(),
        }
    }
}
