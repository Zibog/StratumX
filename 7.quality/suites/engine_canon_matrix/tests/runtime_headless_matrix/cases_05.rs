#[test]
fn runtime_headless_step_case_22() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_23() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_24() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
