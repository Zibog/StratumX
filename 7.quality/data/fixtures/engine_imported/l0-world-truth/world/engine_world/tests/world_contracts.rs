use engine_world::{ApplySegment, WorldState, MAX_PUBLISH_PASSES};

#[test]
fn valid_apply_advances_tick_and_epoch() {
    let mut world = WorldState::new();
    let before = world.read_model();
    world
        .apply(
            &[ApplySegment {
                region_key: (1, 2, 3),
                family_tags: vec![1],
            }],
            1,
        )
        .unwrap();
    let after = world.read_model();
    assert_eq!(after.tick.0, before.tick.0 + 1);
    assert_eq!(after.epoch, before.epoch + 1);
}

#[test]
fn publish_passes_over_ceiling_are_rejected() {
    let mut world = WorldState::new();
    let segments = [ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    }];
    assert!(world.apply(&segments, MAX_PUBLISH_PASSES + 1).is_err());
}

#[test]
fn snapshot_bytes_are_stable_for_same_state() {
    let world = WorldState::new();
    let a = world.snapshot_bytes(1).unwrap();
    let b = world.snapshot_bytes(1).unwrap();
    assert_eq!(a, b);
}
