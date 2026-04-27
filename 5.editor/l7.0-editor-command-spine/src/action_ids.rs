//! Action IDs - Unique identifiers for editor actions
//!
//! This module defines the ActionId type and related functionality for
//! uniquely identifying actions in the command spine.

use std::fmt;

// ============================================================================
// ActionId Type
// ============================================================================

/// Unique identifier for an action in the command spine.
///
/// ActionIds follow a hierarchical naming convention:
/// - `domain.operation` (e.g., "world.open", "material.author")
/// - `domain.subdomain.operation` (e.g., "terrain.layer.configure")
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionId(String);

impl ActionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ActionId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ActionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for ActionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_id_creation() {
        let id = ActionId::new("world.open");
        assert_eq!(id.as_str(), "world.open");

        let id2: ActionId = "material.author".into();
        assert_eq!(id2.as_str(), "material.author");
    }

    #[test]
    fn test_action_id_display() {
        let id = ActionId::new("terrain.sculpt");
        assert_eq!(format!("{}", id), "terrain.sculpt");
    }

    #[test]
    fn test_action_id_ordering() {
        let id1 = ActionId::new("audio.preview");
        let id2 = ActionId::new("world.open");
        assert!(id1 < id2);
    }
}
