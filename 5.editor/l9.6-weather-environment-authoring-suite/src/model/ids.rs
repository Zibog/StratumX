use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StormFrontId(pub u64);

impl StormFrontId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
