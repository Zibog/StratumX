use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QuestId(pub Uuid);

impl QuestId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: QuestId,
    pub name: String,
    pub description: String,
    pub objectives: Vec<Uuid>,
    pub completed: bool,
}

impl Quest {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: QuestId::new(),
            name,
            description,
            objectives: Vec::new(),
            completed: false,
        }
    }
}