//! Diagnostics state type

use crate::*;

#[derive(Debug, Clone)]
pub struct DiagnosticsState {
    pub diagnostics_owner: owners::diagnostics_owner::DiagnosticsOwner,
    pub messages: Vec<DiagnosticMessage>,
    pub failure_codes: Vec<FailureCode>,
}

impl DiagnosticsState {
    pub fn new() -> Self {
        Self {
            diagnostics_owner: owners::diagnostics_owner::DiagnosticsOwner::new(),
            messages: Vec::new(),
            failure_codes: Vec::new(),
        }
    }

    pub fn add_message(&mut self, msg: DiagnosticMessage) {
        self.diagnostics_owner.add_message(msg.clone());
        self.messages.push(msg);
    }

    pub fn get_messages_by_severity(&self, severity: Severity) -> Vec<DiagnosticMessage> {
        self.messages
            .iter()
            .filter(|m| m.severity == severity)
            .cloned()
            .collect()
    }

    pub fn get_failure_codes(&self) -> Vec<FailureCode> {
        self.failure_codes.clone()
    }
}

impl Default for DiagnosticsState {
    fn default() -> Self {
        Self::new()
    }
}
