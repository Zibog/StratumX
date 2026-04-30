//! World persistence summary and conversion logic

use super::snapshot::WorldPersistenceView;
use crate::owners::world_owner::WorldOwner;

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
