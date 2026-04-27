use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AIAgentId(pub Uuid);

impl AIAgentId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAgent {
    pub id: AIAgentId,
    pub name: String,
    pub behavior_tree_id: Option<Uuid>,
    pub perception_radius: f32,
}

impl AIAgent {
    pub fn new(name: String) -> Self {
        Self {
            id: AIAgentId::new(),
            name,
            behavior_tree_id: None,
            perception_radius: 10.0,
        }
    }
}