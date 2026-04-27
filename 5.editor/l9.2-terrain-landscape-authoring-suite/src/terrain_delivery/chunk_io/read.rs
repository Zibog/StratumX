use crate::terrain_delivery::state::TerrainAuthoringState;
use editor_dto_law::ChunkData;
use engine_world::WorldState;
use std::fs;
use std::path::Path;

impl TerrainAuthoringState {
    pub fn read_chunks(&mut self, world: &mut WorldState, world_path: &Path) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No terrain binding exists".to_string());
        }

        let terrain_dir = world_path.join("terrain");
        if !terrain_dir.exists() {
            return Err("Terrain directory does not exist".to_string());
        }

        let entries = fs::read_dir(&terrain_dir)
            .map_err(|e| format!("Failed to read terrain directory: {}", e))?;

        let mut loaded_chunks = 0;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("bin") {
                continue;
            }

            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                if let Some(coords) = self.parse_chunk_filename(filename) {
                    let (chunk_x, chunk_y) = coords;

                    let bytes = fs::read(&path)
                        .map_err(|e| format!("Failed to read chunk file {}: {}", filename, e))?;

                    let chunk_data = ChunkData::from_bytes(&bytes)
                        .map_err(|e| format!("Failed to parse chunk {}: {}", filename, e))?;

                    self.restore_chunk_data(world, chunk_x, chunk_y, &chunk_data)?;

                    if let Some(scene) = world.vertical_slice_scene_mut() {
                        if let Some(chunk) = scene
                            .terrain
                            .chunks
                            .iter_mut()
                            .find(|c| c.chunk_x == chunk_x && c.chunk_y == chunk_y)
                        {
                            chunk.loaded = true;
                            chunk.dirty = false;
                            chunk.mesh_built = false;
                        }
                    }

                    if let Some(region) = self
                        .dirty_regions
                        .iter_mut()
                        .find(|r| r.chunk_x == chunk_x && r.chunk_y == chunk_y)
                    {
                        region.dirty = false;
                    }

                    loaded_chunks += 1;
                }
            }
        }

        if loaded_chunks > 0 {
            self.present = true;
        }

        Ok(())
    }

    pub(super) fn parse_chunk_filename(&self, filename: &str) -> Option<(u32, u32)> {
        let parts: Vec<&str> = filename.split('_').collect();
        if parts.len() == 3 && parts[0] == "chunk" {
            let x = parts[1].parse::<u32>().ok()?;
            let y = parts[2].parse::<u32>().ok()?;
            Some((x, y))
        } else {
            None
        }
    }

    pub(super) fn restore_chunk_data(
        &mut self,
        world: &mut WorldState,
        chunk_x: u32,
        chunk_y: u32,
        chunk_data: &ChunkData,
    ) -> Result<(), String> {
        let scene = world
            .vertical_slice_scene_mut()
            .ok_or("No active scene found")?;
        let terrain = &mut scene.terrain;
        let chunk_size = terrain.chunk_size as usize;
        let terrain_width = terrain.resolution[0] as usize;
        let terrain_height = terrain.resolution[1] as usize;

        let start_x = (chunk_x as usize * chunk_size).min(terrain_width);
        let start_y = (chunk_y as usize * chunk_size).min(terrain_height);
        let end_x = (start_x + chunk_size).min(terrain_width);
        let end_y = (start_y + chunk_size).min(terrain_height);

        let mut chunk_idx = 0;
        for y in start_y..end_y {
            for x in start_x..end_x {
                if chunk_idx < chunk_data.heights.len() {
                    let terrain_idx = y * terrain_width + x;
                    if terrain_idx < terrain.height_samples.len() {
                        terrain.height_samples[terrain_idx] = chunk_data.heights[chunk_idx];
                    }
                }
                chunk_idx += 1;
            }
        }

        chunk_idx = 0;
        for y in start_y..end_y {
            for x in start_x..end_x {
                if chunk_idx < chunk_data.material_weights.len() {
                    let terrain_idx = y * terrain_width + x;
                    if terrain_idx < terrain.layer_weights.len() {
                        terrain.layer_weights[terrain_idx] = chunk_data.material_weights[chunk_idx];
                    }
                }
                chunk_idx += 1;
            }
        }

        Ok(())
    }
}
