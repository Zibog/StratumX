//! World persistence view

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPersistenceView {
    pub world_identity: crate::owners::world_owner::WorldIdentity,
    pub world_snapshot_ref: String,
    pub terrain_state: bool,
    pub environment_state: bool,
    pub world_diagnostics: usize,
    pub runtime_mode: RuntimeModeStateStub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeModeStateStub;
