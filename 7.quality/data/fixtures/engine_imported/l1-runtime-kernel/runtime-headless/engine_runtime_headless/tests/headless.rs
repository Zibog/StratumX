use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_world::WorldState;

#[test]
fn headless_profile_can_emit_snapshots() {
    let mut profile = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let result = profile.step().unwrap();
    assert!(result.snapshot_bytes.is_some());
}
