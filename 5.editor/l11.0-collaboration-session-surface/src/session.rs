use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub id: uuid::Uuid,
    pub name: String,
    pub users: Vec<uuid::Uuid>,
    pub active: bool,
}

impl CollaborationSession {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            users: Vec::new(),
            active: true,
        }
    }
}