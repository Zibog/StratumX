//! Runtime Mode Service
//!
//! Domain service for runtime control including preview mode and simulation management.
//! Abstraction Level: L3 (Runtime Control)

use crate::{DiagnosticsState, EditorEvent, EventBus, WorldState};
use std::sync::{Arc, Mutex};

/// Runtime mode service for runtime control operations
pub struct RuntimeModeService {
    world_state: Arc<Mutex<WorldState>>,
    diagnostics_state: Arc<Mutex<DiagnosticsState>>,
    event_bus: Arc<dyn EventBus>,
}

impl RuntimeModeService {
    /// Create a new runtime mode service
    pub fn new(
        world_state: Arc<Mutex<WorldState>>,
        diagnostics_state: Arc<Mutex<DiagnosticsState>>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            world_state,
            diagnostics_state,
            event_bus,
        }
    }

    /// Enter preview mode
    pub fn enter_preview_mode(&mut self) -> Result<(), String> {
        let mut world = self.world_state.lock().unwrap();
        let runtime_mode = world.get_runtime_mode();

        if runtime_mode.is_preview_mode {
            return Err("Already in preview mode".to_string());
        }

        let mut new_mode = runtime_mode.clone();
        new_mode.is_preview_mode = true;
        world.set_runtime_mode(new_mode);

        // Emit event
        self.event_bus.emit(EditorEvent::PreviewModeEntered);

        Ok(())
    }

    /// Exit preview mode
    pub fn exit_preview_mode(&mut self) -> Result<(), String> {
        // Check if in preview mode and if simulation is running
        let (is_preview, is_sim_running) = {
            let world = self.world_state.lock().unwrap();
            let runtime_mode = world.get_runtime_mode();
            (
                runtime_mode.is_preview_mode,
                runtime_mode.is_simulation_running,
            )
        };

        if !is_preview {
            return Err("Not in preview mode".to_string());
        }

        // Stop simulation if running
        if is_sim_running {
            self.stop_simulation()?;
        }

        // Update runtime mode
        let mut world = self.world_state.lock().unwrap();
        let mut new_mode = world.get_runtime_mode().clone();
        new_mode.is_preview_mode = false;
        world.set_runtime_mode(new_mode);

        // Emit event
        self.event_bus.emit(EditorEvent::PreviewModeExited);

        Ok(())
    }

    /// Initialize runtime
    pub fn initialize_runtime(&mut self) -> Result<(), String> {
        // Validate world state
        {
            let world = self.world_state.lock().unwrap();
            if world.world_identity.world_id.is_nil() {
                return Err("No world loaded".to_string());
            }
        }

        // Emit event
        self.event_bus.emit(EditorEvent::RuntimeInitialized);

        Ok(())
    }

    /// Start simulation
    pub fn start_simulation(&mut self) -> Result<(), String> {
        let mut world = self.world_state.lock().unwrap();
        let runtime_mode = world.get_runtime_mode();

        if !runtime_mode.is_preview_mode {
            return Err("Must be in preview mode to start simulation".to_string());
        }

        if runtime_mode.is_simulation_running {
            return Err("Simulation already running".to_string());
        }

        let mut new_mode = runtime_mode.clone();
        new_mode.is_simulation_running = true;
        world.set_runtime_mode(new_mode);

        // Emit event
        self.event_bus.emit(EditorEvent::SimulationStarted);

        Ok(())
    }

    /// Stop simulation
    pub fn stop_simulation(&mut self) -> Result<(), String> {
        let mut world = self.world_state.lock().unwrap();
        let runtime_mode = world.get_runtime_mode();

        if !runtime_mode.is_simulation_running {
            return Err("Simulation not running".to_string());
        }

        let mut new_mode = runtime_mode.clone();
        new_mode.is_simulation_running = false;
        world.set_runtime_mode(new_mode);

        // Emit event
        self.event_bus.emit(EditorEvent::SimulationStopped);

        Ok(())
    }

    /// Check if in preview mode
    pub fn is_preview_mode(&self) -> bool {
        let world = self.world_state.lock().unwrap();
        world.get_runtime_mode().is_preview_mode
    }

    /// Check if simulation is running
    pub fn is_simulation_running(&self) -> bool {
        let world = self.world_state.lock().unwrap();
        world.get_runtime_mode().is_simulation_running
    }

    /// Get diagnostics state reference
    pub fn diagnostics(&self) -> &Arc<Mutex<DiagnosticsState>> {
        &self.diagnostics_state
    }
}


