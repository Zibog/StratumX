// World saving - canonical package format (modular)

mod environment;
mod manifest;
mod terrain;

use super::WorldLifecycleManager;
use editor_dto_law::StableWorldId;
use engine_world::WorldState;
use std::fs;
use std::path::Path;

impl WorldLifecycleManager {
    /// Save world to disk in canonical package format
    ///
    /// Creates:
    /// - world.json (manifest)
    /// - terrain/terrain_manifest.json
    /// - terrain/chunks/*.bin (binary chunk data)
    /// - environment/sky_binding.json
    pub fn save_world_to_path(
        &self,
        world: &WorldState,
        world_ref: StableWorldId,
        path: &Path,
    ) -> Result<(), String> {
        if !path.exists() {
            fs::create_dir_all(path)
                .map_err(|e| format!("Failed to create world directory: {}", e))?;
        }

        let scene = world
            .vertical_slice_scene()
            .ok_or("No scene in world state")?;

        // Save world manifest
        manifest::save_world_manifest(path, world_ref, scene)?;

        // Save environment
        environment::save_environment_binding(path, scene)?;

        // Save terrain
        terrain::save_terrain_package(path, scene)?;

        Ok(())
    }
}
