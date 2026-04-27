// Network Transport Service Tests

use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportConfig, NetTransportService, PacketDescriptor,
    PacketLane,
};
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

#[test]
fn test_net_transport_send() {
    let service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 100,
            publish_passes: 1,
        },
    );
    let metrics = service
        .send(
            &mut runtime,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1, 2, 3],
            },
        )
        .expect("send");
    assert_eq!(metrics.queued_packets, 1);
    assert_eq!(metrics.queued_bytes, 3);
}
