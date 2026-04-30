//! Runtime mode service

use crate::*;

pub struct RuntimeModeService {
    world_state: std::sync::Arc<std::sync::Mutex<WorldState>>,
    _diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
    event_bus: std::sync::Arc<dyn EventBus>,
}

impl RuntimeModeService {
    pub fn new(
        world_state: std::sync::Arc<std::sync::Mutex<WorldState>>,
        diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
        event_bus: std::sync::Arc<dyn EventBus>,
    ) -> Self {
        Self {
            world_state,
            _diagnostics_state: diagnostics_state,
            event_bus,
        }
    }

    pub fn enter_preview_mode(&mut self) -> Result<(), String> {
        let mut world_state = self.world_state.lock().unwrap();
        world_state.runtime_mode.is_preview_mode = true;
        world_state.runtime_mode.is_simulation_running = false;
        self.event_bus.publish("runtime.preview.entered");
        Ok(())
    }
}
