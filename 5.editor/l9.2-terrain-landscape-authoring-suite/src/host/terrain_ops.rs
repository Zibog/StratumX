//! Terrain-related host helpers.
//!
//! Standalone operations struct for terrain dirty tracking.

use std::collections::HashSet;

/// Terrain operations helper for dirty tracking.
pub struct TerrainOps {
    pub terrain_gpu_dirty: bool,
    pub environment_dirty: bool,
    pub terrain_material_visual_dirty: bool,
    pub dirty_terrain_chunks: HashSet<(u32, u32)>,
}

impl TerrainOps {
    pub fn new() -> Self {
        Self {
            terrain_gpu_dirty: false,
            environment_dirty: false,
            terrain_material_visual_dirty: false,
            dirty_terrain_chunks: HashSet::new(),
        }
    }

    pub fn terrain_needs_gpu_sync(&self) -> bool {
        self.terrain_gpu_dirty
    }

    pub fn clear_terrain_gpu_sync_flag(&mut self) {
        self.terrain_gpu_dirty = false;
    }

    // --- Environment dirty accessors (PHASE 8 REMEDIATED) ---

    pub fn environment_needs_update(&self) -> bool {
        self.environment_dirty
    }

    pub fn mark_environment_dirty(&mut self) {
        self.environment_dirty = true;
    }

    pub fn clear_environment_dirty(&mut self) {
        self.environment_dirty = false;
    }

    // --- Terrain material visual dirty (PHASE E) ---

    /// Mark terrain material visuals as dirty (texture swap, not mesh rebuild).
    pub fn mark_terrain_material_visual_dirty(&mut self) {
        self.terrain_material_visual_dirty = true;
    }

    /// Check if terrain material visuals need update.
    pub fn terrain_material_needs_visual_update(&self) -> bool {
        self.terrain_material_visual_dirty
    }

    /// Clear terrain material visual dirty flag after update.
    pub fn clear_terrain_material_visual_dirty(&mut self) {
        self.terrain_material_visual_dirty = false;
    }

    // --- Chunk-level dirty tracking (PHASE 8 REMEDIATED) ---

    /// Mark a specific chunk as dirty for GPU sync.
    pub fn mark_terrain_chunk_dirty(&mut self, chunk_x: u32, chunk_y: u32) {
        self.dirty_terrain_chunks.insert((chunk_x, chunk_y));
        // Also set the coarse flag so the next frame knows to sync
        self.terrain_gpu_dirty = true;
    }

    /// Get the set of dirty chunk coordinates.
    pub fn dirty_terrain_chunks(&self) -> &HashSet<(u32, u32)> {
        &self.dirty_terrain_chunks
    }

    /// Clear all dirty terrain chunk tracking.
    pub fn clear_dirty_terrain_chunks(&mut self) {
        self.dirty_terrain_chunks.clear();
        self.terrain_gpu_dirty = false;
    }

    pub fn rebuild_terrain(&mut self) -> Result<(), String> {
        self.terrain_gpu_dirty = true;
        Ok(())
    }
}

impl Default for TerrainOps {
    fn default() -> Self {
        Self::new()
    }
}
