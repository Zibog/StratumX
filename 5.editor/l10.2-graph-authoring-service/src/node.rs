use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub node_type: String,
    pub position: (f32, f32),
}

impl Node {
    pub fn new(node_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            node_type,
            position: (0.0, 0.0),
        }
    }
}