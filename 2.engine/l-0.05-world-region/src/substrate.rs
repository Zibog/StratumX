use crate::{DirtyFlags, RegionDescriptor};
use engine_core::Tick;
use engine_world_spatial::{ChunkAddress, RegionAddress};
use std::collections::BTreeMap;

mod dirty_tracking;
mod lifecycle;
mod queries;

/// Canonical region substrate owner.
/// **Owner**: engine_world_region — single source of truth for region lifecycle,
/// dirty tracking, and chunk-to-region mapping.
#[derive(Debug, Default)]
pub struct RegionSubstrate {
    pub(crate) regions: BTreeMap<RegionAddress, RegionDescriptor>,
    pub(crate) dirty_chunks: BTreeMap<RegionAddress, BTreeMap<ChunkAddress, (DirtyFlags, Tick)>>,
}
