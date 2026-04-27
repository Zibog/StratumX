use engine_core::Tick;
use engine_world::{
    ApplySegment, WorldSnapshot, WorldState, MAX_FAMILY_FANOUT_PER_SEGMENT, MAX_PUBLISH_PASSES,
    MAX_SEGMENTS_PER_TICK,
};

// === WorldState Lifecycle Tests ===

#[test]
fn test_world_state_new_is_valid() {
    let world = WorldState::new();
    assert_eq!(world.current_tick(), Tick(0));
    assert!(world.vertical_slice_scene().is_none());
    assert!(world.material_bindings().is_empty());
    assert!(world.damage_memory().is_empty());
    assert!(world.shot_log().is_empty());
    assert!(world.impact_log().is_empty());
    assert!(world.runtime_events().is_empty());
}

#[test]
fn test_world_state_default_equals_new() {
    let w1 = WorldState::new();
    let w2 = WorldState::default();
    assert_eq!(w1.current_tick(), w2.current_tick());
}

// === Vertical Slice Scene Tests ===

#[test]
fn test_vertical_slice_scene_mut_returns_none_when_not_set() {
    let mut world = WorldState::new();
    assert!(world.vertical_slice_scene_mut().is_none());
}

// === Tick and Apply Tests ===

#[test]
fn test_apply_advances_tick() {
    let mut world = WorldState::new();
    assert_eq!(world.current_tick(), Tick(0));

    world.apply(&[], 0).unwrap();
    assert_eq!(world.current_tick(), Tick(1));

    world.apply(&[], 0).unwrap();
    assert_eq!(world.current_tick(), Tick(2));
}

#[test]
fn test_apply_increments_epoch() {
    let mut world = WorldState::new();
    let snapshot_before = world.snapshot(0);
    assert_eq!(snapshot_before.epoch, 0);

    world.apply(&[], 0).unwrap();
    let snapshot_after = world.snapshot(0);
    assert_eq!(snapshot_after.epoch, 1);
}

#[test]
fn test_apply_empty_segments_is_ok() {
    let mut world = WorldState::new();
    let result = world.apply(&[], 0);
    assert!(result.is_ok());
}

#[test]
fn test_apply_valid_segment_is_ok() {
    let mut world = WorldState::new();
    let segment = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2, 3],
    };
    let result = world.apply(&[segment], 1);
    assert!(result.is_ok());
}

// === Apply Limit Tests ===

#[test]
fn test_apply_rejects_too_many_segments() {
    let mut world = WorldState::new();
    let segments: Vec<_> = (0..=MAX_SEGMENTS_PER_TICK)
        .map(|_| ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![],
        })
        .collect();

    let result = world.apply(&segments, 0);
    assert!(result.is_err());
}

#[test]
fn test_apply_accepts_max_segments() {
    let mut world = WorldState::new();
    let segments: Vec<_> = (0..MAX_SEGMENTS_PER_TICK)
        .map(|_| ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![],
        })
        .collect();

    let result = world.apply(&segments, 0);
    assert!(result.is_ok());
}

#[test]
fn test_apply_rejects_too_many_publish_passes() {
    let mut world = WorldState::new();
    let result = world.apply(&[], MAX_PUBLISH_PASSES + 1);
    assert!(result.is_err());
}

#[test]
fn test_apply_accepts_max_publish_passes() {
    let mut world = WorldState::new();
    let result = world.apply(&[], MAX_PUBLISH_PASSES);
    assert!(result.is_ok());
}

#[test]
fn test_apply_rejects_too_many_family_tags() {
    let mut world = WorldState::new();
    let segment = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: (0..=MAX_FAMILY_FANOUT_PER_SEGMENT).map(|i| i as u16).collect(),
    };
    let result = world.apply(&[segment], 0);
    assert!(result.is_err());
}

#[test]
fn test_apply_accepts_max_family_tags() {
    let mut world = WorldState::new();
    let segment = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: (0..MAX_FAMILY_FANOUT_PER_SEGMENT).map(|i| i as u16).collect(),
    };
    let result = world.apply(&[segment], 0);
    assert!(result.is_ok());
}

// === Snapshot Tests ===

#[test]
fn test_snapshot_captures_tick_and_epoch() {
    let mut world = WorldState::new();
    world.apply(&[], 0).unwrap();

    let snapshot = world.snapshot(5);
    assert_eq!(snapshot.tick, Tick(1));
    assert_eq!(snapshot.epoch, 1);
    assert_eq!(snapshot.segment_count, 5);
}

#[test]
fn test_snapshot_bytes_roundtrip() {
    let mut world = WorldState::new();
    world.apply(&[], 0).unwrap();

    let bytes = world.snapshot_bytes(3).unwrap();
    assert!(!bytes.is_empty());

    let snapshot: WorldSnapshot = bincode::deserialize(&bytes).unwrap();
    assert_eq!(snapshot.tick, Tick(1));
    assert_eq!(snapshot.epoch, 1);
    assert_eq!(snapshot.segment_count, 3);
}

// === ReadModel Tests ===

#[test]
fn test_read_model_reflects_world_state() {
    let mut world = WorldState::new();
    let model = world.read_model();
    assert_eq!(model.tick, Tick(0));
    assert_eq!(model.epoch, 0);

    world.apply(&[], 0).unwrap();
    let model = world.read_model();
    assert_eq!(model.tick, Tick(1));
    assert_eq!(model.epoch, 1);
}

// === Constants Tests ===

#[test]
fn test_constants_are_positive() {
    assert!(MAX_SEGMENTS_PER_TICK > 0);
    assert!(MAX_FAMILY_FANOUT_PER_SEGMENT > 0);
    assert!(MAX_PUBLISH_PASSES > 0);
}

#[test]
fn test_constants_have_reasonable_values() {
    assert_eq!(MAX_SEGMENTS_PER_TICK, 256);
    assert_eq!(MAX_FAMILY_FANOUT_PER_SEGMENT, 8);
    assert_eq!(MAX_PUBLISH_PASSES, 2);
}

// === ApplySegment Tests ===

#[test]
fn test_apply_segment_creation() {
    let segment = ApplySegment {
        region_key: (1, 2, 3),
        family_tags: vec![10, 20, 30],
    };
    assert_eq!(segment.region_key, (1, 2, 3));
    assert_eq!(segment.family_tags.len(), 3);
}

#[test]
fn test_apply_segment_equality() {
    let a = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    let b = ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1, 2],
    };
    let c = ApplySegment {
        region_key: (1, 0, 0),
        family_tags: vec![1, 2],
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// === WorldSnapshot Serialization Tests ===

#[test]
fn test_world_snapshot_bincode_roundtrip() {
    let snapshot = WorldSnapshot {
        tick: Tick(42),
        epoch: 100,
        segment_count: 5,
    };
    let bytes = bincode::serialize(&snapshot).unwrap();
    let loaded: WorldSnapshot = bincode::deserialize(&bytes).unwrap();
    assert_eq!(snapshot, loaded);
}
