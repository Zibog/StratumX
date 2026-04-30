#[test]
fn valid_transport_packet_is_accepted() {
    let (mut service, session) = open_transport_session();
    let packet = TransportPacket {
        packet_id: PacketId(1),
        session_id: session.session_id,
        session_epoch: session.session_epoch,
        lane_id: NetworkLaneId(0),
        sequence: 1,
        payload_digest: 12345,
        timestamp_ms: 1000,
    };

    let verdict = service.process_packet(&packet);
    assert_eq!(verdict, DeliveryVerdict::Accepted);
}

#[test]
fn duplicate_packet_receives_duplicate_verdict() {
    let (mut service, session) = open_transport_session();
    let packet = TransportPacket {
        packet_id: PacketId(1),
        session_id: session.session_id,
        session_epoch: session.session_epoch,
        lane_id: NetworkLaneId(0),
        sequence: 1,
        payload_digest: 12345,
        timestamp_ms: 1000,
    };

    assert_eq!(service.process_packet(&packet), DeliveryVerdict::Accepted);
    assert_eq!(service.process_packet(&packet), DeliveryVerdict::Duplicate);
}

#[test]
fn invalid_network_lane_is_rejected() {
    let (mut service, session) = open_transport_session();
    let packet = TransportPacket {
        packet_id: PacketId(1),
        session_id: session.session_id,
        session_epoch: session.session_epoch,
        lane_id: NetworkLaneId(99),
        sequence: 1,
        payload_digest: 12345,
        timestamp_ms: 1000,
    };

    let verdict = service.process_packet(&packet);
    assert_eq!(verdict, DeliveryVerdict::InvalidLane);
}

#[test]
fn invalid_session_is_rejected() {
    let (mut service, session) = open_transport_session();
    let packet = TransportPacket {
        packet_id: PacketId(1),
        session_id: NetworkSessionId(session.session_id.0 + 999),
        session_epoch: session.session_epoch,
        lane_id: NetworkLaneId(0),
        sequence: 1,
        payload_digest: 12345,
        timestamp_ms: 1000,
    };

    let verdict = service.process_packet(&packet);
    assert_eq!(verdict, DeliveryVerdict::InvalidSession);
}

#[test]
fn zero_session_id_is_rejected() {
    let (mut service, session) = open_transport_session();
    let packet = TransportPacket {
        packet_id: PacketId(1),
        session_id: NetworkSessionId(0),
        session_epoch: session.session_epoch,
        lane_id: NetworkLaneId(0),
        sequence: 1,
        payload_digest: 12345,
        timestamp_ms: 1000,
    };

    let verdict = service.process_packet(&packet);
    assert_eq!(verdict, DeliveryVerdict::InvalidSession);
}

#[test]
fn latency_bucket_classification_is_stable() {
    let config = NetLatencyConfig { history_size: 10 };
    let mut service = NetLatencyService::new(config.clone()).unwrap();

    service.record_sample(30);
    service.record_sample(40);
    let metrics = service.metrics();
    assert_eq!(metrics.bucket, LatencyBucket::Low);

    let mut service2 = NetLatencyService::new(config.clone()).unwrap();
    service2.record_sample(80);
    service2.record_sample(100);
    let metrics2 = service2.metrics();
    assert_eq!(metrics2.bucket, LatencyBucket::Medium);

    let mut service3 = NetLatencyService::new(config).unwrap();
    service3.record_sample(150);
    service3.record_sample(200);
    let metrics3 = service3.metrics();
    assert_eq!(metrics3.bucket, LatencyBucket::High);
}

#[test]
fn same_latency_samples_produce_same_bucket() {
    let config = NetLatencyConfig { history_size: 10 };

    let mut service1 = NetLatencyService::new(config.clone()).unwrap();
    service1.record_sample(50);
    service1.record_sample(60);
    let metrics1 = service1.metrics();

    let mut service2 = NetLatencyService::new(config).unwrap();
    service2.record_sample(50);
    service2.record_sample(60);
    let metrics2 = service2.metrics();

    assert_eq!(metrics1.bucket, metrics2.bucket);
    assert_eq!(
        metrics1.estimate.round_trip_ms,
        metrics2.estimate.round_trip_ms
    );
}

#[test]
fn sync_window_rejects_out_of_window_packet() {
    let window = SyncWindow {
        current_tick: Tick(100),
        window_size: 10,
    };

    assert!(window.contains(Tick(95)));
    assert!(window.contains(Tick(100)));
    assert!(window.contains(Tick(105)));
    assert!(!window.contains(Tick(89)));
    assert!(!window.contains(Tick(111)));
}

#[test]
fn rewind_window_validates_replayable_range() {
    let window = RewindWindow {
        max_rewind_ticks: 20,
    };

    assert!(window.is_valid_rewind(Tick(100), Tick(85)));
    assert!(window.is_valid_rewind(Tick(100), Tick(80)));
    assert!(!window.is_valid_rewind(Tick(100), Tick(79)));
    assert!(!window.is_valid_rewind(Tick(100), Tick(50)));
}

#[test]
fn same_sync_inputs_produce_same_digest() {
    let (transport, session) = open_transport_session();
    let config = NetSyncConfig {
        max_interest_regions: 10,
    };
    let mut service = NetSyncService::new(config);

    let world = WorldState::new();
    let interest = vec![(0, 0, 0), (1, 1, 1)];

    let (snapshot1, _) = service
        .snapshot(&transport, &session, &world, interest.clone())
        .unwrap();
    let (snapshot2, _) = service
        .snapshot(&transport, &session, &world, interest.clone())
        .unwrap();

    let digest1 = service.compute_digest(&snapshot1);
    let digest2 = service.compute_digest(&snapshot2);

    assert_eq!(digest1, digest2);
}

