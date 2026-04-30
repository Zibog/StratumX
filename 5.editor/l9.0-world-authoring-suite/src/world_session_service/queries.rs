//! World Session Service - Query Handlers
//!
//! Handles world state queries and summary information.

use super::session::WorldSessionService;
use crate::WorldIdentity;
use serde::{Deserialize, Serialize};

/// Data transfer object for world summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSummaryDto {
    pub world_identity: Option<WorldIdentity>,
    pub world_snapshot_ref: Option<String>,
    pub has_terrain: bool,
    pub has_environment: bool,
    pub diagnostic_count: usize,
}

impl WorldSessionService {
    /// Get a summary of the current world state
    pub fn get_world_summary(&self) -> WorldSummaryDto {
        let world = self.world_state.lock().unwrap();

        WorldSummaryDto {
            world_identity: if world.world_identity.world_id.is_nil() {
                None
            } else {
                Some(world.world_identity.clone())
            },
            world_snapshot_ref: if world.world_snapshot_ref.is_empty() {
                None
            } else {
                Some(world.world_snapshot_ref.clone())
            },
            has_terrain: world.terrain_state.is_some(),
            has_environment: world.environment_state.is_some(),
            diagnostic_count: world.world_diagnostics.len(),
        }
    }
}
