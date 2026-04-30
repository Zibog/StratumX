use engine_core::Tick;
use engine_net_latency::{
    CorrectionReason, NetLatencyConfig, NetLatencyService, PredictionHistory, PredictionInput,
    PredictionRejectReason, RewindWindow,
};
use engine_net_sync::{NetSyncConfig, NetSyncService, SnapshotId, SyncRejectReason};
use engine_net_transport::{
    ConnectionHandle, DeliveryVerdict, LanePolicy, NetPacketEnvelope, NetTransportConfig,
    NetTransportService, PacketDescriptor, PacketLane, TransportPacket, TransportSession,
};
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

fn runtime() -> RuntimeKernel {
    RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 8,
            publish_passes: 1,
        },
    )
}

include!("network_runtime_services_law/cases_01.rs");
include!("network_runtime_services_law/cases_02.rs");
include!("network_runtime_services_law/cases_03.rs");
