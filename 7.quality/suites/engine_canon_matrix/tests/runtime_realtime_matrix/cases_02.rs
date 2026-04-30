#[test]
fn runtime_realtime_step_case_12() {
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
#[test]
fn runtime_realtime_step_case_13() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_14() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 90,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_15() {
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
#[test]
fn runtime_realtime_step_case_16() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_17() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 90,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_18() {
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
#[test]
fn runtime_realtime_step_case_19() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_20() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 90,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_21() {
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
#[test]
fn runtime_realtime_step_case_22() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
#[test]
fn runtime_realtime_step_case_23() {
    let mut p = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 90,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    let r = p.step().unwrap();
    assert!(r.frame_presented);
}
