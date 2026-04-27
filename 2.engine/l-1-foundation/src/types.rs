use serde::{Deserialize, Serialize};

/// Generation counter used for stale-detection semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Generation(pub u32);

impl Generation {
    pub const INITIAL: Self = Self(1);

    pub fn next(self) -> Self {
        Self(self.0.wrapping_add(1).max(1))
    }
}

/// Simulation tick index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tick(pub u64);

/// Stable type id for component columns/signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ComponentTypeId(pub u64);
