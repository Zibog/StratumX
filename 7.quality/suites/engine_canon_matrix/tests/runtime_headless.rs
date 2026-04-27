// Runtime Headless Profile Tests

use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_world::WorldState;

#[test]
fn test_headless_profile_step() {
    let mut profile = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 4,
            emit_snapshot_bytes: true,
        },
    );
    let result = profile.step().expect("headless step");
    assert_eq!(result.kernel_result.tick.0, 1);
    assert!(result.snapshot_bytes.is_some());
    assert!(result
        .snapshot_bytes
        .as_ref()
        .is_some_and(|bytes| !bytes.is_empty()));
}
