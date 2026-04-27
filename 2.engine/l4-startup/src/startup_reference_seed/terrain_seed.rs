// Terrain Seed

use super::seed_ids;
use engine_world::{EntityId, TerrainChunk, TerrainLayerMaterialState, TerrainPatchState};

pub fn create_terrain_patch(terrain_entity: EntityId) -> TerrainPatchState {
    TerrainPatchState {
        entity_id: terrain_entity,
        origin: [0.0, 0.0, 0.0],
        world_size: [1000.0, 1000.0],
        resolution: [256, 256],
        chunk_grid: [4, 4],
        chunk_size: 64,
        height_source_ref: None,
        height_samples: vec![0.0; 256 * 256],
        material_layer_ids: seed_ids::default_terrain_material_layers(),
        layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 256 * 256],
        layer_materials: vec![
            TerrainLayerMaterialState {
                layer_id: 0,
                material_family: "terrain.soil".to_string(),
                albedo_texture_ref: None,
                normal_texture_ref: None,
                orm_texture_ref: None,
                uv_scale: [8.0, 8.0],
                base_tint: [1.0, 1.0, 1.0, 1.0],
            },
            TerrainLayerMaterialState {
                layer_id: 1,
                material_family: "terrain.grass".to_string(),
                albedo_texture_ref: None,
                normal_texture_ref: None,
                orm_texture_ref: None,
                uv_scale: [8.0, 8.0],
                base_tint: [1.0, 1.0, 1.0, 1.0],
            },
        ],
        hole_mask: None,
        chunks: (0..16)
            .map(|i| TerrainChunk {
                chunk_x: i % 4,
                chunk_y: i / 4,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: true,
            })
            .collect(),
        dirty_regions: vec![],
        mesh_revision: 1,
        collision_revision: 1,
    }
}
