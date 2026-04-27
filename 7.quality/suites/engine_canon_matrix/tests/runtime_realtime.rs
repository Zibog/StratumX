// Runtime Realtime Profile Tests

use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

#[test]
fn test_realtime_profile_step() {
    let mut profile = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .expect("realtime profile");
    let result = profile.step().expect("realtime step");
    assert!(result.frame_presented);
}
