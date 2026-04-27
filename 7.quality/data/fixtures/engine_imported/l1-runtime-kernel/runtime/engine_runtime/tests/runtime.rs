use engine_runtime::{PresentableFrame, RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_world::{ApplySegment, WorldState};

#[test]
fn runtime_advances_world_tick_through_apply_boundary() {
    let world = WorldState::new();
    let mut runtime = RuntimeKernel::new(
        world,
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 8,
            publish_passes: 1,
        },
    );
    runtime
        .enqueue_apply_segment(ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        })
        .unwrap();
    runtime
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1,
        })
        .unwrap();
    let result = runtime.run_tick().unwrap();
    assert_eq!(result.applied_segments, 1);
    assert_eq!(result.tick.0, 1);
}
