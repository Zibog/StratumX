use engine_runtime::RuntimeProfile;
use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

#[test]
fn realtime_profile_requires_non_zero_target_fps() {
    let result = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 0,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    );
    assert!(result.is_err());
}

#[test]
fn realtime_profile_uses_interactive_kernel_profile() {
    let profile = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: false,
        },
    )
    .unwrap();
    assert_eq!(profile.kernel().profile(), RuntimeProfile::Interactive60);
}
