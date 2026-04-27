// Path-based world opening - canonical package loading

pub mod assemble_world;
pub mod load_environment;
pub mod load_manifest;
pub mod load_terrain;
pub mod validate;

use super::WorldLifecycleManager;
use editor_dto_law::{
    BindPosture, SkyEnvironmentBindingRef, StableWorldId, TerrainBindingRef, WorldBindState,
    WorldOpenResult,
};
use std::path::Path;
use uuid::Uuid;

impl WorldLifecycleManager {
    pub fn open_world_from_path(&mut self, path: &Path) -> WorldOpenResult {
        if let Err(result) = validate::validate_world_path(path) {
            return result;
        }

        let manifest = match load_manifest::load_world_manifest(path) {
            Ok(m) => m,
            Err(result) => return result,
        };

        match assemble_world::assemble_world_from_manifest(path, &manifest) {
            Ok(world_state) => {
                let world_ref = StableWorldId(manifest.world_id);

                self.world_state = Some(world_state);
                self.current_world = Some(world_ref);
                self.bind_state = Some(WorldBindState {
                    world_ref,
                    terrain_ref: manifest
                        .terrain_root_ref
                        .as_ref()
                        .map(|_| TerrainBindingRef(Uuid::new_v4())),
                    environment_ref: manifest
                        .environment_root_ref
                        .as_ref()
                        .map(|_| SkyEnvironmentBindingRef(Uuid::new_v4())),
                    posture: BindPosture::Binding,
                });

                WorldOpenResult {
                    accepted: true,
                    world_ref: Some(world_ref),
                    world_label: Some(manifest.world_label.clone()),
                    failure_class: None,
                    recovery_hints: vec![],
                }
            }
            Err(e) => WorldOpenResult {
                accepted: false,
                world_ref: None,
                world_label: Some(manifest.world_label.clone()),
                failure_class: Some(editor_dto_law::FailureClass::EngineError),
                recovery_hints: vec![format!("Failed to load world: {}", e)],
            },
        }
    }
}
