// Integration tests for save/load roundtrip
use editor_dto_law::StableWorldId;
use engine_material::SkyWeatherState;
use engine_world::{
    EntityId, TerrainChunk, TerrainLayerMaterialState, TerrainPatchState, ProofRegionScene,
    WorldState,
};
use std::fs;
use tempfile::TempDir;
use uuid::Uuid;

#[test]
fn test_save_load_roundtrip_with_terrain() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("roundtrip_world");

    // Create world with specific terrain data
    let mut world = WorldState::new();
    let terrain_entity = EntityId(1);

    // Create terrain with pattern
    let mut height_samples = vec![0.0; 128 * 128];
    for (i, sample) in height_samples.iter_mut().enumerate() {
        *sample = (i % 50) as f32 + 10.0;
    }

    let terrain = TerrainPatchState {
        entity_id: terrain_entity,
        origin: [100.0, 0.0, 200.0],
        world_size: [512.0, 512.0],
        resolution: [128, 128],
        chunk_grid: [2, 2],
        chunk_size: 64,
        height_source_ref: None,
        height_samples,
        material_layer_ids: vec![0],
        layer_weights: vec![[0.7, 0.3, 0.0, 0.0]; 128 * 128],
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
        chunks: vec![
            TerrainChunk {
                chunk_x: 0,
                chunk_y: 0,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
            TerrainChunk {
                chunk_x: 1,
                chunk_y: 0,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
            TerrainChunk {
                chunk_x: 0,
                chunk_y: 1,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
            TerrainChunk {
                chunk_x: 1,
                chunk_y: 1,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
        ],
        dirty_regions: vec![],
        mesh_revision: 1,
        collision_revision: 1,
    };

    let mut sky = SkyWeatherState::new_default();
    sky.celestial.time_of_day_hours = 16.5;
    sky.celestial.day_of_year = 200;
    sky.celestial.latitude_deg = 55.0;
    sky.cloud_profile.coverage = 0.6;
    sky.atmosphere.fog_density = 0.2;

    let scene = ProofRegionScene {
        scene_name: "Roundtrip Test".to_string(),
        terrain,
        wall: engine_world::WallState {
            entity_id: EntityId(2),
            position: [0.0, 0.0, 0.0],
            dimensions: [0.0, 0.0, 0.0],
            stack_id: engine_world::MaterialStackId(0),
        },
        weapon: engine_world::WeaponState {
            entity_id: EntityId(3),
            profile_id: engine_world::WeaponProfileId(0),
            position: [0.0, 0.0, 0.0],
            aim_direction: [0.0, 0.0, 1.0],
        },
        camera: engine_world::CameraState {
            position: [0.0, 50.0, -100.0],
            look_at: [0.0, 0.0, 0.0],
            fov_deg: 60.0,
        },
        sky,
        sky_bundle_path: Some("shared/sky/sky_bundle.json".to_string()),
    };

    world.set_proof_region_scene(scene);

    // Save
    let world_ref = StableWorldId(Uuid::new_v4());
    let lifecycle = stratumx_editor_l8_0_editor_shell::WorldLifecycleManager::new();
    lifecycle
        .save_world_to_path(&world, world_ref, &world_path)
        .unwrap();

    // Verify files exist
    assert!(world_path.join("world.json").exists());
    assert!(world_path.join("environment/sky_binding.json").exists());
    assert!(world_path.join("terrain/terrain_manifest.json").exists());
    assert!(world_path.join("terrain/chunks/chunk_0_0.bin").exists());
    assert!(world_path.join("terrain/chunks/chunk_1_1.bin").exists());

    // Load back
    let mut lifecycle2 = stratumx_editor_l8_0_editor_shell::WorldLifecycleManager::new();
    let result = lifecycle2.open_world_from_path(&world_path);

    assert!(result.accepted);
    assert_eq!(result.world_label, Some("Roundtrip Test".to_string()));

    // Verify loaded world
    let loaded_world = lifecycle2.get_world_state().unwrap();
    let loaded_scene = loaded_world.vertical_slice_scene().unwrap();

    // Verify terrain
    assert_eq!(loaded_scene.terrain.origin, [100.0, 0.0, 200.0]);
    assert_eq!(loaded_scene.terrain.world_size, [512.0, 512.0]);
    assert_eq!(loaded_scene.terrain.resolution, [128, 128]);
    assert_eq!(loaded_scene.terrain.chunk_grid, [2, 2]);
    assert_eq!(loaded_scene.terrain.chunks.len(), 4);

    // Verify height data preserved
    assert_eq!(loaded_scene.terrain.height_samples.len(), 128 * 128);
    println!(
        "First 10 heights: {:?}",
        &loaded_scene.terrain.height_samples[0..10]
    );
    println!(
        "Chunks loaded: {:?}",
        loaded_scene
            .terrain
            .chunks
            .iter()
            .map(|c| (c.chunk_x, c.chunk_y, c.loaded))
            .collect::<Vec<_>>()
    );
    assert_eq!(loaded_scene.terrain.height_samples[0], 10.0);
    assert_eq!(loaded_scene.terrain.height_samples[25], 35.0);
    assert_eq!(loaded_scene.terrain.height_samples[100], 10.0);

    // Verify material weights preserved
    assert_eq!(loaded_scene.terrain.layer_weights[0], [0.7, 0.3, 0.0, 0.0]);

    // Verify sky
    assert_eq!(loaded_scene.sky.celestial.time_of_day_hours, 16.5);
    assert_eq!(loaded_scene.sky.celestial.day_of_year, 200);
    assert_eq!(loaded_scene.sky.celestial.latitude_deg, 55.0);
    assert_eq!(loaded_scene.sky.cloud_profile.coverage, 0.6);
    assert_eq!(loaded_scene.sky.atmosphere.fog_density, 0.2);
}

#[test]
fn test_chunk_binary_format() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("chunk_format_test");

    // Create world with specific chunk pattern
    let mut world = WorldState::new();
    let chunk_res = 64;
    let mut height_samples = vec![0.0; 128 * 128];

    // Set specific pattern in first chunk (0,0)
    for y in 0..chunk_res {
        for x in 0..chunk_res {
            let idx = y * 128 + x;
            height_samples[idx] = (x + y * 2) as f32;
        }
    }

    let terrain = TerrainPatchState {
        entity_id: EntityId(1),
        origin: [0.0, 0.0, 0.0],
        world_size: [256.0, 256.0],
        resolution: [128, 128],
        chunk_grid: [2, 2],
        chunk_size: chunk_res as u32,
        height_source_ref: None,
        height_samples,
        material_layer_ids: vec![0],
        layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 128 * 128],
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
        chunks: vec![
            TerrainChunk {
                chunk_x: 0,
                chunk_y: 0,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
            TerrainChunk {
                chunk_x: 1,
                chunk_y: 0,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
            TerrainChunk {
                chunk_x: 0,
                chunk_y: 1,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
            TerrainChunk {
                chunk_x: 1,
                chunk_y: 1,
                data_file: None,
                loaded: true,
                dirty: false,
                mesh_built: false,
            },
        ],
        dirty_regions: vec![],
        mesh_revision: 1,
        collision_revision: 1,
    };

    let scene = ProofRegionScene {
        scene_name: "Chunk Format Test".to_string(),
        terrain,
        wall: engine_world::WallState {
            entity_id: EntityId(2),
            position: [0.0, 0.0, 0.0],
            dimensions: [0.0, 0.0, 0.0],
            stack_id: engine_world::MaterialStackId(0),
        },
        weapon: engine_world::WeaponState {
            entity_id: EntityId(3),
            profile_id: engine_world::WeaponProfileId(0),
            position: [0.0, 0.0, 0.0],
            aim_direction: [0.0, 0.0, 1.0],
        },
        camera: engine_world::CameraState {
            position: [0.0, 50.0, -100.0],
            look_at: [0.0, 0.0, 0.0],
            fov_deg: 60.0,
        },
        sky: SkyWeatherState::new_default(),
        sky_bundle_path: Some("shared/sky/sky_bundle.json".to_string()),
    };

    world.set_proof_region_scene(scene);

    // Save
    let world_ref = StableWorldId(Uuid::new_v4());
    let lifecycle = stratumx_editor_l8_0_editor_shell::WorldLifecycleManager::new();
    lifecycle
        .save_world_to_path(&world, world_ref, &world_path)
        .unwrap();

    // Verify chunk file exists and has data
    let chunk_path = world_path.join("terrain/chunks/chunk_0_0.bin");
    assert!(chunk_path.exists());

    let chunk_bytes = fs::read(&chunk_path).unwrap();
    assert!(!chunk_bytes.is_empty());

    // Load and verify chunk data
    let chunk_data = editor_dto_law::ChunkData::from_bytes(&chunk_bytes).unwrap();
    assert_eq!(chunk_data.heights.len(), chunk_res * chunk_res);

    // Verify pattern preserved
    println!("Chunk heights[0..10]: {:?}", &chunk_data.heights[0..10]);
    println!(
        "Expected: x=0,y=0 -> 0, x=1,y=0 -> 1, x=0,y=1 (idx={}) -> {}",
        chunk_res, 2
    );
    assert_eq!(chunk_data.heights[0], 0.0); // x=0, y=0
    assert_eq!(chunk_data.heights[1], 1.0); // x=1, y=0
    assert_eq!(chunk_data.heights[chunk_res], 2.0); // x=0, y=1 -> 0 + 1*2
}

#[test]
fn test_empty_world_save_load() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("empty_world");

    // Create minimal world
    let mut world = WorldState::new();
    let scene = ProofRegionScene {
        scene_name: "Empty World".to_string(),
        terrain: TerrainPatchState {
            entity_id: EntityId(1),
            origin: [0.0, 0.0, 0.0],
            world_size: [1000.0, 1000.0],
            resolution: [256, 256],
            chunk_grid: [4, 4],
            chunk_size: 64,
            height_source_ref: None,
            height_samples: vec![0.0; 256 * 256],
            material_layer_ids: vec![0],
            layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 256 * 256],
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
            mesh_revision: 1,
            collision_revision: 1,
        },
        wall: engine_world::WallState {
            entity_id: EntityId(2),
            position: [0.0, 0.0, 0.0],
            dimensions: [0.0, 0.0, 0.0],
            stack_id: engine_world::MaterialStackId(0),
        },
        weapon: engine_world::WeaponState {
            entity_id: EntityId(3),
            profile_id: engine_world::WeaponProfileId(0),
            position: [0.0, 0.0, 0.0],
            aim_direction: [0.0, 0.0, 1.0],
        },
        camera: engine_world::CameraState {
            position: [0.0, 50.0, -100.0],
            look_at: [0.0, 0.0, 0.0],
            fov_deg: 60.0,
        },
        sky: SkyWeatherState::new_default(),
        sky_bundle_path: Some("shared/sky/sky_bundle.json".to_string()),
    };

    world.set_proof_region_scene(scene);

    // Save
    let world_ref = StableWorldId(Uuid::new_v4());
    let lifecycle = stratumx_editor_l8_0_editor_shell::WorldLifecycleManager::new();
    lifecycle
        .save_world_to_path(&world, world_ref, &world_path)
        .unwrap();

    // Load
    let mut lifecycle2 = stratumx_editor_l8_0_editor_shell::WorldLifecycleManager::new();
    let result = lifecycle2.open_world_from_path(&world_path);

    assert!(result.accepted);
    assert_eq!(result.world_label, Some("Empty World".to_string()));
}
