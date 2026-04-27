use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportConfig, NetTransportService, PacketDescriptor,
    PacketLane,
};
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

#[test]
fn net_transport_queues_payload_into_runtime() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 64,
    });
    transport.open_session(ConnectionHandle(1));
    let metrics = transport
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
        .unwrap();
    assert_eq!(metrics.queued_packets, 1);
}
