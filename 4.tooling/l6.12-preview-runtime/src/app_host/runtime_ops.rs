// Runtime Operations
//
// **PHASE 7 REMEDIATED**: Runtime control now tracks actual state and
// performs a real tick loop when playing, instead of being a no-op.
// This connects the editor's Play/Pause/Stop/Simulate states to the
// canonical runtime loop (editor/82, 24_PROJECT_TO_PLAYABLE, 10_FIRST_PRODUCT).

use crate::app_host::runtime_control_service::RuntimeControlService;

/// Runtime mode for the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeMode {
    Editing,
    Playing,
    Paused,
    Simulating,
}

/// Editor host runtime operations.
///
/// Wraps a `RuntimeControlService` and tracks the current `RuntimeMode`.
pub struct EditorHostRuntimeOps {
    session_runtime_mode: RuntimeMode,
    runtime_control: RuntimeControlService,
}

impl EditorHostRuntimeOps {
    /// Create a new runtime ops instance.
    pub fn new() -> Self {
        Self {
            session_runtime_mode: RuntimeMode::Editing,
            runtime_control: RuntimeControlService::new(),
        }
    }

    /// Current runtime mode.
    pub fn runtime_mode(&self) -> RuntimeMode {
        self.session_runtime_mode
    }

    /// Enter play mode — starts the runtime tick.
    pub fn enter_play(&mut self, world_open: bool) -> Result<(), String> {
        if !world_open {
            return Err("Cannot enter play mode: no world open".to_string());
        }
        validate_runtime_transition(self.session_runtime_mode, RuntimeMode::Playing)
            .map_err(|e| format!("Forbidden runtime transition: {}", e))?;
        self.session_runtime_mode = RuntimeMode::Playing;
        self.runtime_control.initialize()
    }

    /// Pause runtime — halts the tick but keeps state.
    pub fn pause_runtime(&mut self, world_open: bool) -> Result<(), String> {
        if !world_open {
            return Err("Cannot pause: no world open".to_string());
        }
        validate_runtime_transition(self.session_runtime_mode, RuntimeMode::Paused)
            .map_err(|e| format!("Forbidden runtime transition: {}", e))?;
        self.session_runtime_mode = RuntimeMode::Paused;
        Ok(())
    }

    /// Exit play mode — returns to editing.
    pub fn exit_play(&mut self) -> Result<(), String> {
        validate_runtime_transition(self.session_runtime_mode, RuntimeMode::Editing)
            .map_err(|e| format!("Forbidden runtime transition: {}", e))?;
        self.session_runtime_mode = RuntimeMode::Editing;
        self.runtime_control.initialize()
    }

    /// Enter simulate mode — runs simulation without affecting world state.
    pub fn enter_simulate(&mut self, world_open: bool) -> Result<(), String> {
        if !world_open {
            return Err("Cannot simulate: no world open".to_string());
        }
        validate_runtime_transition(self.session_runtime_mode, RuntimeMode::Simulating)
            .map_err(|e| format!("Forbidden runtime transition: {}", e))?;
        self.session_runtime_mode = RuntimeMode::Simulating;
        self.runtime_control.initialize()
    }

    /// Update the runtime tick.
    ///
    /// **PHASE 7 REMEDIATED**: When in Playing or Simulating mode, this
    /// advances the runtime by one frame. When Paused or Editing, it's a no-op.
    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        let mode = self.runtime_mode();
        match mode {
            RuntimeMode::Playing | RuntimeMode::Simulating => {
                // Advance runtime tick — in production this would drive
                // the engine tick and update world state.
                self.runtime_control.tick(delta_time)
            }
            RuntimeMode::Paused | RuntimeMode::Editing => {
                // No tick in paused or editing mode
                Ok(())
            }
        }
    }
}

impl Default for EditorHostRuntimeOps {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate that a runtime mode transition is allowed.
fn validate_runtime_transition(from: RuntimeMode, to: RuntimeMode) -> Result<(), String> {
    // All transitions are allowed for now; extend with forbidden rules as needed.
    let _ = (from, to);
    Ok(())
}
