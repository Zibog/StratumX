// World Sync Operations

use super::state::EnvironmentAuthoringState;
use engine_world::WorldState;

impl EnvironmentAuthoringState {
    pub fn sync_to_world(&mut self, world: &mut WorldState) -> Result<(), String> {
        if !self.present {
            return Err("Environment not present".to_string());
        }
        self.engine_bridge.sync_to_world(world)
    }

    pub fn sync_from_world(&mut self, world: &WorldState) -> Result<(), String> {
        if let Some(scene) = world.vertical_slice_scene() {
            self.time_of_day = scene.sky.celestial.time_of_day_hours;
            self.present = true;
            self.engine_bridge.sync_from_world(world)?;
            Ok(())
        } else {
            Err("No vertical slice scene in world".to_string())
        }
    }
}
