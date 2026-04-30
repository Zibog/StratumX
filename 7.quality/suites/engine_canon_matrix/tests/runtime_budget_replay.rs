mod common;
use common::*;

fn envelope() -> BudgetEnvelope {
    BudgetEnvelope {
        scenario_id: "scenario.combined.old_floor".to_string(),
        baseline_id: "baseline.old_floor.mixed".to_string(),
        compare_horizon_frames: 1800,
    }
}

#[test]
fn runtime_mode_and_budget_decision_are_operator_visible() {
    let realtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 4,
            publish_passes: 1,
        },
    );
    let headless = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 4,
            publish_passes: 1,
        },
    );

    assert_eq!(realtime.mode(), RuntimeMode::Realtime);
    assert_eq!(headless.mode(), RuntimeMode::Headless);

    let decision = realtime
        .evaluate_budget(
            &envelope(),
            &DomainBudgetUsage {
                cpu_frame_ms_x100: 1600,
                gpu_frame_ms_x100: 900,
                ram_resident_mib: 2048,
                io_hitch_ms_x100: 200,
            },
        )
        .unwrap();

    assert_eq!(decision.axis, PressureAxis::Cpu);
    assert_eq!(decision.bucket, PressureBucket::Red);
    assert_eq!(
        decision.rung.as_ref().map(|rung| rung.degrade_step),
        Some(DegradeStep::ReduceFarFieldFrequency)
    );
    assert!(decision
        .first_blocking_code
        .contains("pressure.cpu.frame_time"));
}

#[test]
fn runtime_replay_is_deterministic_for_same_input() {
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 4,
        publish_passes: 1,
    };
    let mut left = RuntimeKernel::new(WorldState::new(), config.clone());
    let mut right = RuntimeKernel::new(WorldState::new(), config);
    let input = ReplayInputFrame {
        tick: Tick(0),
        apply_segments: vec![ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        }],
        usage: DomainBudgetUsage {
            cpu_frame_ms_x100: 1200,
            gpu_frame_ms_x100: 1000,
            ram_resident_mib: 2048,
            io_hitch_ms_x100: 300,
        },
        envelope: envelope(),
    };

    let left_digest = left.replay_frame(input.clone()).unwrap();
    let right_digest = right.replay_frame(input).unwrap();

    assert_eq!(left_digest, right_digest);
    assert_eq!(
        left.last_degrade_decision().unwrap().bucket,
        PressureBucket::Green
    );
}

#[test]
fn runtime_degrade_decision_preserves_world_truth() {
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 4,
        publish_passes: 1,
    };
    let mut baseline = RuntimeKernel::new(WorldState::new(), config.clone());
    let mut degraded = RuntimeKernel::new(WorldState::new(), config);
    let segment = ApplySegment {
        region_key: (1, 0, 0),
        family_tags: vec![2],
    };
    baseline.enqueue_apply_segment(segment.clone()).unwrap();
    degraded.enqueue_apply_segment(segment).unwrap();

    baseline.run_tick().unwrap();
    let (_, decision) = degraded
        .run_tick_with_budget(
            envelope(),
            DomainBudgetUsage {
                cpu_frame_ms_x100: 1555,
                gpu_frame_ms_x100: 900,
                ram_resident_mib: 2048,
                io_hitch_ms_x100: 200,
            },
        )
        .unwrap();

    assert_eq!(decision.bucket, PressureBucket::Red);
    assert_eq!(
        baseline.world().deterministic_digest().unwrap(),
        degraded.world().deterministic_digest().unwrap()
    );
}

#[test]
fn runtime_replay_rejects_tick_mismatch_and_hard_fail_without_mutating_truth() {
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 4,
        publish_passes: 1,
    };
    let mut mismatch = RuntimeKernel::new(WorldState::new(), config.clone());
    assert!(mismatch
        .replay_frame(ReplayInputFrame {
            tick: Tick(1),
            apply_segments: vec![],
            usage: DomainBudgetUsage {
                cpu_frame_ms_x100: 1200,
                gpu_frame_ms_x100: 1000,
                ram_resident_mib: 2048,
                io_hitch_ms_x100: 300,
            },
            envelope: envelope(),
        })
        .is_err());

    let mut hard_fail = RuntimeKernel::new(WorldState::new(), config);
    let digest_before = hard_fail.world().deterministic_digest().unwrap();
    let result = hard_fail.run_tick_with_budget(
        envelope(),
        DomainBudgetUsage {
            cpu_frame_ms_x100: 2000,
            gpu_frame_ms_x100: 1000,
            ram_resident_mib: 2048,
            io_hitch_ms_x100: 300,
        },
    );

    assert!(result.is_err());
    assert_eq!(
        digest_before,
        hard_fail.world().deterministic_digest().unwrap()
    );
    assert_eq!(
        hard_fail.last_degrade_decision().unwrap().bucket,
        PressureBucket::HardFail
    );
}
