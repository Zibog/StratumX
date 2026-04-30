// Runtime Realtime Profile Tests

use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

#[test]
fn realtime_profile_step_with_receipt_keeps_monotonic_frame_ids() {
    let mut profile = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: false,
        },
    )
    .expect("realtime profile");

    let first = profile.step_with_receipt().expect("first frame");
    let second = profile.step_with_receipt().expect("second frame");

    assert_eq!(first.frame_id.0, 1);
    assert_eq!(second.frame_id.0, 2);
    assert_eq!(first.tick.0, 1);
    assert_eq!(second.tick.0, 2);
    assert_eq!(first.frame_budget_micros, second.frame_budget_micros);
    assert!(!first.presented);
    assert!(!second.presented);
}
