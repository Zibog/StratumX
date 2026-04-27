//! World Truth: проверка системы world truth

use engine_world::{ApplySegment, WorldSnapshot};
use stratumx_test_support::{create_test_world, create_world_with_scene};

#[test]
fn world_state_consistency() {
    let world = create_world_with_scene();
    let scene = world
        .vertical_slice_scene()
        .expect("fixture should provide a vertical slice scene");
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
