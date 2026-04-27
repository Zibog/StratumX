use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: uuid::Uuid,
    pub name: String,
    pub source: String,
    pub language: ScriptLanguage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLanguage {
    Lua,
    Python,
    JavaScript,
}

impl Script {
    pub fn new(name: String, language: ScriptLanguage) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            source: String::new(),
            language,
        }
    }
}