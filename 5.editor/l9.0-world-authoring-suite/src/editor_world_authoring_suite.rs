//! Editor World Authoring Suite
//!
//! Role: World authoring, world session management, and world queries.
//! Owns: World state, world session service, world queries, world persistence.

pub mod host;
pub mod session;

pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldIdentity {
    pub world_id: String,
    pub world_name: String,
    pub world_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldRuntimePosture {
    pub is_runtime_active: bool,
    pub simulation_paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldBindings {
    pub bindings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldAuthoringSuite {
    pub identity: WorldIdentity,
    pub runtime_posture: WorldRuntimePosture,
    pub bindings: WorldBindings,
    pub open: bool,
}

impl WorldAuthoringSuite {
    pub fn new() -> Self {
        Self {
            identity: WorldIdentity::default(),
            runtime_posture: WorldRuntimePosture::default(),
            bindings: WorldBindings::default(),
            open: false,
        }
    }

    pub fn open_world(&mut self, world_id: &str, world_name: &str) {
        self.identity.world_id = world_id.to_string();
        self.identity.world_name = world_name.to_string();
        self.open = true;
    }

    pub fn close_world(&mut self) {
        self.identity = WorldIdentity::default();
        self.runtime_posture = WorldRuntimePosture::default();
        self.bindings = WorldBindings::default();
        self.open = false;
    }
}
