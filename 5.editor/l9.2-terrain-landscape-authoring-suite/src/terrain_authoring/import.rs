//! Terrain Import Operations

use super::types::HeightmapFormat;
use super::TerrainAuthoringService;
use engine_world::WorldState;
use std::path::Path;

impl TerrainAuthoringService {
    pub fn import_heightmap(
        &mut self,
        world: &mut WorldState,
        path: &Path,
    ) -> Result<HeightmapFormat, String> {
        if !self.delivery.is_bound() {
            return Err("Terrain must be bound before heightmap import".to_string());
        }

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        let format = match extension.as_str() {
            "raw" => {
                self.delivery.import_heightmap(world, path)?;
                HeightmapFormat::Raw
            }
            "r16" => {
                self.delivery.import_heightmap(world, path)?;
                HeightmapFormat::R16
            }
            "png" => {
                self.delivery.import_heightmap(world, path)?;
                HeightmapFormat::Png
            }
            "tif" | "tiff" => {
                self.import_tiff_heightmap(world, path)?;
                HeightmapFormat::Tiff
            }
            _ => return Err(format!("Unsupported heightmap format: {}", extension)),
        };

        self.update_manifest_from_world(world)?;
        self.capture_world_dirtiness(world);
        Ok(format)
    }

    pub(super) fn import_tiff_heightmap(
        &mut self,
        world: &mut WorldState,
        path: &Path,
    ) -> Result<(), String> {
        let image = image::open(path).map_err(|error| format!("Failed to decode TIFF: {error}"))?;
        let gray = image.to_luma16();
        let width = gray.width();
        let height = gray.height();
        let samples = gray
            .pixels()
            .map(|pixel| pixel.0[0] as f32 / 65535.0 * 1000.0)
            .collect::<Vec<_>>();

        let scene = world
            .proof_region_scene_mut()
            .ok_or_else(|| "No active scene found".to_string())?;
        scene.terrain.resolution = [width, height];
        scene.terrain.height_samples = samples;
        scene.terrain.layer_weights = vec![[1.0, 0.0, 0.0, 0.0]; (width * height) as usize];
        scene.terrain.material_layer_ids = vec![0; (width * height) as usize];
        scene.terrain.chunk_grid = [width.div_ceil(64), height.div_ceil(64)];
        scene.terrain.chunk_size = 64;
        scene.terrain.mesh_revision += 1;
        scene.terrain.collision_revision += 1;
        scene.terrain.dirty_regions.clear();
        for cy in 0..scene.terrain.chunk_grid[1] {
            for cx in 0..scene.terrain.chunk_grid[0] {
                scene
                    .terrain
                    .dirty_regions
                    .push(engine_world::TerrainDirtyRegion {
                        chunk_x: cx,
                        chunk_y: cy,
                        revision: scene.terrain.mesh_revision,
                    });
            }
        }
        if self.delivery.is_bound() {
            self.delivery.terrain_rebuild()?;
        }
        Ok(())
    }
}
