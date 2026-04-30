use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::state::CommandLifecycleState;

pub(crate) fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Tracks the full command lifecycle: reception -> validation -> execution -> result publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_id: u64,
    pub route_id: String,
    pub lifecycle_state: CommandLifecycleState,
    pub payload: Vec<u8>,
    pub origin_timestamp: u64,
    pub last_update_timestamp: u64,
    pub error_message: Option<String>,
}

impl CommandEnvelope {
    pub fn new(command_id: u64, route_id: impl Into<String>, payload: Vec<u8>) -> Self {
        let now = now_ms();
        Self {
            command_id,
            route_id: route_id.into(),
            lifecycle_state: CommandLifecycleState::Accepted,
            payload,
            origin_timestamp: now,
            last_update_timestamp: now,
            error_message: None,
        }
    }
}

/// Orchestrates the full command lifecycle pipeline.
pub struct CommandLifecycleTracker {
    pub(crate) envelopes: HashMap<u64, CommandEnvelope>,
    next_command_id: u64,
}

impl Default for CommandLifecycleTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandLifecycleTracker {
    pub fn new() -> Self {
        Self {
            envelopes: HashMap::new(),
            next_command_id: 1,
        }
    }

    /// Phase 1: Reception - accept a command and create an envelope.
    pub fn receive_command(&mut self, route_id: impl Into<String>, payload: Vec<u8>) -> u64 {
        let command_id = self.next_command_id;
        self.next_command_id += 1;

        let envelope = CommandEnvelope::new(command_id, route_id, payload);
        self.envelopes.insert(command_id, envelope);
        command_id
    }
}
