use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: Uuid,
    pub event_type: String,
    pub parameters: Vec<(String, String)>,
}

impl GameEvent {
    pub fn new(event_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            parameters: Vec::new(),
        }
    }
}