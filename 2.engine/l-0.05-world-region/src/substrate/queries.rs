use super::RegionSubstrate;
use crate::{
    ChunkDescriptor, DirtyChunkEntry, DirtyFlags, DirtyRegionSnapshot, RegionState, RegionVersion,
};
use engine_core::Tick;
use engine_world_spatial::{ChunkAddress, RegionAddress};
use std::collections::BTreeSet;

impl RegionSubstrate {
    /// Get current state of a region.
    pub fn region_state(&self, region: RegionAddress) -> Option<RegionState> {
        self.regions.get(&region).map(|r| r.state)
    }

    /// Get all active regions ordered by priority.
    pub fn active_regions(&self) -> Vec<RegionAddress> {
        let mut regions: Vec<_> = self
            .regions
            .iter()
            .filter(|(_, r)| r.state == RegionState::Active)
            .map(|(addr, r)| (*addr, r.priority))
            .collect();
        regions.sort_by_key(|(_, p)| p.0);
        regions.into_iter().map(|(addr, _)| addr).collect()
    }

    pub fn chunk_descriptor(&self, address: ChunkAddress) -> ChunkDescriptor {
        ChunkDescriptor {
            address,
            region: address.region,
        }
    }

    pub fn dirty_chunks(&self, region: RegionAddress) -> BTreeSet<ChunkAddress> {
        self.dirty_chunks
            .get(&region)
            .map(|v| v.keys().copied().collect())
            .unwrap_or_default()
    }

    pub fn dirty_flags(&self, chunk: ChunkAddress) -> Option<DirtyFlags> {
        self.dirty_chunks
            .get(&chunk.region)
            .and_then(|chunks| chunks.get(&chunk).map(|(flags, _)| *flags))
    }

    /// Get the tick when a chunk was last marked dirty.
    pub fn dirty_tick(&self, chunk: ChunkAddress) -> Option<Tick> {
        self.dirty_chunks
            .get(&chunk.region)
            .and_then(|chunks| chunks.get(&chunk).map(|(_, tick)| *tick))
    }

    pub fn dirty_snapshot(&self, region: RegionAddress) -> Option<DirtyRegionSnapshot> {
        let version = self.region_version(region)?;
        let chunks: Vec<_> = self
            .dirty_chunks
            .get(&region)
            .map(|entries| {
                entries
                    .iter()
                    .map(|(chunk, (flags, tick))| DirtyChunkEntry {
                        chunk: *chunk,
                        flags: *flags,
                        dirty_tick: *tick,
                    })
                    .collect()
            })
            .unwrap_or_default();
        let total_dirty_count = chunks.len() as u32;
        Some(DirtyRegionSnapshot {
            region,
            version,
            chunks,
            total_dirty_count,
        })
    }

    pub fn region_version(&self, region: RegionAddress) -> Option<RegionVersion> {
        self.regions.get(&region).map(|r| r.version)
    }

    /// Get region descriptor with full metadata.
    pub fn region_descriptor(&self, region: RegionAddress) -> Option<&crate::RegionDescriptor> {
        self.regions.get(&region)
    }
}
