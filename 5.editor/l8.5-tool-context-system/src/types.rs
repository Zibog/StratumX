//! Core types for tool context system

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entity identifier using UUID
pub type EntityId = Uuid;

/// Panel identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub String);

/// World identity containing ID and name
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldIdentity {
    pub world_id: Uuid,
    pub world_name: String,
}

impl WorldIdentity {
    pub fn new(world_id: Uuid, world_name: String) -> Self {
        Self {
            world_id,
            world_name,
        }
    }
}
