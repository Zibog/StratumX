//! World projections — derived views, summaries, and inspector data.

use super::world_owner::{
    EnvironmentState, RuntimeModeState, TerrainState, WorldIdentity, WorldOwner,
};

impl WorldOwner {
    /// Gets the world identity
    pub fn get_world_identity(&self) -> &WorldIdentity {
        &self.world_identity
    }

    /// Gets the world snapshot reference
    pub fn get_snapshot_ref(&self) -> &str {
        &self.world_snapshot_ref
    }

    /// Gets the terrain state
    pub fn get_terrain_state(&self) -> Option<&TerrainState> {
        self.terrain_state.as_ref()
    }

    /// Gets the environment state
    pub fn get_environment_state(&self) -> Option<&EnvironmentState> {
        self.environment_state.as_ref()
    }

    /// Gets all diagnostics
    pub fn get_diagnostics(&self) -> &[crate::DiagnosticMessage] {
        &self.world_diagnostics
    }

    /// Gets the runtime mode state
    pub fn get_runtime_mode(&self) -> &RuntimeModeState {
        &self.runtime_mode
    }

    /// Converts to persistence view for serialization
    ///
    /// Excludes runtime-only fields like event_callback.
    pub fn to_persistence_view(&self) -> crate::persistence::WorldPersistenceView {
        crate::persistence::WorldPersistenceView::new(
            self.world_identity.clone(),
            self.world_snapshot_ref.clone(),
            self.terrain_state.clone(),
            self.environment_state.clone(),
            self.world_diagnostics.clone(),
            self.runtime_mode.clone(),
            self.audio_source_count,
        )
    }
}
