#[test]
fn invalid_budget_envelope_rejected() {
    let world = WorldState::new();
    let kernel = RuntimeKernel::new(
        world,
        engine_runtime::RuntimeConfig {
            profile: engine_runtime::RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 16,
            publish_passes: 1,
        },
    );

    // Empty scenario_id
    let envelope = BudgetEnvelope {
        scenario_id: "".to_string(),
        baseline_id: "baseline_v1".to_string(),
        compare_horizon_frames: 60,
    };

    let usage = DomainBudgetUsage {
        cpu_frame_ms_x100: 1000,
        gpu_frame_ms_x100: 1000,
        ram_resident_mib: 4000,
        io_hitch_ms_x100: 200,
    };

    let result = kernel.evaluate_budget(&envelope, &usage);
    assert!(result.is_err());

    // Zero compare_horizon_frames
    let envelope = BudgetEnvelope {
        scenario_id: "test".to_string(),
        baseline_id: "baseline_v1".to_string(),
        compare_horizon_frames: 0,
    };

    let result = kernel.evaluate_budget(&envelope, &usage);
    assert!(result.is_err());
}
