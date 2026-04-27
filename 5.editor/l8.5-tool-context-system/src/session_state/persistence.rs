//! Session State Persistence

use crate::session_errors::{DeserializationError, SerializationError};

use super::types::SessionState;

impl SessionState {
    pub fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        bincode::serialize(self).map_err(|e| SerializationError {
            reason: format!("Failed to serialize SessionState: {}", e),
        })
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, DeserializationError> {
        bincode::deserialize(data).map_err(|e| DeserializationError {
            reason: format!("Failed to deserialize SessionState: {}", e),
        })
    }
}
