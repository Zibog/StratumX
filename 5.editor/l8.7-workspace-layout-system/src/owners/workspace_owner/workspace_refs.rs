use serde::{Deserialize, Serialize};

/// Panel identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub String);

impl PanelId {
    /// Creates a new panel ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for PanelId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for PanelId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}
