#[test]
fn send_without_open_session_fails() {
    let mut service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let mut runtime = runtime();
    let stale_session = TransportSession {
        connection: ConnectionHandle(1),
        session_id: engine_net_transport::NetworkSessionId(1),
        session_epoch: 1,
        state: engine_net_transport::SessionState::Active,
    };

    let result = service.send_with_session(
        &mut runtime,
        &stale_session,
        NetPacketEnvelope {
            connection: ConnectionHandle(1),
            descriptor: PacketDescriptor {
                reliable: true,
                ordered: true,
            },
            lane: PacketLane::Control,
            payload: vec![1, 2, 3],
        },
    );

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("session not active"));
}

#[test]
fn send_after_close_session_fails() {
    let mut service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = service.open_session(ConnectionHandle(1));
    service.close_session(ConnectionHandle(1));

    let mut runtime = runtime();
    let result = service.send_with_session(
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
    );

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("session not active"));
}

#[test]
fn control_lane_requires_reliable_ordered() {
    let policy = LanePolicy::for_lane(PacketLane::Control);
    assert!(policy.requires_reliable);
    assert!(policy.requires_ordered);

    let valid_descriptor = PacketDescriptor {
        reliable: true,
        ordered: true,
    };
    assert!(policy.validate(&valid_descriptor).is_ok());

    let invalid_descriptor = PacketDescriptor {
        reliable: false,
        ordered: true,
    };
    assert!(policy.validate(&invalid_descriptor).is_err());
}

#[test]
fn state_lane_sequence_is_monotonic() {
    let mut service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = service.open_session(ConnectionHandle(1));
    let mut runtime = runtime();

    let first = service
        .send_with_session(
            &mut runtime,
            &session,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: false,
                },
                lane: PacketLane::State,
                payload: vec![1],
            },
        )
        .unwrap();
    let second = service
        .send_with_session(
            &mut runtime,
            &session,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: false,
                },
                lane: PacketLane::State,
                payload: vec![2],
            },
        )
        .unwrap();

    assert_eq!(first.sequence, 1);
    assert_eq!(second.sequence, 2);
    assert!(second.sequence > first.sequence);
}

#[test]
fn oversized_packet_is_rejected_with_reason() {
    let mut service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 4,
    });
    let session = service.open_session(ConnectionHandle(1));
    let mut runtime = runtime();

    let result = service.send_with_session(
        &mut runtime,
        &session,
        NetPacketEnvelope {
            connection: ConnectionHandle(1),
            descriptor: PacketDescriptor {
                reliable: true,
                ordered: true,
            },
            lane: PacketLane::Control,
            payload: vec![1, 2, 3, 4, 5],
        },
    );

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("packet exceeds transport packet ceiling"));
}

