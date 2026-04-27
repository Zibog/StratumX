//! Terrain Painting Operations

use super::types::PaintOperation;
use super::TerrainAuthoringService;
use engine_world::WorldState;

impl TerrainAuthoringService {
    pub fn paint_terrain(
        &mut self,
        world: &mut WorldState,
        operation: PaintOperation,
    ) -> Result<(), String> {
        self.authoring.paint_material(
            world,
            operation.center,
            operation.radius,
            operation.layer_index as u16,
            operation.strength,
        )?;
        self.capture_authoring_dirtiness();
        Ok(())
    }
}
