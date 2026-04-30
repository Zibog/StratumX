#[test]
fn runtime_realtime_step_case_24() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 30,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
