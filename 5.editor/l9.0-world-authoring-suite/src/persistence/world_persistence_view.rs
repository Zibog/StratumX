//! World Persistence View
//!
//! Separate persistence type for WorldOwner that excludes runtime-only fields.

use crate::owners::world_owner::{
    EnvironmentState, RuntimeModeState, TerrainState, WorldIdentity, WorldOwner,
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

/// Convert from WorldOwner reference to WorldPersistenceView
impl From<&WorldOwner> for WorldPersistenceView {
    fn from(owner: &WorldOwner) -> Self {
        Self {
            world_identity: owner.world_identity.clone(),
            world_snapshot_ref: owner.world_snapshot_ref.clone(),
            terrain_state: owner.terrain_state.clone(),
            environment_state: owner.environment_state.clone(),
            world_diagnostics: owner.world_diagnostics.clone(),
            runtime_mode: owner.runtime_mode.clone(),
            audio_source_count: owner.audio_source_count,
        }
    }
}

/// Convert from WorldPersistenceView to WorldOwner
///
/// Note: Runtime-only fields like event_callback are not restored and must be set separately.
impl TryFrom<WorldPersistenceView> for WorldOwner {
    type Error = String;

    fn try_from(view: WorldPersistenceView) -> Result<Self, Self::Error> {
        Ok(WorldOwner::from_persistence(
            view.world_identity,
            view.world_snapshot_ref,
            view.terrain_state,
            view.environment_state,
            view.world_diagnostics,
            view.runtime_mode,
            view.audio_source_count,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use uuid::Uuid;

    #[test]
    fn test_persistence_view_creation() {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let view = WorldPersistenceView::new(
            world_id,
            "snapshot_123".to_string(),
            None,
            None,
            Vec::new(),
            RuntimeModeState::default(),
            0,
        );

        assert_eq!(view.world_snapshot_ref, "snapshot_123");
        assert!(view.terrain_state.is_none());
        assert!(view.environment_state.is_none());
    }

    #[test]
    fn test_persistence_view_serialization() {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let view = WorldPersistenceView::new(
            world_id,
            "snapshot_123".to_string(),
            None,
            None,
            Vec::new(),
            RuntimeModeState::default(),
            0,
        );

        // Test serialization round-trip
        let json = serde_json::to_string(&view).unwrap();
        let deserialized: WorldPersistenceView = serde_json::from_str(&json).unwrap();

        assert_eq!(view.world_snapshot_ref, deserialized.world_snapshot_ref);
    }

    #[test]
    fn test_from_world_owner() {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let owner = WorldOwner::new(world_id.clone(), "snapshot_123".to_string());
        let view = WorldPersistenceView::from(&owner);

        assert_eq!(view.world_identity, world_id);
        assert_eq!(view.world_snapshot_ref, "snapshot_123");
        assert!(view.terrain_state.is_none());
        assert!(view.environment_state.is_none());
    }

    #[test]
    fn test_try_from_persistence_view() {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let view = WorldPersistenceView::new(
            world_id.clone(),
            "snapshot_123".to_string(),
            None,
            None,
            Vec::new(),
            RuntimeModeState::default(),
            0,
        );

        let owner = WorldOwner::try_from(view).unwrap();

        assert_eq!(owner.world_identity, world_id);
        assert_eq!(owner.world_snapshot_ref, "snapshot_123");
        assert!(owner.terrain_state.is_none());
        // Note: event_callback is runtime-only and not restored from persistence
    }

    #[test]
    fn test_round_trip_conversion() {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let mut original = WorldOwner::new(world_id.clone(), "snapshot_123".to_string());
        original.set_terrain_state(Some(TerrainState::new(
            (1024, 1024),
            (1000.0, 1000.0),
            "default".to_string(),
        )));

        // Convert to persistence view and back
        let view = WorldPersistenceView::from(&original);
        let restored = WorldOwner::try_from(view).unwrap();

        // Persistable state should match
        assert_eq!(original.world_identity, restored.world_identity);
        assert_eq!(original.world_snapshot_ref, restored.world_snapshot_ref);
        assert_eq!(
            original.terrain_state.is_some(),
            restored.terrain_state.is_some()
        );
        // Note: EnvironmentState and DiagnosticMessage don't implement PartialEq yet
        // so we can't directly compare them, but we can check their presence
        assert_eq!(
            original.environment_state.is_some(),
            restored.environment_state.is_some()
        );
        assert_eq!(
            original.world_diagnostics.len(),
            restored.world_diagnostics.len()
        );
    }
}
