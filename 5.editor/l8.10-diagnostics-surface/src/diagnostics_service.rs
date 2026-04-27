//! Diagnostics Service
//!
//! Domain service for diagnostic collection, trace lineage, and failure code mapping.
//! Abstraction Level: L2 (Infrastructure)

use crate::{
    DiagnosticMessage, DiagnosticsState, EditorEvent, EventBus, FailureCode, Severity, TraceId,
    TraceLineage,
};
use std::sync::{Arc, Mutex};

/// Diagnostics service for diagnostic management operations
pub struct DiagnosticsService {
    diagnostics_state: Arc<Mutex<DiagnosticsState>>,
    event_bus: Arc<dyn EventBus>,
}

impl DiagnosticsService {
    /// Create a new diagnostics service
    pub fn new(
        diagnostics_state: Arc<Mutex<DiagnosticsState>>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            diagnostics_state,
            event_bus,
        }
    }

    /// Add a diagnostic message
    pub fn add_diagnostic(&mut self, message: DiagnosticMessage) -> Result<(), String> {
        let severity = message.severity;
        let message_text = message.message.clone();

        // Add to diagnostics state
        {
            let mut diagnostics = self.diagnostics_state.lock().unwrap();
            diagnostics.messages.push(message);
        }

        // Emit event
        self.event_bus.emit(EditorEvent::DiagnosticAdded {
            severity,
            message: message_text,
        });

        Ok(())
    }

    /// Clear all diagnostics
    pub fn clear_diagnostics(&mut self) -> Result<(), String> {
        // Clear diagnostics state
        {
            let mut diagnostics = self.diagnostics_state.lock().unwrap();
            diagnostics.messages.clear();
        }

        // Emit event
        self.event_bus.emit(EditorEvent::DiagnosticsCleared);

        Ok(())
    }

    /// Add trace lineage for error tracking
    pub fn add_trace_lineage(
        &mut self,
        trace_id: TraceId,
        lineage: TraceLineage,
    ) -> Result<(), String> {
        let mut diagnostics = self.diagnostics_state.lock().unwrap();
        diagnostics.trace_lineage.insert(trace_id, lineage);

        Ok(())
    }

    /// Get trace lineage by ID
    pub fn get_trace_lineage(&self, trace_id: &TraceId) -> Option<TraceLineage> {
        let diagnostics = self.diagnostics_state.lock().unwrap();
        diagnostics.trace_lineage.get(trace_id).cloned()
    }

    /// Map a failure code to a recovery action
    pub fn map_failure_code(
        &mut self,
        failure_code: FailureCode,
        recovery_action: String,
    ) -> Result<(), String> {
        let mut diagnostics = self.diagnostics_state.lock().unwrap();
        diagnostics
            .recovery_actions
            .insert(failure_code, recovery_action);

        Ok(())
    }

    /// Get recovery action for a failure code
    pub fn get_recovery_action(&self, failure_code: &FailureCode) -> Option<String> {
        let diagnostics = self.diagnostics_state.lock().unwrap();
        diagnostics.recovery_actions.get(failure_code).cloned()
    }

    /// Get all diagnostics by severity
    pub fn get_diagnostics_by_severity(&self, severity: Severity) -> Vec<DiagnosticMessage> {
        let diagnostics = self.diagnostics_state.lock().unwrap();
        diagnostics
            .messages
            .iter()
            .filter(|msg| msg.severity == severity)
            .cloned()
            .collect()
    }

    /// Get all failure codes
    pub fn get_failure_codes(&self) -> Vec<FailureCode> {
        let diagnostics = self.diagnostics_state.lock().unwrap();
        diagnostics.failure_codes.clone()
    }
}

