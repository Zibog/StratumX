//! Tests for terrain preview cache.
//!
//! Note: Types are locally stubbed because stratumx_editor_state_containers
//! is a FUTURE_STUB crate not yet integrated into the product spine.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct ChunkId {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct TerrainChunkPreview {
    chunk_id: ChunkId,
    heightmap_summary: Vec<f32>,
    texture_layers: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TerrainOwnerData {
    resolution: [usize; 2],
    world_size: [f32; 2],
    chunk_grid: [usize; 2],
    chunk_size: usize,
    height_samples: Vec<f32>,
    material_layer_ids: Vec<u8>,
    layer_weights: Vec<[f32; 4]>,
}

struct TerrainPreviewCache {
    data: HashMap<ChunkId, TerrainChunkPreview>,
    dirty: bool,
}

impl TerrainPreviewCache {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            dirty: false,
        }
    }

    fn test_insert(&mut self, key: ChunkId, value: TerrainChunkPreview) {
        self.data.insert(key, value);
    }

    fn test_mark_clean(&mut self) {
        self.dirty = false;
    }

    fn invalidate_key(&mut self, key: &ChunkId) {
        self.data.remove(key);
        self.dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn get(&self, key: &ChunkId) -> Option<&TerrainChunkPreview> {
        self.data.get(key)
    }

    fn entry_count(&self) -> usize {
        self.data.len()
    }

    fn rebuild_from_owner(&mut self, owner: &TerrainOwnerData) {
        let [gx, gy] = owner.chunk_grid;
        let cs = owner.chunk_size;
        for cy in 0..gy {
            for cx in 0..gx {
                let id = ChunkId {
                    x: cx as i32,
                    y: cy as i32,
                };
                let mut summary = Vec::new();
                let row_start = cy * cs;
                let col_start = cx * cs;
                if row_start < owner.resolution[1] && col_start < owner.resolution[0] {
                    let end_y = (row_start + cs).min(owner.resolution[1]);
                    let end_x = (col_start + cs).min(owner.resolution[0]);
                    let mut min_h = f32::MAX;
                    let mut max_h = f32::MIN;
                    let mut sum_h = 0.0;
                    let mut count = 0;
                    for y in row_start..end_y {
                        for x in col_start..end_x {
                            let idx = y * owner.resolution[0] + x;
                            let h = owner.height_samples[idx];
                            if h < min_h {
                                min_h = h;
                            }
                            if h > max_h {
                                max_h = h;
                            }
                            sum_h += h;
                            count += 1;
                        }
                    }
                    if count > 0 {
                        summary.push(min_h);
                        summary.push(max_h);
                        summary.push(sum_h / count as f32);
                    }
                }
                self.data.insert(
                    id,
                    TerrainChunkPreview {
                        chunk_id: id,
                        heightmap_summary: summary,
                        texture_layers: vec!["default".to_string()],
                    },
                );
            }
        }
        self.dirty = false;
    }
}

#[test]
fn test_terrain_cache_invalidate() {
    let mut cache = TerrainPreviewCache::new();
    let id = ChunkId { x: 0, y: 0 };
    cache.test_insert(
        id,
        TerrainChunkPreview {
            chunk_id: id,
            heightmap_summary: vec![0.0, 1.0, 2.0],
            texture_layers: vec!["grass".to_string()],
        },
    );
    cache.test_mark_clean();
    cache.invalidate_key(&id);
    assert!(cache.is_dirty());
    assert!(cache.get(&id).is_none());
}

#[test]
fn test_rebuild_from_owner_populates_chunks() {
    let owner_data = TerrainOwnerData {
        resolution: [64, 64],
        world_size: [1000.0, 1000.0],
        chunk_grid: [2, 2],
        chunk_size: 32,
        height_samples: vec![0.0; 64 * 64],
        material_layer_ids: vec![1; 64 * 64],
        layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 64 * 64],
    };
    let mut cache = TerrainPreviewCache::new();
    cache.rebuild_from_owner(&owner_data);
    assert!(!cache.is_dirty());
    assert_eq!(cache.entry_count(), 4);
    for y in 0..2 {
        for x in 0..2 {
            let id = ChunkId { x, y };
            assert!(cache.get(&id).is_some(), "Missing chunk ({}, {})", x, y);
        }
    }
}

#[test]
fn test_heightmap_summary_computes_stats() {
    let owner_data = TerrainOwnerData {
        resolution: [4, 4],
        world_size: [100.0, 100.0],
        chunk_grid: [1, 1],
        chunk_size: 4,
        height_samples: (0..16).map(|v| v as f32).collect(),
        material_layer_ids: vec![1; 16],
        layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 16],
    };
    let mut cache = TerrainPreviewCache::new();
    cache.rebuild_from_owner(&owner_data);
    let chunk = cache.get(&ChunkId { x: 0, y: 0 }).unwrap();
    let summary = &chunk.heightmap_summary;
    assert_eq!(summary.len(), 3);
    assert!((summary[0] - 0.0).abs() < 0.001);
    assert!((summary[1] - 15.0).abs() < 0.001);
    assert!((summary[2] - 7.5).abs() < 0.001);
}
