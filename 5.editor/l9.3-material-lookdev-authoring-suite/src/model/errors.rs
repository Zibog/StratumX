use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolingError {
    UnknownObject,
    Message(String),
}

impl From<String> for ToolingError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}
