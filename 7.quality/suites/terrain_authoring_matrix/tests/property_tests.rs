use editor_dto_law::StableWorldId;
use engine_material::SkyWeatherState;
use engine_world::{
    CameraState, EntityId, MaterialStackId, TerrainLayerMaterialState, TerrainPatchState,
    VerticalSliceScene, WallState, WeaponProfileId, WeaponState, WorldState,
};
use proptest::prelude::*;
use uuid::Uuid;

use stratumx_editor_l9_2_terrain_landscape_authoring_suite::{
    SculptOperation, TerrainAuthoringService,
};

fn create_world_with_scene() -> WorldState {
    let mut world = WorldState::new();
    world.set_vertical_slice_scene(VerticalSliceScene {
        scene_name: "Proof Region".to_string(),
        terrain: TerrainPatchState {
            entity_id: EntityId(1),
            origin: [0.0, 0.0, 0.0],
            world_size: [16.0, 16.0],
            resolution: [16, 16],
            chunk_grid: [1, 1],
            chunk_size: 16,
            height_source_ref: None,
            height_samples: vec![0.0; 16 * 16],
            material_layer_ids: vec![0; 16 * 16],
            layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 16 * 16],
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn prop_sculpt_operations_mark_dirty_chunks(
        center_x in 0.0f32..16.0f32,
        center_y in 0.0f32..16.0f32,
        radius in 0.5f32..8.0f32,
        strength in 0.05f32..1.0f32,
        target_height in 0.0f32..100.0f32,
        mode in 0u8..4u8,
    ) {
        let mut service = TerrainAuthoringService::new();
        let mut world = create_world_with_scene();
        service.bind_terrain(StableWorldId(Uuid::new_v4()), "proof_region").expect("bind terrain");

        let operation = match mode {
            0 => SculptOperation::Raise { center: [center_x, center_y], radius, strength },
            1 => SculptOperation::Lower { center: [center_x, center_y], radius, strength },
            2 => SculptOperation::Smooth { center: [center_x, center_y], radius, strength },
            _ => SculptOperation::Flatten {
                center: [center_x, center_y],
                radius,
                strength,
                target_height,
            },
        };

        service.sculpt_terrain(&mut world, operation).expect("sculpt terrain");
        prop_assert!(!service.get_dirty_chunks().is_empty());
    }
}
