#[test]
fn memory_reservation_returns_receipt_with_tracked_allocation_id() {
    let config = MemoryConfig {
        heap_budget_bytes: 1024 * 1024,
        staging_budget_bytes: 512 * 1024,
    };
    let mut service = MemoryControlService::new(config);

    let descriptor = AllocationDescriptor {
        allocation_id: 11,
        bytes: 1024,
        layout_class: None,
    };

    let receipt = service.reserve_with_receipt(&descriptor).unwrap();
    assert_eq!(receipt.allocation_id.0, 11);
    assert_eq!(receipt.pool, MemoryAllocationPool::Heap);
    assert_eq!(receipt.bytes_reserved, 1024);
    assert_eq!(receipt.pressure_after, PressureClass::Healthy);
    assert!(service.allocation(receipt.allocation_id).is_some());
}

#[test]
fn memory_release_returns_receipt_and_clears_ledger_entry() {
    let config = MemoryConfig {
        heap_budget_bytes: 1024 * 1024,
        staging_budget_bytes: 512 * 1024,
    };
    let mut service = MemoryControlService::new(config);

    let descriptor = AllocationDescriptor {
        allocation_id: 12,
        bytes: 1024,
        layout_class: None,
    };

    let reserve_receipt = service.reserve_with_receipt(&descriptor).unwrap();
    let release_receipt = service
        .release_with_receipt(reserve_receipt.allocation_id, descriptor.bytes)
        .unwrap();

    assert_eq!(release_receipt.allocation_id, reserve_receipt.allocation_id);
    assert_eq!(release_receipt.pool, MemoryAllocationPool::Heap);
    assert_eq!(release_receipt.bytes_released, 1024);
    assert_eq!(release_receipt.pressure_after, PressureClass::Healthy);
    assert!(service.allocation(release_receipt.allocation_id).is_none());
}

#[test]
fn memory_pressure_signal_on_threshold_cross_publishes_degrade_bridge() {
    let config = MemoryConfig {
        heap_budget_bytes: 1000,
        staging_budget_bytes: 0,
    };
    let mut service = MemoryControlService::new(config);

    assert!(service.check_pressure_signal().is_none());

    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 700,
            layout_class: None,
        })
        .unwrap();

    let signal = service.check_pressure_signal().unwrap();
    assert_eq!(signal.previous, PressureClass::Healthy);
    assert_eq!(signal.current, PressureClass::Elevated);
    assert_eq!(signal.reserved_heap_bytes, 700);
    assert!(signal.degrade_bridge.degrade_lod);
    assert!(!signal.degrade_bridge.degrade_texture_resolution);
}

#[test]
fn headless_run_receipt_uses_canonical_tick_window() {
    let world = WorldState::new();
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 10,
        emit_snapshot_bytes: true,
    };
    let mut profile = HeadlessRuntimeProfile::new(world, config);

    let receipt = profile.run_with_receipt(5).unwrap();
    assert_eq!(receipt.ticks_executed, 5);
    assert!(receipt.snapshot_bytes > 0);
    assert_eq!(
        receipt.authoritative_window_ms,
        5 * u64::from(HEADLESS_TICK_BUDGET_MS)
    );
    assert_eq!(receipt.replay_digest, profile.replay_digest().unwrap());
}

#[test]
fn headless_snapshot_receipt_digest_is_stable_for_same_state() {
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 10,
        emit_snapshot_bytes: true,
    };

    let profile1 = HeadlessRuntimeProfile::new(WorldState::new(), config.clone());
    let profile2 = HeadlessRuntimeProfile::new(WorldState::new(), config);

    let receipt1 = profile1.snapshot_with_receipt().unwrap();
    let receipt2 = profile2.snapshot_with_receipt().unwrap();

    assert_eq!(receipt1.snapshot_bytes, receipt2.snapshot_bytes);
    assert_eq!(receipt1.replay_digest, receipt2.replay_digest);
}

#[test]
fn headless_replay_digest_changes_when_state_changes() {
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 10,
        emit_snapshot_bytes: false,
    };
    let mut advanced = HeadlessRuntimeProfile::new(WorldState::new(), config.clone());
    let baseline = HeadlessRuntimeProfile::new(WorldState::new(), config);

    advanced.step().unwrap();

    let before = baseline.replay_digest().unwrap();
    let after = advanced.replay_digest().unwrap();

    assert_ne!(before, after);
}

#[test]
fn realtime_frame_receipt_contains_monotonic_frame_id() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: true,
    };
    let mut profile = RealtimeRuntimeProfile::new(world, config).unwrap();

    let first = profile.step_with_receipt().unwrap();
    let second = profile.step_with_receipt().unwrap();

    assert_eq!(first.frame_id.0, 1);
    assert_eq!(second.frame_id.0, 2);
    assert_eq!(first.frame_budget_micros, second.frame_budget_micros);
}

#[test]
fn realtime_cadence_from_fps_is_deterministic_and_zero_rejected() {
    let cadence = RealtimeFrameCadence::from_fps(60).unwrap();
    assert_eq!(cadence.target_fps, 60);
    assert_eq!(cadence.frame_budget_micros, 16_666);

    let cadence30 = RealtimeFrameCadence::from_fps(30).unwrap();
    assert_eq!(cadence30.target_fps, 30);
    assert_eq!(cadence30.frame_budget_micros, 33_333);

    let zero = RealtimeFrameCadence::from_fps(0).unwrap_err();
    assert_eq!(zero.reason, RealtimeRuntimeFailureReason::InvalidTargetFps);
}

