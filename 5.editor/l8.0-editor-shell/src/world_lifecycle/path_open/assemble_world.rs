// World assembly from loaded components

use super::{load_environment, load_terrain};
use editor_dto_law::WorldPackageManifest;
use engine_world::{EntityId, ProofRegionScene, WorldState};
use std::path::Path;

pub(super) fn assemble_world_from_manifest(
    world_path: &Path,
    manifest: &WorldPackageManifest,
) -> Result<WorldState, String> {
    if manifest.version != "1.0.0" {
        return Err(format!("Unsupported world version: {}", manifest.version));
    }

    let mut world = WorldState::new();

    let terrain_state = load_terrain::load_terrain_from_package(world_path, manifest)?;
    let sky_state = load_environment::load_sky_from_package(world_path, manifest)?;

    let scene = ProofRegionScene {
        scene_name: manifest.world_label.clone(),
        terrain: terrain_state,
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
        sky: sky_state,
        sky_bundle_path: Some("shared/sky/sky_bundle.json".to_string()),
    };

    world.set_proof_region_scene(scene);

    Ok(world)
}
