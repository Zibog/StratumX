//! Terrain cache types

use super::traits::{CacheEntry, RebuildableCache};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainChunkPreview {
    pub chunk_id: ChunkId,
    pub heightmap_summary: Vec<f32>,
    pub texture_layers: Vec<String>,
}

impl CacheEntry for TerrainChunkPreview {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct TerrainOwnerData {
    pub resolution: [usize; 2],
    pub world_size: [f32; 2],
    pub chunk_grid: [usize; 2],
    pub chunk_size: usize,
    pub height_samples: Vec<f32>,
    pub material_layer_ids: Vec<u8>,
    pub layer_weights: Vec<[f32; 4]>,
}

pub struct TerrainPreviewCache {
    data: HashMap<ChunkId, TerrainChunkPreview>,
    dirty: bool,
}

impl Default for TerrainPreviewCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainPreviewCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            dirty: true,
        }
    }
    pub fn test_insert(&mut self, key: ChunkId, value: TerrainChunkPreview) {
        self.data.insert(key, value);
    }
    pub fn test_mark_clean(&mut self) {
        self.dirty = false;
    }
    pub fn test_clear(&mut self) {
        self.data.clear();
        self.dirty = true;
    }
    pub fn get(&self, key: &ChunkId) -> Option<&TerrainChunkPreview> {
        self.data.get(key)
    }
    pub fn get_preview_data(&self, key: &ChunkId) -> Option<&TerrainChunkPreview> {
        self.data.get(key)
    }
    pub fn invalidate_key(&mut self, key: &ChunkId) {
        self.data.remove(key);
        self.dirty = true;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn entry_count(&self) -> usize {
        self.data.len()
    }
    pub fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
    pub fn rebuild_from_owner(&mut self, owner: &TerrainOwnerData) {
        let [gx, gy] = owner.chunk_grid;
        for cy in 0..gy {
            for cx in 0..gx {
                let id = ChunkId {
                    x: cx as i32,
                    y: cy as i32,
                };
                self.data.insert(
                    id,
                    TerrainChunkPreview {
                        chunk_id: id,
                        heightmap_summary: vec![],
                        texture_layers: vec!["default".to_string()],
                    },
                );
            }
        }
        self.dirty = false;
    }
}

impl RebuildableCache<ChunkId> for TerrainPreviewCache {
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn entry_count(&self) -> usize {
        self.data.len()
    }
    fn invalidate_key(&mut self, key: &ChunkId) {
        self.data.remove(key);
        self.dirty = true;
    }
    fn rebuild_if_needed(&mut self) {
        self.dirty = false;
    }
}

impl CacheEntry for TerrainPreviewCache {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_valid(&self) -> bool {
        !self.dirty
    }

    fn invalidate(&mut self) {
        self.dirty = true;
    }

    fn rebuild(&mut self, state_system: &crate::StateContainerSystem) {
        let Some(world_state) = state_system.world_state.as_ref() else {
            self.data.clear();
            self.dirty = false;
            return;
        };

        let world_state = world_state.lock().unwrap();
        let Some(terrain) = world_state.terrain_state.as_ref() else {
            self.data.clear();
            self.dirty = false;
            return;
        };

        let grid_x = (((terrain.resolution.0 as usize).saturating_add(255)) / 256).max(1);
        let grid_y = (((terrain.resolution.1 as usize).saturating_add(255)) / 256).max(1);

        self.data.clear();
        self.rebuild_from_owner(&TerrainOwnerData {
            resolution: [terrain.resolution.0 as usize, terrain.resolution.1 as usize],
            world_size: [terrain.bounds.0, terrain.bounds.1],
            chunk_grid: [grid_x, grid_y],
            chunk_size: 256,
            height_samples: Vec::new(),
            material_layer_ids: Vec::new(),
            layer_weights: Vec::new(),
        });
    }

    fn dependencies(&self) -> Vec<crate::StateId> {
        vec![crate::StateId::WorldState, crate::StateId::TerrainState]
    }
}
