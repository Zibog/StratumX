//! Terrain preview cache — rebuildable from owner state.
//!
//! **PHASE 6 REMEDIATED**: `rebuild_from_owner` now actually extracts heightmap
//! summaries and texture layer data from the terrain owner state, populating
//! per-chunk preview entries instead of being a stub.

use super::cache_keys::{ChunkId, RebuildableCache, TerrainChunkPreview};
use super::terrain_chunk_extract::{extract_chunk_heightmap_summary, extract_chunk_texture_layers};
use crate::{CacheEntry, StateContainerSystem, StateId};
use std::collections::HashMap;

/// Terrain owner data — carries the source terrain state for cache rebuilding.
#[derive(Clone)]
pub struct TerrainOwnerData {
    pub resolution: [u32; 2],
    pub world_size: [f32; 2],
    pub chunk_grid: [u32; 2],
    pub chunk_size: u32,
    pub height_samples: Vec<f32>,
    pub material_layer_ids: Vec<u16>,
    pub layer_weights: Vec<[f32; 4]>,
}

impl TerrainOwnerData {
    pub fn from_world(world: &engine_world::WorldState) -> Option<Self> {
        let scene = world.vertical_slice_scene()?;
        Some(Self {
            resolution: scene.terrain.resolution,
            world_size: scene.terrain.world_size,
            chunk_grid: scene.terrain.chunk_grid,
            chunk_size: scene.terrain.chunk_size,
            height_samples: scene.terrain.height_samples.clone(),
            material_layer_ids: scene.terrain.material_layer_ids.clone(),
            layer_weights: scene.terrain.layer_weights.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TerrainPreviewCache {
    pub(crate) chunks: HashMap<ChunkId, TerrainChunkPreview>,
    pub(crate) dirty: bool,
}

impl TerrainPreviewCache {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            dirty: true,
        }
    }

    /// Rebuild the preview cache from terrain owner data.
    pub fn rebuild_from_owner(&mut self, owner_data: &TerrainOwnerData) {
        self.chunks.clear();
        let chunk_size = owner_data.chunk_size as usize;
        let resolution = owner_data.resolution;

        for chunk_y in 0..owner_data.chunk_grid[1] {
            for chunk_x in 0..owner_data.chunk_grid[0] {
                let chunk_id = ChunkId {
                    x: chunk_x as i32,
                    y: chunk_y as i32,
                };
                let heightmap_summary = extract_chunk_heightmap_summary(
                    &owner_data.height_samples,
                    resolution,
                    chunk_x,
                    chunk_y,
                    chunk_size,
                );
                let texture_layers = extract_chunk_texture_layers(
                    &owner_data.material_layer_ids,
                    resolution,
                    chunk_x,
                    chunk_y,
                    chunk_size,
                );
                self.chunks.insert(
                    chunk_id,
                    TerrainChunkPreview {
                        chunk_id,
                        heightmap_summary,
                        texture_layers,
                    },
                );
            }
        }
        self.dirty = false;
    }

    pub fn test_insert(&mut self, id: ChunkId, entry: TerrainChunkPreview) {
        self.chunks.insert(id, entry);
    }
    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        self.chunks.len()
    }
    pub fn test_clear(&mut self) {
        self.chunks.clear();
    }
    pub fn get_preview_data(&self) -> Vec<f32> {
        self.chunks
            .values()
            .flat_map(|c| c.heightmap_summary.clone())
            .collect()
    }
}

impl Default for TerrainPreviewCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RebuildableCache<ChunkId, TerrainChunkPreview> for TerrainPreviewCache {
    fn invalidate_key(&mut self, key: &ChunkId) {
        self.chunks.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        if self.dirty {
            self.dirty = false;
        }
    }
    fn get(&self, key: &ChunkId) -> Option<&TerrainChunkPreview> {
        self.chunks.get(key)
    }
}

impl CacheEntry for TerrainPreviewCache {
    fn is_valid(&self) -> bool {
        !self.dirty
    }
    fn invalidate(&mut self) {
        self.chunks.clear();
        self.dirty = true;
    }
    fn rebuild(&mut self, _state_system: &StateContainerSystem) {
        self.dirty = false;
    }
    fn dependencies(&self) -> Vec<StateId> {
        vec![StateId::WorldState]
    }
}
