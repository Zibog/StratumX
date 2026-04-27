#![allow(unused_imports)]
use super::*;

#[test]
fn realtime_requires_non_zero_fps() {
    assert!(RealtimeRuntimeProfile::new(
        engine_world::WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 0,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true
        }
    )
    .is_err());
}
#[test]
fn realtime_step_presents_frame_when_enabled() {
    let mut p = RealtimeRuntimeProfile::new(
        engine_world::WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap();
    assert!(p.step().unwrap().frame_presented);
}
#[test]
fn realtime_step_without_queue_still_runs() {
    let mut p = RealtimeRuntimeProfile::new(
        engine_world::WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: false,
        },
    )
    .unwrap();
    assert!(!p.step().unwrap().frame_presented);
}
#[test]
fn realtime_kernel_accessor_exposes_interactive_profile() {
    let p = RealtimeRuntimeProfile::new(
        engine_world::WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: false,
        },
    )
    .unwrap();
    assert_eq!(
        p.kernel().profile(),
        engine_runtime::RuntimeProfile::Interactive60
    );
}
#[test]
fn realtime_kernel_mut_allows_apply_queueing() {
    let mut p = RealtimeRuntimeProfile::new(
        engine_world::WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 1,
            enqueue_presentable_frames: false,
        },
    )
    .unwrap();
    p.kernel_mut()
        .enqueue_apply_segment(engine_world::ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        })
        .unwrap();
    assert_eq!(p.kernel().diagnostics().apply_queue_depth, 1);
}
