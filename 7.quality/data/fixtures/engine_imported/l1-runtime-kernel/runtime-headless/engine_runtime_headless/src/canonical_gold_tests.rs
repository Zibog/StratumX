#![allow(unused_imports)]
use super::*;

#[test]
fn headless_profile_uses_headless_runtime() {
    let p = HeadlessRuntimeProfile::new(
        engine_world::WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    assert_eq!(
        p.kernel().profile(),
        engine_runtime::RuntimeProfile::Headless20
    );
}
#[test]
fn headless_step_advances_tick() {
    let mut p = HeadlessRuntimeProfile::new(
        engine_world::WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    assert_eq!(p.step().unwrap().kernel_result.tick.0, 1);
}
#[test]
fn headless_step_emits_snapshot_when_enabled() {
    let mut p = HeadlessRuntimeProfile::new(
        engine_world::WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: true,
        },
    );
    assert!(p.step().unwrap().snapshot_bytes.is_some());
}
#[test]
fn kernel_mut_exposes_runtime_kernel() {
    let mut p = HeadlessRuntimeProfile::new(
        engine_world::WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    p.kernel_mut()
        .enqueue_apply_segment(engine_world::ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        })
        .unwrap();
    assert_eq!(p.kernel().diagnostics().apply_queue_depth, 1);
}
#[test]
fn headless_without_snapshot_leaves_none() {
    let mut p = HeadlessRuntimeProfile::new(
        engine_world::WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );
    assert!(p.step().unwrap().snapshot_bytes.is_none());
}
