//! World owner types

use crate::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldIdentity {
    pub world_id: Uuid,
    pub world_name: String,
    pub world_path: PathBuf,
}

impl WorldIdentity {
    pub fn new(world_id: Uuid, world_name: String, world_path: PathBuf) -> Self {
        Self {
            world_id,
            world_name,
            world_path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainState {
    pub resolution: (u32, u32),
    pub bounds: (f32, f32),
    pub material: String,
    pub modification_count: u64,
}

impl TerrainState {
    pub fn new(resolution: (u32, u32), bounds: (f32, f32), material: String) -> Self {
        Self {
            resolution,
            bounds,
            material,
            modification_count: 0,
        }
    }

    pub fn increment_modifications(&mut self) {
        self.modification_count = self.modification_count.saturating_add(1);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeModeState {
    pub is_preview_mode: bool,
    pub is_simulation_running: bool,
}

#[derive(Debug, Clone)]
pub struct WorldOwner {
    pub world_identity: WorldIdentity,
    pub world_snapshot_ref: String,
    pub terrain_state: Option<TerrainState>,
    pub environment_state: Option<crate::EnvironmentState>,
    pub world_diagnostics: Vec<crate::DiagnosticMessage>,
}

impl WorldOwner {
    pub fn new(world_identity: WorldIdentity, world_snapshot_ref: String) -> Self {
        Self {
            world_identity,
            world_snapshot_ref,
            terrain_state: None,
            environment_state: None,
            world_diagnostics: Vec::new(),
        }
    }
    pub fn set_terrain_state(&mut self, terrain: Option<TerrainState>) {
        self.terrain_state = terrain;
    }
    pub fn set_environment_state(&mut self, env: Option<crate::EnvironmentState>) {
        self.environment_state = env;
    }
    pub fn get_terrain_state(&self) -> &Option<TerrainState> {
        &self.terrain_state
    }
    pub fn get_environment_state(&self) -> &Option<crate::EnvironmentState> {
        &self.environment_state
    }
    pub fn add_diagnostic(&mut self, msg: crate::DiagnosticMessage) {
        self.world_diagnostics.push(msg);
    }
    pub fn get_diagnostics(&self) -> &Vec<crate::DiagnosticMessage> {
        &self.world_diagnostics
    }
    pub fn to_persistence_view(&self) -> crate::persistence::WorldPersistenceView {
        crate::persistence::WorldPersistenceView {
            world_identity: self.world_identity.clone(),
            world_snapshot_ref: self.world_snapshot_ref.clone(),
            terrain_state: self.terrain_state.is_some(),
            environment_state: self.environment_state.is_some(),
            world_diagnostics: self.world_diagnostics.len(),
            runtime_mode: crate::persistence::RuntimeModeStateStub,
        }
    }
}
