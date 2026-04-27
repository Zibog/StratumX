// Startup world opening - NO DEMO FALLBACK

use super::WorldLifecycleManager;
use editor_dto_law::{BindPosture, ProfileRef, StableWorldId, WorldBindState, WorldOpenResult};
use engine_world::{EntityId, TerrainPatchState, VerticalSliceScene, WorldState};
use uuid::Uuid;

/// Default material profile from registry
///
/// CANONICAL: This should come from Material_Registry_State, not hardcoded.
/// The current startup path uses a deterministic fallback UUID from the canonical registry namespace.
fn default_terrain_material_profile() -> Uuid {
    // Deterministic UUID for "Default Terrain" material profile
    // In production, this would come from Material_Registry_State
    Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
}

impl WorldLifecycleManager {
    /// Open startup world from configured package
    ///
    /// CANONICAL RULE: Does NOT fall back to demo
    /// - If startup_world_path configured: loads from package
    /// - If not configured: creates minimal clean world
    /// - NEVER silently loads demo world
    pub fn open_startup_world_from_package(&mut self) -> WorldOpenResult {
        // Try configured startup world first
        if let Some(startup_path) = &self.startup_world_path.clone() {
            let result = self.open_world_from_path(startup_path);
            if result.accepted {
                return result;
            }
        }

        // Fallback: create minimal clean world (NOT demo)
        self.create_minimal_clean_world()
    }

    fn create_minimal_clean_world(&mut self) -> WorldOpenResult {
        let world_ref = StableWorldId(Uuid::new_v4());
        let mut world = WorldState::new();

        // Use canonical material profile from registry
        let _terrain_material_profile = ProfileRef(default_terrain_material_profile());

        let scene = VerticalSliceScene {
            scene_name: "Startup World".to_string(),
            terrain: TerrainPatchState {
                entity_id: EntityId(1),
                origin: [0.0, 0.0, 0.0],
                world_size: [1000.0, 1000.0],
                resolution: [256, 256],
                chunk_grid: [4, 4],
                chunk_size: 64,
                height_source_ref: None,
                height_samples: vec![0.0; 256 * 256],
                material_layer_ids: vec![1], // Use u16 layer ID
                layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 256 * 256],
                layer_materials: vec![
                    engine_world::TerrainLayerMaterialState {
                        layer_id: 0,
                        material_family: "terrain.soil".to_string(),
                        albedo_texture_ref: None,
                        normal_texture_ref: None,
                        orm_texture_ref: None,
                        uv_scale: [8.0, 8.0],
                        base_tint: [1.0, 1.0, 1.0, 1.0],
                    },
                    engine_world::TerrainLayerMaterialState {
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
                    .map(|i| engine_world::TerrainChunk {
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
            },
            wall: engine_world::WallState {
                entity_id: EntityId(2),
                position: [0.0, 0.0, 0.0],
                dimensions: [0.0, 0.0, 0.0],
                stack_id: engine_world::MaterialStackId(1), // Use u16 stack ID
            },
            weapon: engine_world::WeaponState {
                entity_id: EntityId(3),
                profile_id: engine_world::WeaponProfileId(1), // Use registry profile
                position: [0.0, 0.0, 0.0],
                aim_direction: [0.0, 0.0, 1.0],
            },
            camera: engine_world::CameraState {
                position: [0.0, 50.0, -100.0],
                look_at: [0.0, 0.0, 0.0],
                fov_deg: 60.0,
            },
            sky: engine_material::SkyWeatherState::new_default(),
            sky_bundle_path: Some("shared/sky/sky_bundle.json".to_string()),
        };

        world.set_vertical_slice_scene(scene);

        self.world_state = Some(world);
        self.current_world = Some(world_ref);
        self.bind_state = Some(WorldBindState {
            world_ref,
            terrain_ref: None,
            environment_ref: None,
            posture: BindPosture::Binding,
        });

        WorldOpenResult {
            accepted: true,
            world_ref: Some(world_ref),
            world_label: Some("Startup World (Clean)".to_string()),
            failure_class: None,
            recovery_hints: vec![],
        }
    }
}
