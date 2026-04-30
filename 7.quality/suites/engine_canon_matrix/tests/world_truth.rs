//! World Truth: проверка системы world truth

use engine_world::{ApplySegment, WorldSnapshot};
use stratumx_test_support::{create_test_world, create_world_with_scene};

#[test]
fn world_state_consistency() {
    let world = create_world_with_scene();
    let scene = world
        .proof_region_scene()
        .expect("fixture should provide a proof region scene");
    let expected_samples = (scene.terrain.resolution[0] * scene.terrain.resolution[1]) as usize;

    assert_eq!(scene.terrain.height_samples.len(), expected_samples);
    assert_eq!(scene.wall.stack_id.0, 1);
    assert_eq!(scene.weapon.entity_id.0, 3);
}

#[test]
fn world_state_persistence() {
    let mut world = create_world_with_scene();
    world
        .apply(
            &[ApplySegment {
                region_key: (0, 0, 0),
                family_tags: vec![1, 2],
            }],
            1,
        )
        .unwrap();

    let snapshot = world.snapshot(1);
    let bytes = world.snapshot_bytes(1).unwrap();
    let restored: WorldSnapshot = bincode::deserialize(&bytes).unwrap();

    assert_eq!(restored, snapshot);
}

#[test]
fn world_state_replication() {
    let mut world = create_test_world();
    let before = world.read_model();

    world
        .apply(
            &[ApplySegment {
                region_key: (4, 8, 12),
                family_tags: vec![7],
            }],
            1,
        )
        .unwrap();

    let after = world.read_model();
    assert_eq!(after.tick.0, before.tick.0 + 1);
    assert_eq!(after.epoch, before.epoch + 1);
}

#[test]
fn world_apply_canonicalizes_segments_and_preserves_digest() {
    let segments_a = [
        ApplySegment {
            region_key: (1, 0, 0),
            family_tags: vec![4, 2, 4],
        },
        ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![3],
        },
    ];
    let segments_b = [
        ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![3],
        },
        ApplySegment {
            region_key: (1, 0, 0),
            family_tags: vec![2, 4],
        },
    ];

    let mut left = create_test_world();
    let mut right = create_test_world();

    left.apply(&segments_a, 1).unwrap();
    right.apply(&segments_b, 1).unwrap();

    assert_eq!(left.last_apply_journal(), right.last_apply_journal());
    assert_eq!(left.causal_summary(), right.causal_summary());
    assert_eq!(
        left.deterministic_digest().unwrap(),
        right.deterministic_digest().unwrap()
    );
    assert_eq!(left.last_apply_journal().segment_count, 2);
    assert_eq!(left.causal_summary().family_tags, vec![2, 3, 4]);
}

#[test]
fn world_apply_rejects_empty_family_tags_and_digest_changes_on_input_change() {
    let mut invalid_world = create_test_world();
    assert!(invalid_world
        .apply(
            &[ApplySegment {
                region_key: (0, 0, 0),
                family_tags: vec![],
            }],
            1,
        )
        .is_err());

    let mut left = create_test_world();
    let mut right = create_test_world();
    left.apply(
        &[ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        }],
        1,
    )
    .unwrap();
    right
        .apply(
            &[ApplySegment {
                region_key: (0, 0, 0),
                family_tags: vec![2],
            }],
            1,
        )
        .unwrap();

    assert_ne!(
        left.deterministic_digest().unwrap(),
        right.deterministic_digest().unwrap()
    );
}
