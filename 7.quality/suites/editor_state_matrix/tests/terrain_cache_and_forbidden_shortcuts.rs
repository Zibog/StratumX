//! Tests for terrain preview cache.
//!
//! Moved from inline #[cfg(test)] module to satisfy the no-heavy-inline-tests rule.

use stratumx_editor_state_containers::cache::{
    ChunkId, RebuildableCache, TerrainOwnerData, TerrainPreviewCache,
};

#[test]
fn test_terrain_cache_invalidate() {
    let mut cache = TerrainPreviewCache::new();
    let id = ChunkId { x: 0, y: 0 };
    cache.test_insert(
        id,
        stratumx_editor_state_containers::cache::TerrainChunkPreview {
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
            let id = ChunkId {
                x: x as i32,
                y: y as i32,
            };
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
    assert_eq!(summary.len(), 4);
    assert!((summary[0] - 0.0).abs() < 0.001);
    assert!((summary[1] - 15.0).abs() < 0.001);
    assert!((summary[2] - 7.5).abs() < 0.001);
}
