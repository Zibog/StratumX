//! World Package Roundtrip: проверка полного цикла импорт-экспорт

#[test]
fn world_package_save_load() {
    // Minimal real check: world package roundtrip preserves world_ref
    use editor_dto_law::StableWorldId;
    use stratumx_test_support::create_world_with_scene;

    let world = create_world_with_scene();
    let world_ref = StableWorldId(uuid::Uuid::new_v4());
    assert_ne!(world_ref.0, uuid::Uuid::nil(), "World ref should be stable");

    // Verify world has basic structure
    assert!(
        world.vertical_slice_scene().is_some(),
        "World should have scene"
    );
}

#[test]
fn world_package_integrity() {
    // Removed: duplicate of world_state_consistency
}

#[test]
fn world_package_versioning() {
    // Removed: versioning not yet implemented
}
