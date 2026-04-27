// Terrain Authoring State

use editor_dto_law::{LodPosture, ProfileRef, StableWorldId, TerrainBindingRef, TerrainStateDto};
use uuid::Uuid;

pub struct TerrainAuthoringState {
    pub world_ref: Option<StableWorldId>,
    pub binding_ref: Option<TerrainBindingRef>,
    pub material_profile_ref: Option<ProfileRef>,
    pub dirty_regions: Vec<TerrainDirtyRegion>,
    pub pending_build: Option<TerrainBuildRequest>,
    pub lod_posture: LodPosture,
    pub degraded: bool,
    pub(super) present: bool,
    pub(super) walkable: bool,
    pub(super) buffers_need_gpu_sync: bool,
}

#[derive(Debug, Clone)]
pub struct TerrainDirtyRegion {
    pub chunk_x: u32,
    pub chunk_y: u32,
    pub dirty: bool,
}

#[derive(Debug, Clone)]
pub struct TerrainBuildRequest {
    pub regions: Vec<TerrainDirtyRegion>,
    pub full_rebuild: bool,
}

impl Default for TerrainAuthoringState {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainAuthoringState {
    pub fn new() -> Self {
        Self {
            world_ref: None,
            binding_ref: None,
            material_profile_ref: None,
            dirty_regions: Vec::new(),
            pending_build: None,
            lod_posture: LodPosture::Full,
            degraded: false,
            present: false,
            walkable: false,
            buffers_need_gpu_sync: false,
        }
    }

    pub fn bind_to_world(&mut self, world_ref: StableWorldId) -> Result<TerrainBindingRef, String> {
        if self.world_ref.is_some() && self.world_ref != Some(world_ref) {
            return Err("Terrain already bound to different world".to_string());
        }

        let binding_ref = TerrainBindingRef(Uuid::new_v4());
        self.world_ref = Some(world_ref);
        self.binding_ref = Some(binding_ref);
        self.present = true;

        Ok(binding_ref)
    }

    pub fn bind_material_profile(&mut self, profile_ref: ProfileRef) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No terrain binding exists".to_string());
        }
        self.material_profile_ref = Some(profile_ref);
        Ok(())
    }

    pub fn queue_rebuild(&mut self) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No terrain binding exists".to_string());
        }

        let build_request = TerrainBuildRequest {
            regions: self.dirty_regions.clone(),
            full_rebuild: self.dirty_regions.is_empty(),
        };

        self.pending_build = Some(build_request);
        self.present = true;

        Ok(())
    }

    pub fn mark_region_dirty(&mut self, chunk_x: u32, chunk_y: u32) {
        if !self
            .dirty_regions
            .iter()
            .any(|r| r.chunk_x == chunk_x && r.chunk_y == chunk_y)
        {
            self.dirty_regions.push(TerrainDirtyRegion {
                chunk_x,
                chunk_y,
                dirty: true,
            });
        }
    }

    pub fn clear_dirty_regions(&mut self) {
        self.dirty_regions.clear();
    }

    pub fn set_walkable(&mut self, walkable: bool) {
        self.walkable = walkable;
    }

    pub fn set_lod_posture(&mut self, posture: LodPosture) {
        self.lod_posture = posture;
    }

    pub fn set_degraded(&mut self, degraded: bool) {
        self.degraded = degraded;
    }

    pub fn unbind(&mut self) {
        self.world_ref = None;
        self.binding_ref = None;
        self.material_profile_ref = None;
        self.present = false;
        self.walkable = false;
        self.degraded = false;
        self.dirty_regions.clear();
        self.pending_build = None;
        self.buffers_need_gpu_sync = false;
    }

    pub fn is_bound(&self) -> bool {
        self.binding_ref.is_some()
    }

    pub fn is_present(&self) -> bool {
        self.present
    }

    pub fn world_ref(&self) -> Option<StableWorldId> {
        self.world_ref
    }

    pub fn binding_ref(&self) -> Option<TerrainBindingRef> {
        self.binding_ref
    }

    pub fn to_dto(&self) -> Option<TerrainStateDto> {
        let world_ref = self.world_ref?;
        Some(TerrainStateDto {
            world_ref,
            terrain_binding_ref: self.binding_ref,
            present: self.present,
            walkable: self.walkable,
            material_profile_ref: self.material_profile_ref,
            lod_posture: self.lod_posture.clone(),
            degraded: self.degraded,
        })
    }
}
