// Dirty region tracking for terrain chunks

use super::TerrainAuthoring;
use engine_world::TerrainPatchState;

impl TerrainAuthoring {
    pub(crate) fn mark_region_dirty(
        &mut self,
        terrain: &TerrainPatchState,
        center: [f32; 2],
        radius: f32,
    ) {
        let min_x = ((center[0] - radius - terrain.origin[0]) / terrain.world_size[0]
            * terrain.chunk_grid[0] as f32)
            .floor() as i32;
        let max_x = ((center[0] + radius - terrain.origin[0]) / terrain.world_size[0]
            * terrain.chunk_grid[0] as f32)
            .ceil() as i32;
        let min_y = ((center[1] - radius - terrain.origin[2]) / terrain.world_size[1]
            * terrain.chunk_grid[1] as f32)
            .floor() as i32;
        let max_y = ((center[1] + radius - terrain.origin[2]) / terrain.world_size[1]
            * terrain.chunk_grid[1] as f32)
            .ceil() as i32;

        for cy in min_y..=max_y {
            for cx in min_x..=max_x {
                if cx >= 0
                    && cx < terrain.chunk_grid[0] as i32
                    && cy >= 0
                    && cy < terrain.chunk_grid[1] as i32
                {
                    self.mark_chunk_dirty(cx as u32, cy as u32);
                }
            }
        }
    }

    pub(crate) fn _mark_all_chunks_dirty(&mut self, terrain: &TerrainPatchState) {
        for cy in 0..terrain.chunk_grid[1] {
            for cx in 0..terrain.chunk_grid[0] {
                self.mark_chunk_dirty(cx, cy);
            }
        }
    }
}
