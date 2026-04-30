use engine_core::Tick;
use engine_net_transport::NetworkSessionId;
use engine_world::WorldSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetSyncConfig {
    pub max_interest_regions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncState {
    pub authority_tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncAuthority {
    pub authoritative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncRollback {
    pub from_tick: Tick,
    pub to_tick: Tick,
}

/// Snapshot identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub u64);

/// Delta identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeltaId(pub u64);

/// Snapshot sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SnapshotSequence(pub u64);

/// Delta sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeltaSequence(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncSnapshot {
    pub snapshot_id: SnapshotId,
    pub session_id: NetworkSessionId,
    pub session_epoch: u64,
    pub world_snapshot: WorldSnapshot,
    pub interest_region: Vec<(i32, i32, i32)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDelta {
    pub delta_id: DeltaId,
    pub session_id: NetworkSessionId,
    pub session_epoch: u64,
    pub base_snapshot_id: SnapshotId,
    pub from_tick: Tick,
    pub to_tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetSyncMetrics {
    pub snapshot_bytes: usize,
    pub interest_region_count: usize,
}

/// Sync acknowledgment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncAck {
    pub snapshot_id: SnapshotId,
    pub delta_id: Option<DeltaId>,
    pub sequence: u64,
}

/// Sync delivery receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDeliveryReceipt {
    pub snapshot_id: SnapshotId,
    pub ack: SyncAck,
    pub digest: u64,
}

/// Sync reject reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncRejectReason {
    InvalidSnapshot,
    InvalidDelta,
    MissingBaseSnapshot,
    InterestRegionOverflow,
    InactiveSession,
    StaleSessionEpoch,
}

impl SyncSnapshot {
    /// Validate snapshot.
    pub fn validate(&self) -> Result<(), SyncRejectReason> {
        if self.snapshot_id.0 == 0 {
            return Err(SyncRejectReason::InvalidSnapshot);
        }
        if self.session_id.0 == 0 || self.session_epoch == 0 {
            return Err(SyncRejectReason::InactiveSession);
        }
        if self.interest_region.len() > 1000 {
            return Err(SyncRejectReason::InterestRegionOverflow);
        }
        Ok(())
    }
}

impl SyncDelta {
    /// Validate delta.
    pub fn validate(&self) -> Result<(), SyncRejectReason> {
        if self.delta_id.0 == 0 {
            return Err(SyncRejectReason::InvalidDelta);
        }
        if self.base_snapshot_id.0 == 0 {
            return Err(SyncRejectReason::MissingBaseSnapshot);
        }
        if self.session_id.0 == 0 || self.session_epoch == 0 {
            return Err(SyncRejectReason::InactiveSession);
        }
        Ok(())
    }
}
