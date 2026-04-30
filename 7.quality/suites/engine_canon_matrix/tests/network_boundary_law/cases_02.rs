#[test]
fn different_sync_inputs_change_digest() {
    let (transport, session) = open_transport_session();
    let config = NetSyncConfig {
        max_interest_regions: 10,
    };
    let mut service = NetSyncService::new(config);

    let world = WorldState::new();
    let interest1 = vec![(0, 0, 0), (1, 1, 1)];
    let interest2 = vec![(0, 0, 0), (2, 2, 2)];

    let (snapshot1, _) = service
        .snapshot(&transport, &session, &world, interest1)
        .unwrap();
    let (snapshot2, _) = service
        .snapshot(&transport, &session, &world, interest2)
        .unwrap();

    let digest1 = service.compute_digest(&snapshot1);
    let digest2 = service.compute_digest(&snapshot2);

    assert_ne!(digest1, digest2);
}

#[test]
fn invalid_interest_region_count_rejected() {
    let (transport, session) = open_transport_session();
    let config = NetSyncConfig {
        max_interest_regions: 5,
    };
    let mut service = NetSyncService::new(config);

    let world = WorldState::new();
    let interest = vec![(0, 0, 0); 10];

    let result = service.snapshot(&transport, &session, &world, interest);
    assert_eq!(
        result.unwrap_err(),
        SyncRejectReason::InterestRegionOverflow
    );
}

#[test]
fn latency_service_rejects_zero_history_size() {
    let config = NetLatencyConfig { history_size: 0 };
    let result = NetLatencyService::new(config);
    assert!(result.is_err());
}

#[test]
fn reconcile_detects_prediction_mismatch() {
    let config = NetLatencyConfig { history_size: 10 };
    let service = NetLatencyService::new(config).unwrap();

    let context = PredictionContext {
        predicted_tick: Tick(105),
        authoritative_tick: Tick(100),
    };

    let result = service.reconcile(context);
    assert!(result.rewound);
    assert_eq!(result.delta_ticks, 5);
    let correction = result.correction.expect("correction receipt");
    assert_eq!(correction.corrected_tick, 100);
    assert_eq!(correction.delta_ticks, 5);
}

#[test]
fn reconcile_no_rewind_when_ticks_match() {
    let config = NetLatencyConfig { history_size: 10 };
    let service = NetLatencyService::new(config).unwrap();

    let context = PredictionContext {
        predicted_tick: Tick(100),
        authoritative_tick: Tick(100),
    };

    let result = service.reconcile(context);
    assert!(!result.rewound);
    assert_eq!(result.delta_ticks, 0);
    assert!(result.correction.is_none());
}

#[test]
fn sync_window_validation_through_service() {
    let service = NetSyncService::new(NetSyncConfig {
        max_interest_regions: 10,
    });

    let window = SyncWindow {
        current_tick: Tick(100),
        window_size: 10,
    };

    assert!(service.validate_sync_window(&window, Tick(95)));
    assert!(service.validate_sync_window(&window, Tick(105)));
    assert!(!service.validate_sync_window(&window, Tick(89)));
}
