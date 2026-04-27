#![allow(unused_imports)]
use super::*;

#[test]
fn new_world_starts_at_zero_tick() {
    assert_eq!(WorldState::new().read_model().tick, Tick(0));
}
#[test]
fn snapshot_reflects_segment_count() {
    let w = WorldState::new();
    assert_eq!(w.snapshot(3).segment_count, 3);
}
#[test]
fn snapshot_bytes_are_non_empty() {
    let w = WorldState::new();
    assert!(!w.snapshot_bytes(0).unwrap().is_empty());
}
#[test]
fn apply_advances_tick_and_epoch() {
    let mut w = WorldState::new();
    w.apply(
        &[ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        }],
        1,
    )
    .unwrap();
    let r = w.read_model();
    assert_eq!(r.tick, Tick(1));
    assert_eq!(r.epoch, 1);
}
#[test]
fn apply_rejects_publish_pass_overflow() {
    let mut w = WorldState::new();
    assert!(w.apply(&[], MAX_PUBLISH_PASSES + 1).is_err());
}
