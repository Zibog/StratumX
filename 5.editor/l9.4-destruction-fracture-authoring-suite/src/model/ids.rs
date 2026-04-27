use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectHandle(pub u64);

impl ObjectHandle {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}
