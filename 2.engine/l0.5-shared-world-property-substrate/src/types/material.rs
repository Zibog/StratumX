use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::PropertyType;

/// Conflict resolution strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Use priority order (first property wins).
    Priority,
    /// Blend values with weights.
    Blend,
    /// Take maximum value.
    Maximum,
    /// Take minimum value.
    Minimum,
}

/// Conflict resolution rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRule {
    /// Properties involved in conflict.
    pub properties: Vec<PropertyType>,
    /// Resolution strategy.
    pub resolution: ConflictResolution,
    /// Weights for blending (if using Blend strategy).
    pub weights: HashMap<PropertyType, f32>,
}
