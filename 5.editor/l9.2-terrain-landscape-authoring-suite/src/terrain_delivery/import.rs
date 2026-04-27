// Terrain Import Operations

use super::state::TerrainAuthoringState;
use engine_world::{TerrainChunk, TerrainDirtyRegion, WorldState};
use std::fs;
use std::path::Path;

impl TerrainAuthoringState {
    /// Import heightmap from file
    ///
    /// Supports:
    /// - Raw 16-bit grayscale (.r16)
    /// - Raw 8-bit grayscale (.raw)
    /// - PNG grayscale (.png)
    pub fn import_heightmap(&mut self, world: &mut WorldState, path: &Path) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No terrain binding exists".to_string());
        }

        // Read source file
        let data = fs::read(path).map_err(|e| format!("Failed to read heightmap file: {}", e))?;

        // Determine format and decode
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let (height_samples, width, height) = match extension {
            "r16" => self.decode_r16_heightmap(&data)?,
            "raw" => self.decode_raw_heightmap(&data)?,
            "png" => self.decode_png_heightmap(&data)?,
            _ => return Err(format!("Unsupported file extension: {}", extension)),
        };

        // Validate dimensions
        if height_samples.len() != (width * height) as usize {
            return Err(format!(
                "Heightmap size mismatch: expected {} samples, got {}",
                width * height,
                height_samples.len()
            ));
        }

        // Update terrain state
        if let Some(scene) = world.vertical_slice_scene_mut() {
            // Set terrain properties
            scene.terrain.resolution = [width, height];
            scene.terrain.height_samples = height_samples;
            scene.terrain.layer_weights = vec![[1.0, 0.0, 0.0, 0.0]; (width * height) as usize];
            scene.terrain.material_layer_ids = vec![0; (width * height) as usize];

            // Calculate chunk grid (64x64 chunks)
            let chunk_size = 64;
            let chunk_grid_x = width.div_ceil(chunk_size);
            let chunk_grid_y = height.div_ceil(chunk_size);
            scene.terrain.chunk_grid = [chunk_grid_x, chunk_grid_y];
            scene.terrain.chunk_size = chunk_size;

            // Initialize chunks if needed
            if scene.terrain.chunks.len() != (chunk_grid_x * chunk_grid_y) as usize {
                scene.terrain.chunks.clear();
                for cy in 0..chunk_grid_y {
                    for cx in 0..chunk_grid_x {
                        scene.terrain.chunks.push(TerrainChunk {
                            chunk_x: cx,
                            chunk_y: cy,
                            data_file: None,
                            loaded: false,
                            dirty: true,
                            mesh_built: false,
                        });
                    }
                }
            }

            // Mark all regions dirty
            scene.terrain.dirty_regions.clear();
            for cy in 0..chunk_grid_y {
                for cx in 0..chunk_grid_x {
                    scene.terrain.dirty_regions.push(TerrainDirtyRegion {
                        chunk_x: cx,
                        chunk_y: cy,
                        revision: scene.terrain.mesh_revision + 1,
                    });

                    // Also mark in delivery state
                    self.mark_region_dirty(cx, cy);
                }
            }

            // Update mesh revision to trigger rebuild
            scene.terrain.mesh_revision += 1;
            scene.terrain.collision_revision += 1;

            // Create pending build request
            self.queue_rebuild()?;
        } else {
            return Err("No active scene found".to_string());
        }

        Ok(())
    }

    fn decode_r16_heightmap(&self, data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
        // Calculate dimensions from data size (2 bytes per pixel)
        let total_pixels = data.len() / 2;
        let size = (total_pixels as f64).sqrt() as u32;
        let expected_size = (size * size * 2) as usize;

        if data.len() != expected_size {
            return Err(format!(
                "R16 heightmap size mismatch: expected {} bytes for {}x{}, got {}",
                expected_size,
                size,
                size,
                data.len()
            ));
        }

        let mut samples = Vec::with_capacity((size * size) as usize);
        for i in 0..(size * size) as usize {
            let idx = i * 2;
            let value = u16::from_le_bytes([data[idx], data[idx + 1]]);
            samples.push(value as f32 / 65535.0 * 1000.0); // Scale to 0-1000m
        }

        Ok((samples, size, size))
    }

    fn decode_raw_heightmap(&self, data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
        // Calculate dimensions from data size (1 byte per pixel)
        let total_pixels = data.len();
        let size = (total_pixels as f64).sqrt() as u32;
        let expected_size = (size * size) as usize;

        if data.len() != expected_size {
            return Err(format!(
                "Raw heightmap size mismatch: expected {} bytes for {}x{}, got {}",
                expected_size,
                size,
                size,
                data.len()
            ));
        }

        let samples: Vec<f32> = data
            .iter()
            .map(|&b| b as f32 / 255.0 * 1000.0) // Scale to 0-1000m
            .collect();

        Ok((samples, size, size))
    }

    fn decode_png_heightmap(&self, data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
        // Decode PNG using proper image library
        let image =
            image::load_from_memory(data).map_err(|e| format!("Failed to decode PNG: {}", e))?;

        // Convert to grayscale
        let gray_image = image.to_luma8();
        let width = gray_image.width();
        let height = gray_image.height();

        // Extract height samples
        let samples: Vec<f32> = gray_image
            .pixels()
            .map(|p| p[0] as f32 / 255.0 * 1000.0) // Scale to 0-1000m
            .collect();

        Ok((samples, width, height))
    }
}
