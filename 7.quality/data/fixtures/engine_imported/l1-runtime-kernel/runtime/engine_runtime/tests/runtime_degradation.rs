use engine_runtime::{PresentableFrame, RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::{ApplySegment, WorldState};

#[test]
fn runtime_respects_apply_budget_across_multiple_ticks() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    for region in [(0, 0, 0), (1, 0, 0), (2, 0, 0)] {
        runtime
            .enqueue_apply_segment(ApplySegment {
                region_key: region,
                family_tags: vec![1],
            })
            .unwrap();
    }
    runtime
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1,
        })
        .unwrap();
    let first = runtime.run_tick().unwrap();
    assert_eq!(first.applied_segments, 1);
    assert_eq!(runtime.diagnostics().apply_queue_depth, 2);
}

#[test]
fn runtime_enforces_single_presentable_frame_inflight() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    runtime
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1,
        })
        .unwrap();
    assert!(runtime
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 2,
            visibility_freshness_frames: 1,
        })
        .is_err());
}
