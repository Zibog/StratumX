//! Diagnostics service

use crate::*;

pub struct DiagnosticsService {
    diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
    event_bus: std::sync::Arc<dyn EventBus>,
}

impl DiagnosticsService {
    pub fn new(
        diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
        event_bus: std::sync::Arc<dyn EventBus>,
    ) -> Self {
        Self {
            diagnostics_state,
            event_bus,
        }
    }

    pub fn add_diagnostic(&mut self, message: DiagnosticMessage) -> Result<(), String> {
        self.diagnostics_state.lock().unwrap().add_message(message);
        self.event_bus.publish("diagnostics.message_added");
        Ok(())
    }
}
