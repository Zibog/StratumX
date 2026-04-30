use crate::{
    DeltaId, NetSyncConfig, NetSyncMetrics, SnapshotId, SyncDelta, SyncRejectReason, SyncSnapshot,
};
use engine_core::{EngineCoreError, Tick};
use engine_net_transport::{
    NetPacketEnvelope, NetTransportService, NetworkFailureReason, PacketDeliveryReceipt,
    PacketDescriptor, PacketLane, TransportSession,
};
use engine_world::WorldState;
use std::collections::BTreeMap;

const SYNC_PACKET_SERIALIZATION_FAILURE: &str = "sync packet serialization failed";

#[derive(Debug, Clone)]
pub struct NetSyncService {
    config: NetSyncConfig,
    next_snapshot_id: u64,
    next_delta_id: u64,
    snapshot_store: BTreeMap<SnapshotId, SyncSnapshot>,
}

impl NetSyncService {
    pub fn new(config: NetSyncConfig) -> Self {
        Self {
            config,
            next_snapshot_id: 1,
            next_delta_id: 1,
            snapshot_store: BTreeMap::new(),
        }
    }

    pub fn snapshot(
        &mut self,
        transport: &NetTransportService,
        session: &TransportSession,
        world: &WorldState,
        interest_regions: Vec<(i32, i32, i32)>,
    ) -> Result<(SyncSnapshot, NetSyncMetrics), SyncRejectReason> {
        ensure_active_session(transport, session)?;

        if interest_regions.len() > self.config.max_interest_regions {
            return Err(SyncRejectReason::InterestRegionOverflow);
        }

        let snapshot_id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;

        let snapshot = SyncSnapshot {
            snapshot_id,
            session_id: session.session_id,
            session_epoch: session.session_epoch,
            world_snapshot: world.snapshot(interest_regions.len()),
            interest_region: interest_regions,
        };
        snapshot.validate()?;

        let bytes = bincode::serialize(&snapshot).map_err(|_| SyncRejectReason::InvalidSnapshot)?;
        self.snapshot_store.insert(snapshot_id, snapshot.clone());

        Ok((
            snapshot.clone(),
            NetSyncMetrics {
                snapshot_bytes: bytes.len(),
                interest_region_count: snapshot.interest_region.len(),
            },
        ))
    }

    pub fn queue_snapshot(
        &self,
        transport: &mut NetTransportService,
        runtime: &mut engine_runtime::RuntimeKernel,
        session: &TransportSession,
        snapshot: &SyncSnapshot,
    ) -> Result<PacketDeliveryReceipt, EngineCoreError> {
        let payload = bincode::serialize(snapshot)
            .map_err(|_| EngineCoreError::InvalidDescriptor(SYNC_PACKET_SERIALIZATION_FAILURE))?;
        transport.send_with_session(
            runtime,
            session,
            NetPacketEnvelope {
                connection: session.connection,
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: false,
                },
                lane: PacketLane::State,
                payload,
            },
        )
    }

    pub fn delta(
        &mut self,
        transport: &NetTransportService,
        session: &TransportSession,
        base_snapshot: SnapshotId,
        from_tick: Tick,
        to_tick: Tick,
    ) -> Result<SyncDelta, SyncRejectReason> {
        ensure_active_session(transport, session)?;

        let Some(snapshot) = self.snapshot_store.get(&base_snapshot) else {
            return Err(SyncRejectReason::MissingBaseSnapshot);
        };
        if snapshot.session_id != session.session_id {
            return Err(SyncRejectReason::MissingBaseSnapshot);
        }
        if snapshot.session_epoch != session.session_epoch {
            return Err(SyncRejectReason::StaleSessionEpoch);
        }

        let delta_id = DeltaId(self.next_delta_id);
        self.next_delta_id += 1;

        let delta = SyncDelta {
            delta_id,
            session_id: session.session_id,
            session_epoch: session.session_epoch,
            base_snapshot_id: base_snapshot,
            from_tick,
            to_tick,
        };
        delta.validate()?;
        Ok(delta)
    }
}

fn ensure_active_session(
    transport: &NetTransportService,
    session: &TransportSession,
) -> Result<(), SyncRejectReason> {
    transport
        .validate_active_session(session)
        .map_err(|reason| match reason {
            NetworkFailureReason::StaleSessionEpoch => SyncRejectReason::StaleSessionEpoch,
            _ => SyncRejectReason::InactiveSession,
        })
}
