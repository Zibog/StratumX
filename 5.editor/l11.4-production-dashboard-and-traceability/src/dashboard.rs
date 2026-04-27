use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: uuid::Uuid,
    pub name: String,
    pub widgets: Vec<uuid::Uuid>,
}

impl Dashboard {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            widgets: Vec::new(),
        }
    }
}