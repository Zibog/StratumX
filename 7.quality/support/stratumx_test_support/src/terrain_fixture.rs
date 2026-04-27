//! Terrain fixture for testing

use engine_world::{EntityId, TerrainLayerMaterialState, TerrainPatchState};

/// Creates a minimal terrain patch for testing
pub fn create_test_terrain() -> TerrainPatchState {
    TerrainPatchState {
        entity_id: EntityId(1),
        origin: [0.0, 0.0, 0.0],
        world_size: [100.0, 100.0],
        resolution: [64, 64],
        chunk_grid: [4, 4],
        chunk_size: 16,
        height_source_ref: None,
        height_samples: vec![0.0; 64 * 64],
        material_layer_ids: vec![0; 64 * 64],
        layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 64 * 64],
        layer_materials: vec![TerrainLayerMaterialState {
            layer_id: 0,
            material_family: "terrain.soil".to_string(),
            albedo_texture_ref: None,
            normal_texture_ref: None,
            orm_texture_ref: None,
            uv_scale: [8.0, 8.0],
            base_tint: [1.0, 1.0, 1.0, 1.0],
        }],
        hole_mask: None,
        chunks: vec![],
        dirty_regions: vec![],
        mesh_revision: 0,
        collision_revision: 0,
    }
}

/// Creates a terrain patch with given dimensions
pub fn create_terrain_with_size(width: u32, depth: u32) -> TerrainPatchState {
    let mut terrain = create_test_terrain();
    terrain.world_size = [width as f32, depth as f32];
    terrain
}
