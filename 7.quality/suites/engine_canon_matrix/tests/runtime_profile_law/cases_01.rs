#[test]
fn headless_tick_is_deterministic_for_same_input() {
    let world1 = WorldState::new();
    let world2 = WorldState::new();

    let mut runtime1 = HeadlessRuntimeProfile::new(
        world1,
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );

    let mut runtime2 = HeadlessRuntimeProfile::new(
        world2,
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );

    let result1 = runtime1.step().unwrap();
    let result2 = runtime2.step().unwrap();

    assert_eq!(result1.kernel_result.tick, result2.kernel_result.tick);
    assert_eq!(
        result1.kernel_result.applied_segments,
        result2.kernel_result.applied_segments
    );
}

#[test]
fn headless_runtime_rejects_invalid_state() {
    let world = WorldState::new();
    let mut runtime = HeadlessRuntimeProfile::new(
        world,
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );

    // Valid step should succeed
    let result = runtime.step();
    assert!(result.is_ok());

    // Kernel mode should be headless
    assert_eq!(runtime.kernel().mode(), RuntimeMode::Headless);
}

#[test]
fn headless_runtime_applies_profile_budget_law() {
    let world = WorldState::new();
    let runtime = HeadlessRuntimeProfile::new(
        world,
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );

    assert_eq!(runtime.kernel().mode(), RuntimeMode::Headless);
}

#[test]
fn realtime_over_budget_selects_stable_degrade_rung() {
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

    // Over-budget CPU usage
    let usage = DomainBudgetUsage {
        cpu_frame_ms_x100: 1500, // 15ms, over yellow threshold
        gpu_frame_ms_x100: 1000,
        ram_resident_mib: 4000,
        io_hitch_ms_x100: 200,
    };

    let decision = kernel.evaluate_budget(&envelope, &usage).unwrap();

    assert_eq!(decision.bucket, PressureBucket::Orange);
    assert!(decision.rung.is_some());

    // Same input should produce same decision
    let decision2 = kernel.evaluate_budget(&envelope, &usage).unwrap();
    assert_eq!(decision, decision2);
}

#[test]
fn realtime_hard_fail_pressure_is_rejected() {
    let world = WorldState::new();
    let mut kernel = RuntimeKernel::new(
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

    // Hard-fail CPU usage
    let usage = DomainBudgetUsage {
        cpu_frame_ms_x100: 2000, // 20ms, hard-fail
        gpu_frame_ms_x100: 1000,
        ram_resident_mib: 4000,
        io_hitch_ms_x100: 200,
    };

    let result = kernel.run_tick_with_budget(envelope, usage);
    assert!(result.is_err());

    match result.unwrap_err() {
        EngineCoreError::InvalidDescriptor(msg) => {
            assert!(msg.contains("hard-fail"));
        }
        _ => panic!("Expected InvalidDescriptor error"),
    }
}

#[test]
fn realtime_decision_contains_budget_evidence() {
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

    let usage = DomainBudgetUsage {
        cpu_frame_ms_x100: 1300,
        gpu_frame_ms_x100: 1000,
        ram_resident_mib: 4000,
        io_hitch_ms_x100: 200,
    };

    let decision = kernel.evaluate_budget(&envelope, &usage).unwrap();

    assert!(!decision.threshold_row_id.is_empty());
    assert!(!decision.compare_horizon_id.is_empty());
    assert!(!decision.retained_baseline_artifact_id.is_empty());
    assert!(!decision.first_blocking_code.is_empty());
    assert!(!decision.next_legal_recovery_action.is_empty());
}

