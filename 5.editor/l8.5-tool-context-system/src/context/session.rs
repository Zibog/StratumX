//! Editor session helpers for a single active world.

use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use editor_dto_law::StableWorldId;
use engine_world::{
    CameraState, EntityId, TerrainChunk, TerrainPatchState, VerticalSliceScene, WallState,
    WeaponState, WorldState,
};
use uuid::Uuid;

/// Runtime mode for the editor session
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeMode {
    Editing,
    Playing,
    Simulating,
}

pub struct EditorSession {
    pub world_ref: StableWorldId,
    pub world: WorldState,
    pub world_label: String,
    pub world_path: Option<PathBuf>,
    /// **PHASE 7 REMEDIATED**: Runtime mode tracking on the session.
    pub runtime_mode: RuntimeMode,
}

impl EditorSession {
    pub fn startup_world() -> Self {
        Self::new(
            stable_world_id_from_seed("startup_world"),
            "Startup World".to_string(),
            None,
            default_world_state("Startup World", [256, 256], [1000.0, 1000.0], [4, 4]),
            RuntimeMode::Editing,
        )
    }

    pub fn world_package(
        world_path: &Path,
        world_label: String,
        resolution: [u32; 2],
        world_size: [f32; 2],
        chunk_grid: [u32; 2],
        time_of_day_hours: f32,
        weather_regime: engine_material::WeatherRegime,
        cloud_coverage: f32,
    ) -> Self {
        let mut world = default_world_state(&world_label, resolution, world_size, chunk_grid);
        if let Some(scene) = world.vertical_slice_scene_mut() {
            scene.sky.set_time_of_day(time_of_day_hours);
            scene.sky.set_weather_regime(weather_regime);
            scene.sky.set_cloud_coverage(cloud_coverage);
        }

        Self::new(
            stable_world_id_from_seed(&world_path.to_string_lossy()),
            world_label,
            Some(world_path.to_path_buf()),
            world,
            RuntimeMode::Editing,
        )
    }

    fn new(
        world_ref: StableWorldId,
        world_label: String,
        world_path: Option<PathBuf>,
        world: WorldState,
        runtime_mode: RuntimeMode,
    ) -> Self {
        Self {
            world_ref,
            world,
            world_label,
            world_path,
            runtime_mode,
        }
    }
}

fn stable_world_id_from_seed(seed: &str) -> StableWorldId {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish() as u128;
    StableWorldId(Uuid::from_u128(hash | (0x53545241u128 << 96)))
}

fn default_world_state(
    label: &str,
    resolution: [u32; 2],
    world_size: [f32; 2],
    chunk_grid: [u32; 2],
) -> WorldState {
    let mut world = WorldState::new();
    let scene = VerticalSliceScene {
        scene_name: label.to_string(),
        terrain: TerrainPatchState {
            entity_id: EntityId(1),
            origin: [0.0, 0.0, 0.0],
            world_size,
            resolution,
            chunk_grid,
            chunk_size: 64,
            height_source_ref: None,
            height_samples: vec![0.0; (resolution[0] * resolution[1]) as usize],
            material_layer_ids: vec![1; (resolution[0] * resolution[1]) as usize],
            layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; (resolution[0] * resolution[1]) as usize],
            layer_materials: vec![engine_world::TerrainLayerMaterialState {
                layer_id: 0,
                material_family: "terrain.soil".to_string(),
                albedo_texture_ref: None,
                normal_texture_ref: None,
                orm_texture_ref: None,
                uv_scale: [8.0, 8.0],
                base_tint: [1.0, 1.0, 1.0, 1.0],
            }],
            hole_mask: None,
            chunks: (0..chunk_grid[1])
                .flat_map(|chunk_y| {
                    (0..chunk_grid[0]).map(move |chunk_x| TerrainChunk {
                        chunk_x,
                        chunk_y,
                        data_file: None,
                        loaded: true,
                        dirty: false,
                        mesh_built: true,
                    })
                })
                .collect(),
            dirty_regions: Vec::new(),
            mesh_revision: 1,
            collision_revision: 1,
        },
        wall: WallState {
            entity_id: EntityId(2),
            position: [0.0, 0.0, 0.0],
            dimensions: [0.0, 0.0, 0.0],
            stack_id: engine_world::MaterialStackId(1),
        },
        weapon: WeaponState {
            entity_id: EntityId(3),
            profile_id: engine_world::WeaponProfileId(1),
            position: [0.0, 0.0, 0.0],
            aim_direction: [0.0, 0.0, 1.0],
        },
        camera: CameraState {
            position: [0.0, 50.0, -100.0],
            look_at: [0.0, 0.0, 0.0],
            fov_deg: 60.0,
        },
        sky: engine_material::SkyWeatherState::new_default(),
        sky_bundle_path: Some("shared/sky/sky_bundle.json".to_string()),
    };

    world.set_vertical_slice_scene(scene);
    world
}
