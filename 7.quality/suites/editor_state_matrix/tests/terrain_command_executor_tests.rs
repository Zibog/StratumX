use std::fs;

use engine_material::SkyWeatherState;
use engine_world::{
    CameraState, EntityId, MaterialStackId, TerrainLayerMaterialState, TerrainPatchState,
    ProofRegionScene, WallState, WeaponProfileId, WeaponState, WorldState,
};
use stratumx_editor_state_containers::runtime::terrain_command_executor::RuntimeHostAccess;
use stratumx_editor_state_containers::runtime::terrain_command_executor::TerrainCommandExecutor;
use tempfile::tempdir;

struct MockRuntimeHost {
    world: WorldState,
    terrain_gpu_dirty: bool,
}

impl RuntimeHostAccess for MockRuntimeHost {
    fn get_world_state_mut(&mut self) -> Option<&mut WorldState> {
        Some(&mut self.world)
    }

    fn mark_terrain_gpu_dirty(&mut self) {
        self.terrain_gpu_dirty = true;
    }
}

fn create_host() -> MockRuntimeHost {
    let mut world = WorldState::new();
    world.set_proof_region_scene(ProofRegionScene {
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

    MockRuntimeHost {
        world,
        terrain_gpu_dirty: false,
    }
}

#[test]
fn terrain_command_executor_imports_square_raw_and_r16_heightmaps() {
    let dir = tempdir().unwrap();
    let raw_path = dir.path().join("terrain.raw");
    let r16_path = dir.path().join("terrain.r16");
    fs::write(&raw_path, vec![0u8; 256 * 256]).unwrap();
    fs::write(&r16_path, vec![0u8; 256 * 256 * 2]).unwrap();

    let mut host = create_host();
    let mut executor = TerrainCommandExecutor::new(MockRuntimeHost {
        world: WorldState::new(),
        terrain_gpu_dirty: false,
    });

    assert!(executor.import_heightmap(&mut host, &raw_path).is_ok());
    assert!(executor.import_heightmap(&mut host, &r16_path).is_ok());
    assert!(host.terrain_gpu_dirty);
    assert_eq!(
        host.world
            .proof_region_scene()
            .unwrap()
            .terrain
            .resolution,
        [256, 256]
    );
}

#[test]
fn terrain_command_executor_rejects_invalid_raw_and_r16_inputs() {
    let dir = tempdir().unwrap();
    let raw_path = dir.path().join("bad.raw");
    let r16_path = dir.path().join("bad.r16");
    fs::write(&raw_path, vec![0u8; 99]).unwrap();
    fs::write(&r16_path, vec![0u8; 255]).unwrap();

    let mut host = create_host();
    let mut executor = TerrainCommandExecutor::new(MockRuntimeHost {
        world: WorldState::new(),
        terrain_gpu_dirty: false,
    });

    assert!(executor.import_heightmap(&mut host, &raw_path).is_err());
    assert!(executor.import_heightmap(&mut host, &r16_path).is_err());
}
