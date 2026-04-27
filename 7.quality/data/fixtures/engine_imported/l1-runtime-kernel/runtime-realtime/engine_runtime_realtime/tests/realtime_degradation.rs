use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

#[test]
fn realtime_profile_can_run_without_auto_enqueued_frames() {
    let mut profile = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: false,
        },
    )
    .unwrap();
    let result = profile.step().unwrap();
    assert!(!result.frame_presented);
}
