use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationTask {
    pub id: uuid::Uuid,
    pub name: String,
    pub script: String,
}

impl AutomationTask {
    pub fn new(name: String, script: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            script,
        }
    }
}