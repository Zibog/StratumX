use engine_runtime::RuntimeProfile;
use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_world::WorldState;

#[test]
fn headless_profile_can_disable_snapshot_emission() {
    let mut profile = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    let result = profile.step().unwrap();
    assert!(result.snapshot_bytes.is_none());
}

#[test]
fn headless_profile_exposes_headless_kernel_profile() {
    let profile = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    assert_eq!(profile.kernel().profile(), RuntimeProfile::Headless20);
}
