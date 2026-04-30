#[test]
fn snapshot_delta_ack_is_deterministic() {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = transport.open_session(ConnectionHandle(1));
    let mut sync = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });
    let world = WorldState::new();

    let (snapshot1, _) = sync
        .snapshot(&transport, &session, &world, vec![(0, 0, 0)])
        .unwrap();
    let (snapshot2, _) = sync
        .snapshot(&transport, &session, &world, vec![(0, 0, 0)])
        .unwrap();

    assert_eq!(
        sync.compute_digest(&snapshot1),
        sync.compute_digest(&snapshot2)
    );

    let mut runtime = runtime();
    let first = transport
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
    let second = transport
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
    let ack = transport.acknowledge(&session, second.sequence).unwrap();

    assert_eq!(first.sequence, 1);
    assert_eq!(second.sequence, 2);
    assert_eq!(ack.highest_acked_sequence, 2);
}

#[test]
fn delta_without_base_snapshot_fails() {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = transport.open_session(ConnectionHandle(1));
    let mut service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 8,
    });

    let result = service.delta(&transport, &session, SnapshotId(999), Tick(100), Tick(101));
    assert_eq!(result.unwrap_err(), SyncRejectReason::MissingBaseSnapshot);
}

#[test]
fn interest_region_overflow_fails() {
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let session = transport.open_session(ConnectionHandle(1));
    let mut service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 10,
    });

    let large_region = vec![(0, 0, 0); 1001];
    let result = service.snapshot(&transport, &session, &WorldState::new(), large_region);
    assert_eq!(
        result.unwrap_err(),
        SyncRejectReason::InterestRegionOverflow
    );
}

#[test]
fn prediction_outside_rewind_window_fails() {
    let mut history = PredictionHistory::new(10);
    history.add_input(PredictionInput {
        tick: 100,
        input_data: vec![1, 2, 3],
    });

    let window = RewindWindow {
        max_rewind_ticks: 5,
    };

    assert!(history.validate_rewind(100, &window).is_ok());
    assert!(history.validate_rewind(105, &window).is_ok());
    assert_eq!(
        history.validate_rewind(106, &window).unwrap_err(),
        PredictionRejectReason::OutsideRewindWindow
    );
}

#[test]
fn reconcile_emits_authoritative_correction_receipt() {
    let service = NetLatencyService::new(NetLatencyConfig { history_size: 10 }).unwrap();
    let result = service.reconcile(engine_net_latency::PredictionContext {
        predicted_tick: Tick(103),
        authoritative_tick: Tick(100),
    });

    let receipt = result.correction.expect("correction receipt");
    assert!(result.rewound);
    assert_eq!(receipt.corrected_tick, 100);
    assert_eq!(receipt.delta_ticks, 3);
    assert_eq!(receipt.reason, CorrectionReason::Misprediction);
    assert_ne!(receipt.digest, 0);
}

#[test]
fn delivery_ack_closes_packet_window() {
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

    let ack_one = service.acknowledge(&session, first.sequence).unwrap();
    assert_eq!(ack_one.in_flight_packets, 1);

    let ack_two = service.acknowledge(&session, second.sequence).unwrap();
    assert_eq!(ack_two.in_flight_packets, 0);
}

