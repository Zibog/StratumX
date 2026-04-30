#[test]
fn realtime_zero_fps_rejected() {
    let world = WorldState::new();
    let result = RealtimeRuntimeProfile::new(
        world,
        RealtimeRuntimeConfig {
            target_fps: 0, // Invalid
            visibility_freshness_frames: 2,
            enqueue_presentable_frames: true,
        },
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        EngineCoreError::InvalidDescriptor(msg) => {
            assert!(msg.contains("non-zero"));
        }
        _ => panic!("Expected InvalidDescriptor error"),
    }
}

#[test]
fn memory_pressure_escalates_deterministically() {
    let config = MemoryConfig {
        heap_budget_bytes: 1024 * 1024,   // 1MB
        staging_budget_bytes: 512 * 1024, // 512KB
    };

    let mut service = MemoryControlService::new(config);

    // Healthy state
    let metrics = service.metrics();
    assert_eq!(metrics.pressure, PressureClass::Healthy);

    // Reserve to elevated (70-89% of total)
    // Total budget = 1536KB, 70% = 1075KB
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 800 * 1024, // 800KB heap
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 300 * 1024, // 300KB staging = 1100KB total (~71%)
            layout_class: None,
        })
        .unwrap();

    let metrics = service.metrics();
    assert_eq!(metrics.pressure, PressureClass::Elevated);

    // Reserve more to critical (90%+)
    // Need 1382KB total for 90%, currently at 1100KB, add 300KB more
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 3,
            bytes: 200 * 1024, // 1000KB heap total
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 4,
            bytes: 100 * 1024, // 400KB staging total = 1400KB (~91%)
            layout_class: None,
        })
        .unwrap();

    let metrics = service.metrics();
    assert_eq!(metrics.pressure, PressureClass::Critical);
}

#[test]
fn memory_control_rejects_invalid_budget() {
    let config = MemoryConfig {
        heap_budget_bytes: 1024,
        staging_budget_bytes: 1024,
    };

    let mut service = MemoryControlService::new(config);

    let result = service.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 2048, // Exceeds budget
        layout_class: None,
    });

    assert!(result.is_err());
    match result.unwrap_err() {
        EngineCoreError::InvalidDescriptor(msg) => {
            assert!(msg.contains("exceeds"));
        }
        _ => panic!("Expected InvalidDescriptor error"),
    }
}

#[test]
fn memory_pressure_maps_to_runtime_degrade_rung() {
    let config = MemoryConfig {
        heap_budget_bytes: 1024 * 1024,
        staging_budget_bytes: 512 * 1024,
    };

    let mut service = MemoryControlService::new(config);

    // Reserve to critical pressure (90%+)
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 1024 * 1024, // 100% of heap
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 400 * 1024, // ~91% total
            layout_class: None,
        })
        .unwrap();

    let metrics = service.metrics();
    assert_eq!(metrics.pressure, PressureClass::Critical);

    // This pressure should map to runtime degrade decision
    let world = WorldState::new();
    let kernel = RuntimeKernel::new(
        world,
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 16,
            publish_passes: 1,
        },
    );

    let envelope = BudgetEnvelope {
        scenario_id: "test_scenario".to_string(),
        baseline_id: "baseline_v1".to_string(),
        compare_horizon_frames: 60,
    };

    // High RAM usage matching critical memory pressure
    let usage = DomainBudgetUsage {
        cpu_frame_ms_x100: 1000,
        gpu_frame_ms_x100: 1000,
        ram_resident_mib: 6000, // Critical RAM
        io_hitch_ms_x100: 200,
    };

    let decision = kernel.evaluate_budget(&envelope, &usage).unwrap();
    assert_eq!(decision.bucket, PressureBucket::Red);
    assert!(decision.rung.is_some());
}

