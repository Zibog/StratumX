// Network Transport Service Tests

use engine_net_transport::{
    ConnectionHandle, DeliveryVerdict, NetPacketEnvelope, NetTransportConfig, NetTransportService,
    PacketDescriptor, PacketLane,
};
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

#[test]
fn test_net_transport_send() {
    let mut service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = service.open_session(ConnectionHandle(1));
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 100,
            publish_passes: 1,
        },
    );

    let receipt = service
        .send_with_session(
            &mut runtime,
            &session,
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

    assert_eq!(receipt.verdict, DeliveryVerdict::Accepted);
    assert_eq!(receipt.sequence, 1);
    assert_eq!(receipt.ack_window.highest_sent_sequence, 1);
    assert_eq!(receipt.ack_window.in_flight_packets, 1);
}
