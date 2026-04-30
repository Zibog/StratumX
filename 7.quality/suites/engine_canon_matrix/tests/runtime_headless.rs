// Runtime Headless Profile Tests

use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_world::WorldState;

#[test]
fn headless_profile_step_mutates_state_and_changes_digest() {
    let mut profile = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 4,
            emit_snapshot_bytes: true,
        },
    );

    let before = profile.replay_digest().expect("digest before step");
    let result = profile.step().expect("headless step");
    let after = profile.replay_digest().expect("digest after step");

    assert_eq!(result.kernel_result.tick.0, 1);
    assert!(result.snapshot_bytes.is_some());
    assert!(result
        .snapshot_bytes
        .as_ref()
        .is_some_and(|bytes| !bytes.is_empty()));
    assert_ne!(before, after);
}
