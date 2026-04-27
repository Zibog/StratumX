use serde::{Deserialize, Serialize};

/// Runtime mode state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeModeState {
    /// Whether preview mode is active.
    pub is_preview_mode: bool,

    /// Whether simulation is running.
    pub is_simulation_running: bool,
}

impl RuntimeModeState {
    /// Creates a new runtime mode state with default values.
    pub fn new() -> Self {
        Self {
            is_preview_mode: false,
            is_simulation_running: false,
        }
    }
}

impl Default for RuntimeModeState {
    fn default() -> Self {
        Self::new()
    }
}
