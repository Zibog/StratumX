use serde::{Deserialize, Serialize};

// Re-export ObjectHandle from SDK layer
pub use editor_dto_law::ObjectHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}
