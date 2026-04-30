use crate::{NetSyncService, SyncSnapshot};
use serde::{Deserialize, Serialize};

/// Sync window for packet acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncWindow {
    pub current_tick: engine_core::Tick,
    pub window_size: u32,
}

impl SyncWindow {
    pub fn contains(&self, tick: engine_core::Tick) -> bool {
        let current = self.current_tick.0;
        let target = tick.0;
        let window = self.window_size as u64;
        target >= current.saturating_sub(window) && target <= current.saturating_add(window)
    }
}

/// Rewind window for replay validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewindWindow {
    pub max_rewind_ticks: u32,
}

impl RewindWindow {
    pub fn is_valid_rewind(
        &self,
        from_tick: engine_core::Tick,
        to_tick: engine_core::Tick,
    ) -> bool {
        let distance = from_tick.0.saturating_sub(to_tick.0);
        distance <= self.max_rewind_ticks as u64
    }
}

/// Sync digest for deterministic comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDigest(pub u64);

impl NetSyncService {
    pub fn compute_digest(&self, snapshot: &SyncSnapshot) -> SyncDigest {
        let mut builder = engine_core::StableDigestBuilder::new();
        builder.write_bytes(b"engine_net_sync.snapshot.v1");
        builder.write_u64(snapshot.world_snapshot.tick.0);
        builder.write_u64(snapshot.world_snapshot.world_id.0);
        builder.write_u64(snapshot.world_snapshot.epoch);
        builder.write_u64(snapshot.world_snapshot.segment_count as u64);
        builder.write_u64(snapshot.interest_region.len() as u64);
        for region in &snapshot.interest_region {
            builder.write_u32(region.0 as u32);
            builder.write_u32(region.1 as u32);
            builder.write_u32(region.2 as u32);
        }
        SyncDigest(builder.finish().0)
    }

    pub fn validate_sync_window(&self, window: &SyncWindow, tick: engine_core::Tick) -> bool {
        window.contains(tick)
    }
}
