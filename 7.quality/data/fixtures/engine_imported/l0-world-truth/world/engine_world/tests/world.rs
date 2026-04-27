use engine_world::{
    ApplySegment, WorldState, MAX_FAMILY_FANOUT_PER_SEGMENT, MAX_PUBLISH_PASSES,
    MAX_SEGMENTS_PER_TICK,
};

#[test]
fn apply_enforces_segment_and_publish_ceilings() {
    let mut world = WorldState::new();

    let too_many_segments: Vec<_> = (0..=MAX_SEGMENTS_PER_TICK)
        .map(|_| ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![],
        })
        .collect();
    assert!(world.apply(&too_many_segments, 1).is_err());

    let mut over_fanout = vec![1u16; MAX_FAMILY_FANOUT_PER_SEGMENT + 1];
    let one_segment = vec![ApplySegment {
        region_key: (0, 0, 0),
        family_tags: std::mem::take(&mut over_fanout),
    }];
    assert!(world.apply(&one_segment, 1).is_err());

    let valid = vec![ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    }];
    assert!(world.apply(&valid, MAX_PUBLISH_PASSES).is_ok());
}

#[test]
fn snapshots_are_immutable_read_products() {
    let world = WorldState::new();
    let a = world.snapshot(2);
    let b = world.snapshot(2);
    assert_eq!(a, b);

    let bytes = world.snapshot_bytes(2).unwrap();
    assert!(!bytes.is_empty());
}
