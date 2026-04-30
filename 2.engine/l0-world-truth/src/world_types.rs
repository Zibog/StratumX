use crate::WorldId;
use engine_core::Tick;
use engine_world_spatial::{FarPhenomenonTrackRef, PrecisionZoneCode, RegionFrameRef};
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
    pub world_id: WorldId,
    pub tick: Tick,
    pub epoch: u64,
    pub segment_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppliedSegmentRecord {
    pub region_key: (i32, i32, i32),
    pub family_tags: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldApplyJournal {
    pub tick: Tick,
    pub epoch: u64,
    pub publish_passes: usize,
    pub segment_count: usize,
    pub segments: Vec<AppliedSegmentRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldCausalSummary {
    pub region_keys: Vec<(i32, i32, i32)>,
    pub family_tags: Vec<u16>,
    pub publish_passes: usize,
    pub near_region_frame_ref: Option<RegionFrameRef>,
    pub far_phenomenon_track_refs: Vec<FarPhenomenonTrackRef>,
    pub precision_zone_code: PrecisionZoneCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadModel {
    pub tick: Tick,
    pub epoch: u64,
}
