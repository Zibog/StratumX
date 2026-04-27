// Weather Simulation Operations

use super::state::EnvironmentAuthoringState;

impl EnvironmentAuthoringState {
    pub fn step_simulation(&mut self, _delta_time: f32) {
        // Simulation step removed - weather is static in authoring mode
    }
}
