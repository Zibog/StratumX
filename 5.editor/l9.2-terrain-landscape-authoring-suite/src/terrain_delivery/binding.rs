// Terrain Binding Operations

use super::state::TerrainAuthoringState;
use engine_world::WorldState;

impl TerrainAuthoringState {
    pub fn sync_to_world(&self, world: &mut WorldState) -> Result<(), String> {
        if !self.present {
            return Err("Terrain not present".to_string());
        }

        if let Some(_scene) = world.proof_region_scene() {
            Ok(())
        } else {
            Err("No vertical slice scene in world".to_string())
        }
    }

    pub fn sync_from_world(&mut self, world: &WorldState) -> Result<(), String> {
        if let Some(_scene) = world.proof_region_scene() {
            self.present = true;
            Ok(())
        } else {
            Err("No vertical slice scene in world".to_string())
        }
    }
}
