//! Diagnostics mutations — methods that change diagnostics state.

use super::diagnostics_owner::DiagnosticsOwner;
use crate::model::diagnostics_types::{DiagnosticMessage, FailureCode, TraceId, TraceLineage};

impl DiagnosticsOwner {
    /// Adds a diagnostic message
    pub fn add_message(&mut self, message: DiagnosticMessage) {
        self.messages.push(message);
    }

    /// Clears all diagnostic messages
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    /// Adds trace lineage
    pub fn add_trace_lineage(&mut self, trace_id: TraceId, lineage: TraceLineage) {
        self.trace_lineage.insert(trace_id, lineage);
    }

    /// Adds a failure code
    pub fn add_failure_code(&mut self, code: FailureCode) {
        if !self.failure_codes.contains(&code) {
            self.failure_codes.push(code);
        }
    }

    /// Registers a recovery action for a failure code
    pub fn register_recovery_action(&mut self, code: FailureCode, action_id: String) {
        self.recovery_actions.insert(code, action_id);
    }
}
