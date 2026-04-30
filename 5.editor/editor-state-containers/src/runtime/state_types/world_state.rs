//! World state type

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorldState {
    pub identity: WorldIdentity,
    pub world_identity: WorldIdentity, // Alias for compatibility
    pub snapshot_ref: String,
    pub terrain_state: Option<crate::owners::world_owner::TerrainState>,
    pub environment_state: Option<EnvironmentState>,
    pub runtime_mode: crate::owners::world_owner::RuntimeModeState,
    pub audio_source_count: usize,
    pub diagnostics: Vec<DiagnosticMessage>,
    #[serde(skip)]
    event_callback: Option<Box<dyn Fn(&str)>>,
}

impl std::fmt::Debug for WorldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorldState")
            .field("identity", &self.identity)
            .field("world_identity", &self.world_identity)
            .field("snapshot_ref", &self.snapshot_ref)
            .field("terrain_state", &self.terrain_state)
            .field("environment_state", &self.environment_state)
            .field("runtime_mode", &self.runtime_mode)
            .field("audio_source_count", &self.audio_source_count)
            .field("diagnostics", &self.diagnostics)
            .finish_non_exhaustive()
    }
}

impl Clone for WorldState {
    fn clone(&self) -> Self {
        Self {
            identity: self.identity.clone(),
            world_identity: self.world_identity.clone(),
            snapshot_ref: self.snapshot_ref.clone(),
            terrain_state: self.terrain_state.clone(),
            environment_state: self.environment_state.clone(),
            runtime_mode: self.runtime_mode.clone(),
            audio_source_count: self.audio_source_count,
            diagnostics: self.diagnostics.clone(),
            event_callback: None,
        }
    }
}

impl WorldState {
    pub fn new(identity: WorldIdentity, snapshot_ref: String) -> Self {
        Self {
            identity: identity.clone(),
            world_identity: identity,
            snapshot_ref,
            terrain_state: None,
            environment_state: None,
            runtime_mode: crate::owners::world_owner::RuntimeModeState::default(),
            audio_source_count: 0,
            diagnostics: Vec::new(),
            event_callback: None,
        }
    }

    pub fn get_terrain_state(&self) -> &Option<crate::owners::world_owner::TerrainState> {
        &self.terrain_state
    }

    pub fn get_terrain_state_mut(
        &mut self,
    ) -> Option<&mut crate::owners::world_owner::TerrainState> {
        self.terrain_state.as_mut()
    }

    pub fn set_terrain_state(&mut self, terrain: Option<crate::owners::world_owner::TerrainState>) {
        self.terrain_state = terrain;
        self.emit_event("TerrainStateUpdated");
    }

    pub fn get_environment_state(&self) -> &Option<EnvironmentState> {
        &self.environment_state
    }

    pub fn get_environment_state_mut(&mut self) -> Option<&mut EnvironmentState> {
        self.environment_state.as_mut()
    }

    pub fn set_environment_state(&mut self, env: Option<EnvironmentState>) {
        self.environment_state = env;
        self.emit_event("EnvironmentStateUpdated");
    }

    pub fn add_diagnostic(&mut self, msg: DiagnosticMessage) {
        self.diagnostics.push(msg);
        self.emit_event("DiagnosticAdded");
    }

    pub fn get_diagnostics(&self) -> &Vec<DiagnosticMessage> {
        &self.diagnostics
    }

    pub fn set_event_callback(&mut self, callback: Box<dyn Fn(&str)>) {
        self.event_callback = Some(callback);
    }

    pub fn get_world_identity(&self) -> &WorldIdentity {
        &self.identity
    }

    pub fn get_snapshot_ref(&self) -> &str {
        &self.snapshot_ref
    }

    fn emit_event(&self, event: &str) {
        if let Some(callback) = self.event_callback.as_ref() {
            callback(event);
        }
    }
}
