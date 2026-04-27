use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportConfig, NetTransportService, PacketDescriptor,
    PacketLane,
};
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

#[test]
fn open_session_assigns_monotonic_epochs() {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 64,
    });
    let a = transport.open_session(ConnectionHandle(1));
    let b = transport.open_session(ConnectionHandle(2));
    assert!(b.session_epoch > a.session_epoch);
}

#[test]
fn oversize_packet_is_rejected() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 2,
    });
    let result = transport.send(
        &mut runtime,
        NetPacketEnvelope {
            connection: ConnectionHandle(1),
            descriptor: PacketDescriptor {
                reliable: true,
                ordered: true,
            },
            lane: PacketLane::Bulk,
            payload: vec![1, 2, 3],
        },
    );
    assert!(result.is_err());
}
