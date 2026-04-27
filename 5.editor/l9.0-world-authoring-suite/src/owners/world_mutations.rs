//! World mutations — methods that change world state.

use super::world_owner::{
    EnvironmentState, RuntimeModeState, TerrainState, WorldIdentity, WorldOwner, WorldStateEvent,
};

impl WorldOwner {
    /// Sets the event callback for state mutations
    pub fn set_event_callback(
        &mut self,
        callback: crate::owners::world_owner::WorldStateEventCallback,
    ) {
        self.event_callback = Some(std::sync::Arc::new(callback));
    }

    /// Emits an event if a callback is registered
    fn emit_event(&self, event: WorldStateEvent) {
        if let Some(callback) = &self.event_callback {
            callback(event);
        }
    }

    /// Updates the world snapshot reference and emits event
    pub fn update_snapshot_ref(&mut self, snapshot_ref: String) {
        self.world_snapshot_ref = snapshot_ref.clone();
        self.emit_event(WorldStateEvent::SnapshotRefUpdated {
            new_ref: snapshot_ref,
        });
    }

    /// Sets the terrain state and emits event
    pub fn set_terrain_state(&mut self, terrain: Option<TerrainState>) {
        self.terrain_state = terrain;
        self.emit_event(WorldStateEvent::TerrainStateUpdated);
    }

    /// Sets the environment state and emits event
    pub fn set_environment_state(&mut self, environment: Option<EnvironmentState>) {
        self.environment_state = environment;
        self.emit_event(WorldStateEvent::EnvironmentStateUpdated);
    }

    /// Adds a diagnostic message and emits event
    pub fn add_diagnostic(&mut self, diagnostic: crate::DiagnosticMessage) {
        let message = diagnostic.message.clone();
        self.world_diagnostics.push(diagnostic);
        self.emit_event(WorldStateEvent::DiagnosticAdded { message });
    }

    /// Clears all diagnostics and emits event
    pub fn clear_diagnostics(&mut self) {
        self.world_diagnostics.clear();
        self.emit_event(WorldStateEvent::DiagnosticsCleared);
    }

    /// Updates world identity and emits event
    pub fn update_world_identity(&mut self, identity: WorldIdentity) {
        self.world_identity = identity;
        self.emit_event(WorldStateEvent::WorldIdentityUpdated);
    }

    /// Sets the runtime mode state and emits event
    pub fn set_runtime_mode(&mut self, runtime_mode: RuntimeModeState) {
        self.runtime_mode = runtime_mode;
        self.emit_event(WorldStateEvent::RuntimeModeUpdated);
    }

    /// Gets a mutable reference to terrain state for in-place modifications
    pub fn get_terrain_state_mut(&mut self) -> Option<&mut TerrainState> {
        self.terrain_state.as_mut()
    }

    /// Gets a mutable reference to environment state for in-place modifications
    pub fn get_environment_state_mut(&mut self) -> Option<&mut EnvironmentState> {
        self.environment_state.as_mut()
    }

    /// Increments the audio source count
    pub fn increment_audio_source_count(&mut self) {
        self.audio_source_count += 1;
    }
}
