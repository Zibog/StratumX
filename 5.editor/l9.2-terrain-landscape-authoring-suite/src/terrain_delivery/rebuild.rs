// Terrain Rebuild Operations

use super::state::TerrainAuthoringState;
use engine_world::{TerrainDirtyRegion, WorldState};

impl TerrainAuthoringState {
    pub fn rebuild_dirty_chunks(&mut self, world: &mut WorldState) -> Result<(), String> {
        if self.pending_build.is_none() {
            return Ok(());
        }

        let build_request = self.pending_build.take().unwrap();
        let scene = world
            .proof_region_scene_mut()
            .ok_or("No active scene found")?;

        if build_request.full_rebuild {
            // Full rebuild - process all chunks
            self.perform_full_rebuild(scene)?;
        } else {
            // Partial rebuild - process only marked regions
            self.perform_partial_rebuild(scene, &build_request.regions)?;
        }

        // Clear dirty flags and sync state
        self.clear_dirty_regions();
        self.sync_state_to_world(scene);

        // Mark that GPU buffers need update
        self.mark_terrain_buffers_dirty();

        Ok(())
    }

    /// Perform full terrain rebuild
    fn perform_full_rebuild(
        &mut self,
        scene: &mut engine_world::ProofRegionScene,
    ) -> Result<(), String> {
        let terrain = &mut scene.terrain;

        // Update revisions to trigger rebuild
        terrain.mesh_revision += 1;
        terrain.collision_revision += 1;

        // Mark all chunks for rebuild
        for chunk in &mut terrain.chunks {
            chunk.dirty = true;
            chunk.mesh_built = false;
        }

        // Update dirty regions with new revision
        terrain.dirty_regions.clear();
        for cy in 0..terrain.chunk_grid[1] {
            for cx in 0..terrain.chunk_grid[0] {
                terrain.dirty_regions.push(TerrainDirtyRegion {
                    chunk_x: cx,
                    chunk_y: cy,
                    revision: terrain.mesh_revision,
                });
            }
        }

        // Note: mesh_built will be set after renderer buffer update
        // This ensures real mesh generation, not just flag setting
        Ok(())
    }

    /// Perform partial terrain rebuild for specific regions
    fn perform_partial_rebuild(
        &mut self,
        scene: &mut engine_world::ProofRegionScene,
        regions: &[super::state::TerrainDirtyRegion],
    ) -> Result<(), String> {
        let terrain = &mut scene.terrain;

        // Update revisions
        terrain.mesh_revision += 1;
        terrain.collision_revision += 1;

        // Process only specified regions
        for region in regions {
            // Update dirty region revision
            if let Some(dirty_region) = terrain
                .dirty_regions
                .iter_mut()
                .find(|r| r.chunk_x == region.chunk_x && r.chunk_y == region.chunk_y)
            {
                dirty_region.revision = terrain.mesh_revision;
            }

            // Mark corresponding chunk for rebuild
            if let Some(chunk) = terrain
                .chunks
                .iter_mut()
                .find(|c| c.chunk_x == region.chunk_x && c.chunk_y == region.chunk_y)
            {
                chunk.dirty = true;
                chunk.mesh_built = false;
                // Note: mesh_built will be set after renderer buffer update
            }
        }

        Ok(())
    }

    /// Sync terrain state back to world truth
    fn sync_state_to_world(&mut self, scene: &mut engine_world::ProofRegionScene) {
        // Update chunk states to match authoring state
        for region in &self.dirty_regions {
            if let Some(chunk) = scene
                .terrain
                .chunks
                .iter_mut()
                .find(|c| c.chunk_x == region.chunk_x && c.chunk_y == region.chunk_y)
            {
                chunk.loaded = true;
                chunk.dirty = region.dirty;
            }
        }
    }

    /// Force immediate terrain buffer update to renderer
    /// This is called by EditorHost after rebuild to sync GPU buffers
    pub fn mark_terrain_buffers_dirty(&mut self) {
        // Mark that terrain buffers need GPU update
        // The actual GPU update happens in viewport_panel via renderer.update_terrain_from_world()
        self.buffers_need_gpu_sync = true;
    }

    pub fn needs_gpu_sync(&self) -> bool {
        self.buffers_need_gpu_sync
    }

    pub fn clear_gpu_sync_flag(&mut self) {
        self.buffers_need_gpu_sync = false;
    }
}
