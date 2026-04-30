//! Diagnostics owner types

use crate::{DiagnosticMessage, FailureCode, TraceId, TraceLineage};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct DiagnosticsOwner {
    pub diagnostics: Vec<DiagnosticMessage>,
    pub messages: Vec<DiagnosticMessage>,
    pub failure_codes: Vec<FailureCode>,
    pub trace_lineage: HashMap<TraceId, TraceLineage>,
}

impl DiagnosticsOwner {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            messages: Vec::new(),
            failure_codes: Vec::new(),
            trace_lineage: HashMap::new(),
        }
    }

    pub fn add_diagnostic(&mut self, msg: DiagnosticMessage) {
        self.diagnostics.push(msg.clone());
        self.messages.push(msg);
    }

    pub fn add_trace_lineage(&mut self, trace_id: TraceId, lineage: TraceLineage) {
        self.trace_lineage.insert(trace_id, lineage);
    }

    pub fn add_message(&mut self, msg: DiagnosticMessage) {
        self.add_diagnostic(msg);
    }

    pub fn add_failure_code(&mut self, code: FailureCode) {
        self.failure_codes.push(code);
    }
}
