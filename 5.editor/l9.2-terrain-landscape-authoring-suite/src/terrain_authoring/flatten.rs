// Terrain flatten operations

use super::TerrainAuthoring;
use engine_world::WorldState;

impl TerrainAuthoring {
    pub fn sculpt_flatten(
        &mut self,
        world: &mut WorldState,
        center: [f32; 2],
        radius: f32,
        target_height: f32,
        strength: f32,
    ) -> Result<(), String> {
        if let Some(scene) = world.proof_region_scene_mut() {
            let terrain = &mut scene.terrain;
            let res_x = terrain.resolution[0] as usize;
            let res_y = terrain.resolution[1] as usize;
            let world_size = terrain.world_size;
            let origin = terrain.origin;

            let terrain_x = ((center[0] - origin[0]) / world_size[0]) * res_x as f32;
            let terrain_y = ((center[1] - origin[2]) / world_size[1]) * res_y as f32;
            let radius_samples = (radius / world_size[0] * res_x as f32).max(1.0);

            for y in 0..res_y {
                for x in 0..res_x {
                    let dx = x as f32 - terrain_x;
                    let dy = y as f32 - terrain_y;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < radius_samples {
                        let falloff = 1.0 - (dist / radius_samples).min(1.0);
                        let blend = falloff * strength;

                        let idx = y * res_x + x;
                        if idx < terrain.height_samples.len() {
                            terrain.height_samples[idx] =
                                terrain.height_samples[idx] * (1.0 - blend) + target_height * blend;
                        }
                    }
                }
            }

            self.mark_region_dirty(terrain, center, radius);

            Ok(())
        } else {
            Err("No scene in world".to_string())
        }
    }
}
