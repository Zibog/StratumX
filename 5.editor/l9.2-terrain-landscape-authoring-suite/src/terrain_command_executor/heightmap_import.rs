use super::heightmap_decode::{decode_png_heightmap, decode_r16_heightmap, decode_raw_heightmap};
use super::{RuntimeHostAccess, TerrainCommandExecutor};
use std::path::Path;

impl TerrainCommandExecutor {
    /// Import a heightmap into the active terrain
    ///
    /// Imports a heightmap file into the active world's terrain.
    pub fn import_heightmap(
        &self,
        runtime_host: &mut dyn RuntimeHostAccess,
        path: &Path,
    ) -> Result<String, String> {
        if !path.exists() {
            return Err(format!("Heightmap not found: {}", path.display()));
        }

        let data =
            std::fs::read(path).map_err(|error| format!("Failed to read heightmap: {}", error))?;

        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        let (samples, width, height) = match extension.as_str() {
            "raw" => decode_raw_heightmap(&data)?,
            "r16" => decode_r16_heightmap(&data)?,
            "png" => decode_png_heightmap(&data)?,
            _ => return Err(format!("Unsupported heightmap format: {}", extension)),
        };

        {
            let scene = runtime_host
                .get_world_state_mut()
                .and_then(|world| world.vertical_slice_scene_mut())
                .ok_or_else(|| "No world loaded".to_string())?;

            scene.terrain.resolution = [width, height];
            scene.terrain.height_samples = samples;
            scene.terrain.layer_weights = vec![[1.0, 0.0, 0.0, 0.0]; (width * height) as usize];
            scene.terrain.material_layer_ids = vec![0; (width * height) as usize];
            scene.terrain.chunk_size = 64;
            scene.terrain.chunk_grid = [width.div_ceil(64), height.div_ceil(64)];
            scene.terrain.chunks = (0..scene.terrain.chunk_grid[1])
                .flat_map(|chunk_y| {
                    (0..scene.terrain.chunk_grid[0]).map(move |chunk_x| {
                        engine_world::TerrainChunk {
                            chunk_x,
                            chunk_y,
                            data_file: None,
                            loaded: true,
                            dirty: false,
                            mesh_built: true,
                        }
                    })
                })
                .collect();
            scene.terrain.mesh_revision = scene.terrain.mesh_revision.saturating_add(1);
            scene.terrain.collision_revision = scene.terrain.collision_revision.saturating_add(1);
        }

        runtime_host.mark_terrain_gpu_dirty();
        Ok(format!("Heightmap imported: {}", path.display()))
    }
}
