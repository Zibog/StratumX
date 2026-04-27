//! Query interfaces for region inspection and traversal.

use crate::{DirtyChunkEntry, DirtyRegionSnapshot, RegionAddress, RegionDescriptor, RegionState};
use engine_core::Tick;
use std::collections::BTreeMap;

/// Query interface for single region lookup and inspection.
pub struct RegionQuery<'a> {
    regions: &'a BTreeMap<RegionAddress, RegionDescriptor>,
}

impl<'a> RegionQuery<'a> {
    /// Create a new region query from a regions map.
    pub fn new(regions: &'a BTreeMap<RegionAddress, RegionDescriptor>) -> Self {
        Self { regions }
    }

    /// Get a region by address.
    pub fn get(&self, address: RegionAddress) -> Option<&RegionDescriptor> {
        self.regions.get(&address)
    }

    /// Check if a region exists.
    pub fn exists(&self, address: RegionAddress) -> bool {
        self.regions.contains_key(&address)
    }

    /// Get all regions in a specific state.
    pub fn regions_in_state(&self, state: RegionState) -> Vec<&RegionDescriptor> {
        self.regions.values().filter(|r| r.state == state).collect()
    }

    /// Count total regions.
    pub fn total_count(&self) -> usize {
        self.regions.len()
    }

    /// Get regions ordered by priority.
    pub fn by_priority(&self) -> Vec<&RegionDescriptor> {
        let mut regions: Vec<_> = self.regions.values().collect();
        regions.sort_by_key(|r| r.priority);
        regions
    }
}

/// Query interface for dirty chunk tracking and snapshots.
pub struct DirtyRegionQuery<'a> {
    dirty_chunks:
        &'a BTreeMap<RegionAddress, BTreeMap<crate::ChunkAddress, (crate::DirtyFlags, Tick)>>,
}

impl<'a> DirtyRegionQuery<'a> {
    /// Create a new dirty region query.
    pub fn new(
        dirty_chunks: &'a BTreeMap<
            RegionAddress,
            BTreeMap<crate::ChunkAddress, (crate::DirtyFlags, Tick)>,
        >,
    ) -> Self {
        Self { dirty_chunks }
    }

    /// Get a snapshot of dirty chunks in a region.
    pub fn snapshot(
        &self,
        region: RegionAddress,
        version: crate::RegionVersion,
    ) -> Option<DirtyRegionSnapshot> {
        let dirty_map = self.dirty_chunks.get(&region)?;
        let chunks = dirty_map
            .iter()
            .map(|(chunk, (flags, tick))| DirtyChunkEntry {
                chunk: *chunk,
                flags: *flags,
                dirty_tick: *tick,
            })
            .collect::<Vec<_>>();

        let total_dirty_count = chunks.len() as u32;
        Some(DirtyRegionSnapshot {
            region,
            version,
            chunks,
            total_dirty_count,
        })
    }

    /// Check if a region has any dirty chunks.
    pub fn has_dirty(&self, region: RegionAddress) -> bool {
        self.dirty_chunks
            .get(&region)
            .map(|m: &BTreeMap<crate::ChunkAddress, (crate::DirtyFlags, Tick)>| !m.is_empty())
            .unwrap_or(false)
    }

    /// Count dirty chunks in a region.
    pub fn dirty_count(&self, region: RegionAddress) -> usize {
        self.dirty_chunks
            .get(&region)
            .map(|m: &BTreeMap<crate::ChunkAddress, (crate::DirtyFlags, Tick)>| m.len())
            .unwrap_or(0)
    }
}
