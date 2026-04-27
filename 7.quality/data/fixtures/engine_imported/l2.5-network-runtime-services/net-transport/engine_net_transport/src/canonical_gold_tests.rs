#![allow(unused_imports)]
use super::*;

#[test]
fn open_session_assigns_epoch() {
    let mut s = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    let session = s.open_session(ConnectionHandle(1));
    assert_eq!(session.connection, ConnectionHandle(1));
}
#[test]
fn send_rejects_oversized_packet() {
    let mut runtime = engine_runtime::RuntimeKernel::new(
        engine_world::WorldState::new(),
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let s = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1,
    });
    assert!(s
        .send(
            &mut runtime,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true
                },
                lane: PacketLane::Control,
                payload: vec![1, 2]
            }
        )
        .is_err());
}
#[test]
fn send_queues_publication() {
    let mut runtime = engine_runtime::RuntimeKernel::new(
        engine_world::WorldState::new(),
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let s = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    let m = s
        .send(
            &mut runtime,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn send_reports_queued_bytes() {
    let mut runtime = engine_runtime::RuntimeKernel::new(
        engine_world::WorldState::new(),
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let s = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    let m = s
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
    assert_eq!(m.queued_bytes, 3);
}
#[test]
fn send_accepts_control_lane() {
    let mut runtime = engine_runtime::RuntimeKernel::new(
        engine_world::WorldState::new(),
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let s = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    assert!(s
        .send(
            &mut runtime,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true
                },
                lane: PacketLane::Control,
                payload: vec![1]
            }
        )
        .is_ok());
}
