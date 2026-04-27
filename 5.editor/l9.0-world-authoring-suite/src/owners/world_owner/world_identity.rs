use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// World identity from World_Registry.
///
/// World IDs must come from the World Registry, never generated at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldIdentity {
    /// World ID from registry (not generated).
    pub world_id: Uuid,

    /// World name.
    pub world_name: String,

    /// World path on disk.
    pub world_path: PathBuf,
}

impl WorldIdentity {
    /// Creates a new world identity.
    pub fn new(world_id: Uuid, world_name: String, world_path: PathBuf) -> Self {
        Self {
            world_id,
            world_name,
            world_path,
        }
    }
}
