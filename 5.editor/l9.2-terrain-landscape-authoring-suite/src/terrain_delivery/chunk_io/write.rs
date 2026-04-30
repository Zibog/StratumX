use crate::terrain_delivery::state::TerrainAuthoringState;
use editor_dto_law::ChunkData;
use engine_world::WorldState;
use std::fs;
use std::path::Path;

impl TerrainAuthoringState {
    pub fn write_chunks(&self, world: &mut WorldState, world_path: &Path) -> Result<(), String> {
        if !self.present {
            return Err("Terrain not present".to_string());
        }

        let terrain_dir = world_path.join("terrain");
        fs::create_dir_all(&terrain_dir)
            .map_err(|e| format!("Failed to create terrain directory: {}", e))?;

        let scene = world
            .proof_region_scene()
            .ok_or("No active scene found")?;
        let terrain = &scene.terrain;
        let chunk_size = terrain.chunk_size as usize;
        let terrain_width = terrain.resolution[0] as usize;
        let terrain_height = terrain.resolution[1] as usize;

        for region in &self.dirty_regions {
            if !region.dirty {
                continue;
            }

            let chunk_filename = format!("chunk_{}_{}.bin", region.chunk_x, region.chunk_y);
            let chunk_path = terrain_dir.join(&chunk_filename);

            let chunk_data = self.extract_chunk_data(
                terrain,
                region.chunk_x,
                region.chunk_y,
                chunk_size,
                terrain_width,
                terrain_height,
            )?;

            let bytes = chunk_data.to_bytes();

            fs::write(&chunk_path, bytes)
                .map_err(|e| format!("Failed to write chunk {}: {}", chunk_filename, e))?;
        }

        Ok(())
    }

    pub(super) fn extract_chunk_data(
        &self,
        terrain: &engine_world::TerrainPatchState,
        chunk_x: u32,
        chunk_y: u32,
        chunk_size: usize,
        terrain_width: usize,
        terrain_height: usize,
    ) -> Result<ChunkData, String> {
        let start_x = (chunk_x as usize * chunk_size).min(terrain_width);
        let start_y = (chunk_y as usize * chunk_size).min(terrain_height);
        let end_x = (start_x + chunk_size).min(terrain_width);
        let end_y = (start_y + chunk_size).min(terrain_height);

        let actual_width = (end_x - start_x) as u32;
        let actual_height = (end_y - start_y) as u32;
        let chunk_area = (actual_width * actual_height) as usize;

        let mut heights = Vec::with_capacity(chunk_area);
        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * terrain_width + x;
                if idx < terrain.height_samples.len() {
                    heights.push(terrain.height_samples[idx]);
                } else {
                    heights.push(0.0);
                }
            }
        }

        let mut material_weights = Vec::with_capacity(chunk_area);
        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * terrain_width + x;
                if idx < terrain.layer_weights.len() {
                    material_weights.push(terrain.layer_weights[idx]);
                } else {
                    material_weights.push([1.0, 0.0, 0.0, 0.0]);
                }
            }
        }

        Ok(ChunkData {
            header: editor_dto_law::ChunkHeader::new(actual_width, actual_height),
            heights,
            material_weights,
        })
    }
}
