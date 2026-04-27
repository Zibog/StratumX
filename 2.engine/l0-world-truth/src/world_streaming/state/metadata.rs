use crate::world_streaming::mapping::RegionKey;
use crate::world_streaming::residency::ResidencyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionMetadata {
    pub key: RegionKey,
    pub residency_state: ResidencyState,
    pub memory_bytes: usize,
    pub last_access_time: f32,
    pub priority: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    LeastRecentlyUsed,
    LeastPriority,
    FarthestFromPlayer,
}
