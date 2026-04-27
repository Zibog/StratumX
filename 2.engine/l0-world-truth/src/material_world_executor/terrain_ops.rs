// Terrain operations - blast response

use super::executor::MaterialWorldExecutor;
use engine_material::{TerrainBlastResponse, TerrainMaterialType};

impl MaterialWorldExecutor {
    pub fn apply_terrain_blast(
        &mut self,
        position: [f32; 3],
        energy_j: f32,
        terrain_type: TerrainMaterialType,
    ) -> usize {
        let response = TerrainBlastResponse::from_explosion(position, energy_j, terrain_type);
        self.terrain_responses.push(response);
        self.terrain_responses.len() - 1
    }

    pub fn get_terrain_response(&self, index: usize) -> Option<&TerrainBlastResponse> {
        self.terrain_responses.get(index)
    }
}
