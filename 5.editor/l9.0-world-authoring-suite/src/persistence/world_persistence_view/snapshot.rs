//! World persistence snapshot types

use crate::owners::world_owner::{
    EnvironmentState, RuntimeModeState, TerrainState, WorldIdentity,
};
use crate::DiagnosticMessage;
use serde::{Deserialize, Serialize};

/// World persistence view - contains only persistable state
///
/// This is separate from WorldOwner to exclude runtime-only fields like event_bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPersistenceView {
    /// World identity from World_Registry (not generated)
    pub world_identity: WorldIdentity,

    /// World snapshot reference
    pub world_snapshot_ref: String,

    /// Terrain state (None if no terrain)
    pub terrain_state: Option<TerrainState>,

    /// Environment state (None if no environment)
    pub environment_state: Option<EnvironmentState>,

    /// World-level diagnostics
    pub world_diagnostics: Vec<DiagnosticMessage>,

    /// Runtime mode state
    pub runtime_mode: RuntimeModeState,

    /// Audio source count
    pub audio_source_count: usize,
}

impl WorldPersistenceView {
    /// Creates a new world persistence view
    pub fn new(
        world_identity: WorldIdentity,
        world_snapshot_ref: String,
        terrain_state: Option<TerrainState>,
        environment_state: Option<EnvironmentState>,
        world_diagnostics: Vec<DiagnosticMessage>,
        runtime_mode: RuntimeModeState,
        audio_source_count: usize,
    ) -> Self {
        Self {
            world_identity,
            world_snapshot_ref,
            terrain_state,
            environment_state,
            world_diagnostics,
            runtime_mode,
            audio_source_count,
        }
    }
}
