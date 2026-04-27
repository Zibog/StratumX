use super::{RuntimeHostAccess, TerrainCommandExecutor};

impl TerrainCommandExecutor {
    /// Rebuild the active terrain
    ///
    /// Rebuilds the terrain mesh and collision for the active world.
    pub fn rebuild(&self, runtime_host: &mut dyn RuntimeHostAccess) -> Result<String, String> {
        {
            let scene = runtime_host
                .get_world_state_mut()
                .and_then(|world| world.vertical_slice_scene_mut())
                .ok_or_else(|| "No world loaded".to_string())?;

            scene.terrain.mesh_revision = scene.terrain.mesh_revision.saturating_add(1);
            scene.terrain.collision_revision = scene.terrain.collision_revision.saturating_add(1);
            for chunk in &mut scene.terrain.chunks {
                chunk.dirty = false;
                chunk.mesh_built = true;
                chunk.loaded = true;
            }
        }

        runtime_host.mark_terrain_gpu_dirty();
        Ok("Terrain mesh rebuilt".to_string())
    }
}
