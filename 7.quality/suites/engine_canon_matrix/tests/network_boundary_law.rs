// Network Boundary Law Tests
//
// Tests for BLKR-NET-BOUNDARY-01:
// - Transport packet validation
// - Delivery verdict
// - Latency bucket classification
// - Sync window
// - Rewind window
// - Sync digest determinism

use engine_core::Tick;
use engine_net_latency::{LatencyBucket, NetLatencyConfig, NetLatencyService, PredictionContext};
use engine_net_sync::{NetSyncConfig, NetSyncService, RewindWindow, SyncRejectReason, SyncWindow};
use engine_net_transport::{
    ConnectionHandle, DeliveryVerdict, NetTransportConfig, NetTransportService, NetworkLaneId,
    NetworkSessionId, PacketId, TransportPacket,
};
use engine_world::WorldState;

fn open_transport_session() -> (NetTransportService, engine_net_transport::TransportSession) {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = transport.open_session(ConnectionHandle(1));
    (transport, session)
}

include!("network_boundary_law/cases_01.rs");
include!("network_boundary_law/cases_02.rs");
