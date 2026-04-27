use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_world::WorldState;

#[test]
fn headless_profile_never_presents_a_frame() {
    let mut profile = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    let result = profile.step().unwrap();
    assert!(!result.kernel_result.presented_frame);
}
