use std::fs;

use editor_dto_law::StableWorldId;
use engine_material::SkyWeatherState;
use engine_world::{
    CameraState, EntityId, MaterialStackId, TerrainLayerMaterialState, TerrainPatchState,
    VerticalSliceScene, WallState, WeaponProfileId, WeaponState, WorldState,
};
use tempfile::tempdir;
use uuid::Uuid;

use stratumx_editor_l9_2_terrain_landscape_authoring_suite::{
    PaintOperation, SculptOperation, TerrainAuthoringService,
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

#[test]
fn terrain_lane_bind_and_sync_reports_bound_state() {
    let mut service = TerrainAuthoringService::new();
    let mut world = create_world_with_scene();
    let world_ref = StableWorldId(Uuid::new_v4());

    service
        .bind_terrain(world_ref, "proof_region")
        .expect("bind terrain");
    let report = service.sync_terrain(&mut world).expect("sync terrain");

    assert!(report.bound);
    assert_eq!(service.bound_scene_label(), Some("proof_region"));
    assert!(service.get_manifest().is_some());
}

#[test]
fn terrain_lane_sculpt_and_paint_mark_dirty_and_update_world() {
    let mut service = TerrainAuthoringService::new();
    let mut world = create_world_with_scene();

    service
        .sculpt_terrain(
            &mut world,
            SculptOperation::Raise {
                center: [8.0, 8.0],
                radius: 4.0,
                strength: 0.5,
            },
        )
        .expect("sculpt terrain");
    service
        .paint_terrain(
            &mut world,
            PaintOperation {
                center: [8.0, 8.0],
                radius: 4.0,
                layer_index: 0,
                strength: 0.25,
            },
        )
        .expect("paint terrain");

    let scene = world.vertical_slice_scene().expect("scene");
    assert!(scene
        .terrain
        .height_samples
        .iter()
        .any(|sample| *sample > 0.0));
    assert!(!service.get_dirty_chunks().is_empty());
}

#[test]
fn terrain_lane_imports_raw_heightmap_and_rebuilds() {
    let temp = tempdir().expect("tempdir");
    let raw_path = temp.path().join("terrain.raw");
    fs::write(&raw_path, [0u8, 64u8, 128u8, 255u8]).expect("write raw");

    let mut service = TerrainAuthoringService::new();
    let mut world = create_world_with_scene();
    service
        .bind_terrain(StableWorldId(Uuid::new_v4()), "proof_region")
        .expect("bind terrain");

    let format = service
        .import_heightmap(&mut world, &raw_path)
        .expect("import heightmap");
    assert_eq!(
        format,
        stratumx_editor_l9_2_terrain_landscape_authoring_suite::HeightmapFormat::Raw
    );

    service
        .rebuild_terrain(&mut world)
        .expect("rebuild terrain");
    let report = service.sync_terrain(&mut world).expect("sync terrain");

    assert!(report.gpu_sync_required);
    assert_eq!(
        world
            .vertical_slice_scene()
            .expect("scene")
            .terrain
            .resolution,
        [2, 2]
    );
}
