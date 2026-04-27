//! World owner aggregate.
//!
//! This module keeps the owner surface thin while responsibility-specific
//! world types live in focused submodules.

use serde::{Deserialize, Serialize};

mod world_bindings;
mod world_diagnostics_link;
mod world_event_link;
mod world_identity;
mod world_runtime_posture;
mod world_snapshot_ref;

pub use world_bindings::{EnvironmentState, TerrainState, WeatherCondition};
pub use world_diagnostics_link::WorldDiagnosticsLink;
pub use world_event_link::{WorldStateEvent, WorldStateEventCallback};
pub use world_identity::WorldIdentity;
pub use world_runtime_posture::RuntimeModeState;
pub use world_snapshot_ref::WorldSnapshotRef;

/// World state container.
///
/// Owns world-level state including identity from World_Registry,
/// terrain state, environment state, and world-level diagnostics.
#[derive(Clone, Serialize, Deserialize)]
pub struct WorldOwner {
    /// World identity from World_Registry (not generated).
    pub world_identity: WorldIdentity,

    /// World snapshot reference.
    pub world_snapshot_ref: WorldSnapshotRef,

    /// Terrain state (None if no terrain).
    pub terrain_state: Option<TerrainState>,

    /// Environment state (None if no environment).
    pub environment_state: Option<EnvironmentState>,

    /// World-level diagnostics.
    pub world_diagnostics: WorldDiagnosticsLink,

    /// Runtime mode state.
    pub runtime_mode: RuntimeModeState,

    /// Audio source count.
    pub audio_source_count: usize,

    /// Event callback (not serialized).
    #[serde(skip)]
    pub(crate) event_callback: Option<std::sync::Arc<WorldStateEventCallback>>,
}

impl std::fmt::Debug for WorldOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorldOwner")
            .field("world_identity", &self.world_identity)
            .field("world_snapshot_ref", &self.world_snapshot_ref)
            .field("terrain_state", &self.terrain_state)
            .field("environment_state", &self.environment_state)
            .field("world_diagnostics", &self.world_diagnostics)
            .field("runtime_mode", &self.runtime_mode)
            .field("audio_source_count", &self.audio_source_count)
            .field(
                "event_callback",
                &self.event_callback.as_ref().map(|_| "<callback>"),
            )
            .finish()
    }
}

impl WorldOwner {
    /// Creates a new world state.
    pub fn new(world_identity: WorldIdentity, world_snapshot_ref: WorldSnapshotRef) -> Self {
        Self {
            world_identity,
            world_snapshot_ref,
            terrain_state: None,
            environment_state: None,
            world_diagnostics: Vec::new(),
            runtime_mode: RuntimeModeState::default(),
            audio_source_count: 0,
            event_callback: None,
        }
    }

    /// Creates a world state from persistence data.
    pub fn from_persistence(
        world_identity: WorldIdentity,
        world_snapshot_ref: WorldSnapshotRef,
        terrain_state: Option<TerrainState>,
        environment_state: Option<EnvironmentState>,
        world_diagnostics: WorldDiagnosticsLink,
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
            event_callback: None,
        }
    }
}
