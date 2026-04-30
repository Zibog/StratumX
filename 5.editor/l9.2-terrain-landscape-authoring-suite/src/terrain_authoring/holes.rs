// Terrain hole operations (caves, tunnels)

use super::TerrainAuthoring;
use engine_world::WorldState;

impl TerrainAuthoring {
    pub fn create_hole(
        &mut self,
        world: &mut WorldState,
        center: [f32; 2],
        radius: f32,
    ) -> Result<(), String> {
        if let Some(scene) = world.proof_region_scene_mut() {
            let terrain = &mut scene.terrain;

            if terrain.hole_mask.is_none() {
                let total_samples = (terrain.resolution[0] * terrain.resolution[1]) as usize;
                terrain.hole_mask = Some(vec![0u8; total_samples.div_ceil(8)]);
            }

            let res_x = terrain.resolution[0] as usize;
            let res_y = terrain.resolution[1] as usize;
            let world_size = terrain.world_size;
            let origin = terrain.origin;

            let terrain_x = ((center[0] - origin[0]) / world_size[0]) * res_x as f32;
            let terrain_y = ((center[1] - origin[2]) / world_size[1]) * res_y as f32;
            let radius_samples = (radius / world_size[0] * res_x as f32).max(1.0);

            if let Some(hole_mask) = &mut terrain.hole_mask {
                for y in 0..res_y {
                    for x in 0..res_x {
                        let dx = x as f32 - terrain_x;
                        let dy = y as f32 - terrain_y;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist < radius_samples {
                            let idx = y * res_x + x;
                            let byte_idx = idx / 8;
                            let bit_idx = idx % 8;

                            if byte_idx < hole_mask.len() {
                                hole_mask[byte_idx] |= 1 << bit_idx;
                            }
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

    pub fn fill_hole(
        &mut self,
        world: &mut WorldState,
        center: [f32; 2],
        radius: f32,
    ) -> Result<(), String> {
        if let Some(scene) = world.proof_region_scene_mut() {
            let terrain = &mut scene.terrain;

            if terrain.hole_mask.is_none() {
                return Ok(());
            }

            let res_x = terrain.resolution[0] as usize;
            let res_y = terrain.resolution[1] as usize;
            let world_size = terrain.world_size;
            let origin = terrain.origin;

            let terrain_x = ((center[0] - origin[0]) / world_size[0]) * res_x as f32;
            let terrain_y = ((center[1] - origin[2]) / world_size[1]) * res_y as f32;
            let radius_samples = (radius / world_size[0] * res_x as f32).max(1.0);

            if let Some(hole_mask) = &mut terrain.hole_mask {
                for y in 0..res_y {
                    for x in 0..res_x {
                        let dx = x as f32 - terrain_x;
                        let dy = y as f32 - terrain_y;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist < radius_samples {
                            let idx = y * res_x + x;
                            let byte_idx = idx / 8;
                            let bit_idx = idx % 8;

                            if byte_idx < hole_mask.len() {
                                hole_mask[byte_idx] &= !(1 << bit_idx);
                            }
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
