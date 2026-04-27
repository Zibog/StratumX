//! Ducking Policy - priority and mix policy.

use serde::{Deserialize, Serialize};

use crate::model::ObjectHandle;

/// Ducking Policy - priority and mix policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuckingPolicy {
    pub handle: ObjectHandle,
    pub name: String,
    pub priority_levels: Vec<u32>,
    pub duck_amount: u32, // 0-100
}
