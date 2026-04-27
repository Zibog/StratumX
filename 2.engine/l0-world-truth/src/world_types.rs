use engine_core::Tick;
use serde::{Deserialize, Serialize};

pub const MAX_SEGMENTS_PER_TICK: usize = 256;
pub const MAX_FAMILY_FANOUT_PER_SEGMENT: usize = 8;
pub const MAX_PUBLISH_PASSES: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplySegment {
    pub region_key: (i32, i32, i32),
    pub family_tags: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: Tick,
    pub epoch: u64,
    pub segment_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadModel {
    pub tick: Tick,
    pub epoch: u64,
}
