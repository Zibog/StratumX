//! Diagnostics query views
//!
//! Read-only views of DiagnosticsOwner state implementing ReadModel trait.

use crate::owners::diagnostics_owner::{DiagnosticsOwner, FailureCode, Severity};
use crate::queries::ReadModel;

/// Diagnostics summary view
///
/// Read-only summary of diagnostic messages by severity.
#[derive(Debug, Clone)]
pub struct DiagnosticsSummaryView {
    pub total_messages: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub critical_count: usize,
    pub has_errors: bool,
    pub has_critical: bool,
}

impl ReadModel<DiagnosticsOwner, DiagnosticsSummaryView> for DiagnosticsSummaryView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        let error_count = owner.get_messages_by_severity(Severity::Error).len();
        let warning_count = owner.get_messages_by_severity(Severity::Warning).len();
        let info_count = owner.get_messages_by_severity(Severity::Info).len();
        let critical_count = owner.get_messages_by_severity(Severity::Critical).len();

        Self {
            total_messages: owner.messages.len(),
            error_count,
            warning_count,
            info_count,
            critical_count,
            has_errors: error_count > 0,
            has_critical: critical_count > 0,
        }
    }
}

/// Diagnostics messages view
///
/// Read-only view of all diagnostic messages.
#[derive(Debug, Clone)]
pub struct DiagnosticsMessagesView {
    pub messages: Vec<DiagnosticMessageInfo>,
}

impl ReadModel<DiagnosticsOwner, DiagnosticsMessagesView> for DiagnosticsMessagesView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        let messages = owner
            .messages
            .iter()
            .map(|m| DiagnosticMessageInfo {
                severity: m.severity,
                message: m.message.clone(),
                source: format!("{:?}", m.source),
                timestamp: m.timestamp,
                has_trace: m.trace_id.is_some(),
            })
            .collect();

        Self { messages }
    }
}

/// Diagnostic message information (simplified view)
#[derive(Debug, Clone)]
pub struct DiagnosticMessageInfo {
    pub severity: Severity,
    pub message: String,
    pub source: String,
    pub timestamp: u64,
    pub has_trace: bool,
}

/// Diagnostics failure codes view
///
/// Read-only view of failure codes and recovery actions.
#[derive(Debug, Clone)]
pub struct DiagnosticsFailureCodesView {
    pub failure_codes: Vec<FailureCode>,
    pub failure_count: usize,
    pub recovery_actions_available: usize,
}

impl ReadModel<DiagnosticsOwner, DiagnosticsFailureCodesView> for DiagnosticsFailureCodesView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        Self {
            failure_codes: owner.failure_codes.clone(),
            failure_count: owner.failure_codes.len(),
            recovery_actions_available: owner.recovery_actions.len(),
        }
    }
}

/// Diagnostics trace lineage view
///
/// Read-only view of trace lineage information.
#[derive(Debug, Clone)]
pub struct DiagnosticsTraceLineageView {
    pub trace_count: usize,
    pub traces_with_parents: usize,
}

impl ReadModel<DiagnosticsOwner, DiagnosticsTraceLineageView> for DiagnosticsTraceLineageView {
    fn build(owner: &DiagnosticsOwner) -> Self {
        let traces_with_parents = owner
            .trace_lineage
            .values()
            .filter(|lineage| lineage.parent_trace_id.is_some())
            .count();

        Self {
            trace_count: owner.trace_lineage.len(),
            traces_with_parents,
        }
    }
}

