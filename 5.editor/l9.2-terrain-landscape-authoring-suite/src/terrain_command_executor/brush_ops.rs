use super::{RuntimeHostAccess, TerrainCommandExecutor};

impl TerrainCommandExecutor {
    /// Apply a terrain brush
    ///
    /// Applies a terrain sculpting brush at the specified position.
    pub fn apply_brush(
        &self,
        runtime_host: &mut dyn RuntimeHostAccess,
        position: [f32; 2],
        radius: f32,
        delta_strength: f32,
    ) -> Result<String, String> {
        {
            let scene = runtime_host
                .get_world_state_mut()
                .and_then(|world| world.vertical_slice_scene_mut())
                .ok_or_else(|| "No world loaded".to_string())?;

            let resolution = scene.terrain.resolution;
            let world_size = scene.terrain.world_size;
            let center_x =
                ((position[0] / world_size[0]).clamp(0.0, 1.0) * resolution[0] as f32) as i32;
            let center_y =
                ((position[1] / world_size[1]).clamp(0.0, 1.0) * resolution[1] as f32) as i32;
            let radius_px = ((radius / world_size[0]) * resolution[0] as f32).max(1.0) as i32;

            for y in (center_y - radius_px)..=(center_y + radius_px) {
                for x in (center_x - radius_px)..=(center_x + radius_px) {
                    if x < 0 || y < 0 || x >= resolution[0] as i32 || y >= resolution[1] as i32 {
                        continue;
                    }

                    let dx = x - center_x;
                    let dy = y - center_y;
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();
                    if distance > radius_px as f32 {
                        continue;
                    }

                    let weight = 1.0 - (distance / radius_px as f32);
                    let index = (y as usize * resolution[0] as usize) + x as usize;
                    if let Some(sample) = scene.terrain.height_samples.get_mut(index) {
                        *sample = (*sample + delta_strength * weight * 5.0).max(0.0);
                    }
                }
            }

            scene.terrain.mesh_revision = scene.terrain.mesh_revision.saturating_add(1);
            scene.terrain.collision_revision = scene.terrain.collision_revision.saturating_add(1);
        }

        runtime_host.mark_terrain_gpu_dirty();
        Ok("Terrain updated".to_string())
    }
}
