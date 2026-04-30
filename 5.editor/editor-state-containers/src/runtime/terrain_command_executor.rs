//! Terrain command executor

use std::path::Path;

pub trait RuntimeHostAccess {
    fn get_world_state_mut(&mut self) -> Option<&mut engine_world::WorldState>;
    fn mark_terrain_gpu_dirty(&mut self);
}

pub struct TerrainCommandExecutor;

impl TerrainCommandExecutor {
    pub fn new<T: RuntimeHostAccess>(_host: T) -> Self {
        Self
    }

    pub fn execute(&mut self, _cmd: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn import_heightmap<T: RuntimeHostAccess>(
        &mut self,
        host: &mut T,
        path: &Path,
    ) -> Result<(), String> {
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
            _ => return Err(format!("Unsupported heightmap format: {}", extension)),
        };

        let scene = host
            .get_world_state_mut()
            .and_then(|world| world.proof_region_scene_mut())
            .ok_or_else(|| "No world loaded".to_string())?;

        scene.terrain.resolution = [width, height];
        scene.terrain.height_source_ref = Some(path.display().to_string());
        scene.terrain.height_samples = samples;
        scene.terrain.layer_weights = vec![[1.0, 0.0, 0.0, 0.0]; (width * height) as usize];
        scene.terrain.material_layer_ids = vec![0; (width * height) as usize];
        scene.terrain.chunk_size = 64;
        scene.terrain.chunk_grid = [width.div_ceil(64), height.div_ceil(64)];
        scene.terrain.chunks = (0..scene.terrain.chunk_grid[1])
            .flat_map(|chunk_y| {
                (0..scene.terrain.chunk_grid[0]).map(move |chunk_x| engine_world::TerrainChunk {
                    chunk_x,
                    chunk_y,
                    data_file: None,
                    loaded: true,
                    dirty: false,
                    mesh_built: true,
                })
            })
            .collect();
        scene.terrain.dirty_regions.clear();
        scene.terrain.mesh_revision = scene.terrain.mesh_revision.saturating_add(1);
        scene.terrain.collision_revision = scene.terrain.collision_revision.saturating_add(1);

        host.mark_terrain_gpu_dirty();
        Ok(())
    }
}

fn decode_raw_heightmap(data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
    let total_pixels = data.len();
    let size = (total_pixels as f64).sqrt() as u32;
    if (size * size) as usize != total_pixels {
        return Err("RAW heightmap must be square".to_string());
    }

    let samples = data
        .iter()
        .map(|value| *value as f32 / 255.0 * 1000.0)
        .collect();
    Ok((samples, size, size))
}

fn decode_r16_heightmap(data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
    if data.len() % 2 != 0 {
        return Err("R16 heightmap byte count must be even".to_string());
    }

    let total_pixels = data.len() / 2;
    let size = (total_pixels as f64).sqrt() as u32;
    if (size * size) as usize != total_pixels {
        return Err("R16 heightmap must be square".to_string());
    }

    let mut samples = Vec::with_capacity(total_pixels);
    for index in 0..total_pixels {
        let offset = index * 2;
        let value = u16::from_le_bytes([data[offset], data[offset + 1]]);
        samples.push(value as f32 / 65535.0 * 1000.0);
    }

    Ok((samples, size, size))
}
