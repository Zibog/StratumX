use super::RegionSubstrate;
use crate::DirtyFlags;
use engine_core::Tick;
use engine_world_spatial::ChunkAddress;

impl RegionSubstrate {
    /// Mark a chunk as dirty with a specific tick.
    /// **Canon rule**: region ownership is established before dirty tracking.
    pub fn mark_dirty(&mut self, chunk: ChunkAddress, flags: DirtyFlags, tick: Tick) {
        self.register_region(chunk.region, tick);
        self.dirty_chunks
            .entry(chunk.region)
            .or_default()
            .entry(chunk)
            .and_modify(|(existing_flags, existing_tick)| {
                *existing_flags |= flags;
                if tick > *existing_tick {
                    *existing_tick = tick;
                }
            })
            .or_insert((flags, tick));

        if let Some(region) = self.regions.get_mut(&chunk.region) {
            region.version.epoch = region.version.epoch.saturating_add(1);
            region.version.last_dirty_tick = tick;
        }
    }

    /// Clear dirty state for a specific chunk.
    pub fn clear_chunk_dirty(&mut self, chunk: ChunkAddress) {
        if let Some(chunks) = self.dirty_chunks.get_mut(&chunk.region) {
            chunks.remove(&chunk);
            if chunks.is_empty() {
                self.dirty_chunks.remove(&chunk.region);
            }
        }
    }

    /// Clear all dirty state for a region.
    pub fn clear_region_dirty(&mut self, region: engine_world_spatial::RegionAddress) {
        self.dirty_chunks.remove(&region);
    }
}
