use serde::{Deserialize, Serialize};

use super::PropertyType;

/// Object-local field data (per-entity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLocalField {
    /// Property type.
    pub property: PropertyType,
    /// Entity ID.
    pub entity_id: u64,
    /// Field value.
    pub value: f32,
}
