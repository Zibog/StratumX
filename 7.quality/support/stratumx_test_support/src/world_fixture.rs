//! World fixture for testing

use editor_dto_law::StableWorldId;
use engine_material::SkyWeatherState;
use engine_world::{
    CameraState, EntityId, MaterialStackId, TerrainLayerMaterialState, TerrainPatchState,
    VerticalSliceScene, WallState, WeaponProfileId, WeaponState, WorldState,
};
use uuid::Uuid;

/// Creates a minimal world state for testing
pub fn create_test_world() -> WorldState {
    WorldState::new()
}

/// Creates a world with a vertical slice scene
pub fn create_world_with_scene() -> WorldState {
    let mut world = WorldState::new();
    world.set_vertical_slice_scene(VerticalSliceScene {
        scene_name: "Test Scene".to_string(),
        terrain: TerrainPatchState {
            entity_id: EntityId(1),
            origin: [0.0, 0.0, 0.0],
            world_size: [16.0, 16.0],
            resolution: [1, 1],
            chunk_grid: [1, 1],
            chunk_size: 16,
            height_source_ref: None,
            height_samples: vec![0.0],
            material_layer_ids: vec![1],
            layer_weights: vec![[1.0, 0.0, 0.0, 0.0]],
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
            chunks: Vec::new(),
            dirty_regions: Vec::new(),
            mesh_revision: 1,
            collision_revision: 1,
        },
        wall: WallState {
            entity_id: EntityId(2),
            position: [0.0, 0.0, 4.0],
            dimensions: [2.0, 2.0, 0.25],
            stack_id: MaterialStackId(1),
        },
        weapon: WeaponState {
            entity_id: EntityId(3),
            profile_id: WeaponProfileId(1),
            position: [0.0, 1.0, 0.0],
            aim_direction: [0.0, 0.0, 1.0],
        },
        camera: CameraState {
            position: [0.0, 2.0, -4.0],
            look_at: [0.0, 0.0, 0.0],
            fov_deg: 60.0,
        },
        sky: SkyWeatherState::new_default(),
        sky_bundle_path: None,
    });
    world
}

/// Get test world ID
pub fn test_world_id() -> StableWorldId {
    StableWorldId(Uuid::from_u128(0x0000_0000_0000_0001))
}
