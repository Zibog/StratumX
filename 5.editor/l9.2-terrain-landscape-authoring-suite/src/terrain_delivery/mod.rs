// Terrain Delivery - Modular Structure

pub mod binding;
pub mod chunk_io;
pub mod diagnostics;
pub mod import;
pub mod rebuild;
pub mod state;
pub mod sync;

pub use diagnostics::*;
pub use state::*;

use editor_dto_law::{LodPosture, ProfileRef, StableWorldId, TerrainBindingRef, TerrainStateDto};
use engine_world::WorldState;

pub struct TerrainDeliveryThread {
    state: TerrainAuthoringState,
}

impl TerrainDeliveryThread {
    pub fn new() -> Self {
        Self {
            state: TerrainAuthoringState::new(),
        }
    }

    pub fn terrain_root(&mut self, world_ref: StableWorldId) -> Result<TerrainBindingRef, String> {
        self.state.bind_to_world(world_ref)
    }

    pub fn terrain_bind(&mut self, profile_ref: ProfileRef) -> Result<(), String> {
        self.state.bind_material_profile(profile_ref)
    }

    pub fn terrain_rebuild(&mut self) -> Result<(), String> {
        self.state.queue_rebuild()
    }

    pub fn sync_to_world(&self, world: &mut WorldState) -> Result<(), String> {
        self.state.sync_to_world(world)
    }

    pub fn sync_from_world(&mut self, world: &WorldState) -> Result<(), String> {
        self.state.sync_from_world(world)
    }

    pub fn terrain_material_binding(&mut self, profile_ref: ProfileRef) -> Result<(), String> {
        self.terrain_bind(profile_ref)
    }

    pub fn collision_walkable_ground(&mut self, walkable: bool) -> Result<(), String> {
        self.state.set_walkable(walkable);
        Ok(())
    }

    pub fn terrain_diagnostics(&self) -> TerrainDiagnostics {
        self.state.diagnostics()
    }

    pub fn terrain_visible_in_same_frame_as_sky(&self) -> bool {
        self.state.is_present()
    }

    pub fn get_state_dto(&self) -> Option<TerrainStateDto> {
        self.state.to_dto()
    }

    pub fn set_lod_posture(&mut self, posture: LodPosture) {
        self.state.set_lod_posture(posture);
    }

    pub fn set_degraded(&mut self, degraded: bool) {
        self.state.set_degraded(degraded);
    }

    pub fn unbind(&mut self) {
        self.state.unbind();
    }

    pub fn is_bound(&self) -> bool {
        self.state.is_bound()
    }

    pub fn get_world_ref(&self) -> Option<StableWorldId> {
        self.state.world_ref()
    }

    pub fn get_binding_ref(&self) -> Option<TerrainBindingRef> {
        self.state.binding_ref()
    }

    pub fn import_heightmap(
        &mut self,
        world: &mut WorldState,
        path: &std::path::Path,
    ) -> Result<(), String> {
        self.state.import_heightmap(world, path)
    }

    pub fn rebuild_dirty_chunks(&mut self, world: &mut WorldState) -> Result<(), String> {
        self.state.rebuild_dirty_chunks(world)
    }

    pub fn write_chunks(
        &self,
        world: &mut WorldState,
        world_path: &std::path::Path,
    ) -> Result<(), String> {
        self.state.write_chunks(world, world_path)
    }

    pub fn read_chunks(
        &mut self,
        world: &mut WorldState,
        world_path: &std::path::Path,
    ) -> Result<(), String> {
        self.state.read_chunks(world, world_path)
    }

    pub fn needs_gpu_sync(&self) -> bool {
        self.state.buffers_need_gpu_sync
    }

    pub fn clear_gpu_sync_flag(&mut self) {
        self.state.buffers_need_gpu_sync = false;
    }
}

impl Default for TerrainDeliveryThread {
    fn default() -> Self {
        Self::new()
    }
}
