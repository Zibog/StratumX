//! Terrain Sculpting Operations

use super::types::{ChunkId, SculptOperation};
use super::TerrainAuthoringService;
use engine_world::WorldState;

impl TerrainAuthoringService {
    pub fn sculpt_terrain(
        &mut self,
        world: &mut WorldState,
        operation: SculptOperation,
    ) -> Result<(), String> {
        match operation {
            SculptOperation::Raise {
                center,
                radius,
                strength,
            } => self
                .authoring
                .sculpt_raise(world, center, radius, strength)?,
            SculptOperation::Lower {
                center,
                radius,
                strength,
            } => self
                .authoring
                .sculpt_lower(world, center, radius, strength)?,
            SculptOperation::Smooth {
                center,
                radius,
                strength,
            } => self
                .authoring
                .sculpt_smooth(world, center, radius, strength)?,
            SculptOperation::Flatten {
                center,
                radius,
                strength,
                target_height,
            } => self
                .authoring
                .sculpt_flatten(world, center, radius, target_height, strength)?,
        }

        self.capture_authoring_dirtiness();
        Ok(())
    }

    pub(super) fn capture_authoring_dirtiness(&mut self) {
        for (chunk_x, chunk_y) in self.authoring.get_dirty_chunks() {
            self.chunk_dirtiness
                .insert(ChunkId::new(*chunk_x, *chunk_y), true);
        }
    }
}
