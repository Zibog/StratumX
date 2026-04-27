use crate::{NetSyncConfig, NetSyncMetrics, SyncDelta, SyncSnapshot};
use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportMetrics, NetTransportService,
    PacketDescriptor, PacketLane,
};
use engine_world::WorldState;

#[derive(Debug, Clone)]
pub struct NetSyncService {
    config: NetSyncConfig,
}

impl NetSyncService {
    pub fn new(config: NetSyncConfig) -> Self {
        Self { config }
    }

    pub fn snapshot(
        &self,
        world: &WorldState,
        interest_regions: Vec<(i32, i32, i32)>,
    ) -> EngineCoreResult<(SyncSnapshot, NetSyncMetrics)> {
        if interest_regions.len() > self.config.max_interest_regions {
            return Err(EngineCoreError::InvalidDescriptor(
                "interest region count exceeds configured ceiling",
            ));
        }
        let snapshot = SyncSnapshot {
            world_snapshot: world.snapshot(interest_regions.len()),
            interest_regions,
        };
        let bytes = bincode::serialize(&snapshot).map_err(|_| {
            EngineCoreError::InvalidDescriptor("sync snapshot serialization failed")
        })?;
        Ok((
            snapshot.clone(),
            NetSyncMetrics {
                snapshot_bytes: bytes.len(),
                interest_region_count: snapshot.interest_regions.len(),
            },
        ))
    }

    pub fn queue_snapshot(
        &self,
        transport: &NetTransportService,
        runtime: &mut engine_runtime::RuntimeKernel,
        connection: ConnectionHandle,
        snapshot: &SyncSnapshot,
    ) -> EngineCoreResult<NetTransportMetrics> {
        let payload = bincode::serialize(snapshot)
            .map_err(|_| EngineCoreError::InvalidDescriptor("sync packet serialization failed"))?;
        transport.send(
            runtime,
            NetPacketEnvelope {
                connection,
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::State,
                payload,
            },
        )
    }

    pub fn delta(&self, from_tick: Tick, to_tick: Tick) -> SyncDelta {
        SyncDelta { from_tick, to_tick }
    }
}
