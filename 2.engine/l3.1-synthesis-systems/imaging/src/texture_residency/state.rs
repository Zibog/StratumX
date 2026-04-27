use super::types::TextureDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureResidencyState {
    NotLoaded,
    Streaming,
    Resident,
    Evicted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureResidencyInfo {
    pub descriptor: TextureDescriptor,
    pub state: TextureResidencyState,
    pub last_access_frame: u64,
    pub priority: u32,
}
