// Tests for world save/load roundtrip

use editor_dto_law::{StableWorldId, WorldPackageManifest};
use std::fs;
use stratumx_editor_l8_0_editor_shell::WorldLifecycleManager;
use tempfile::TempDir;
use uuid::Uuid;

#[test]
fn save_world_roundtrip_preserves_identity() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("roundtrip_test");

    // Create world
    let world_ref = StableWorldId(Uuid::new_v4());
    let mut lifecycle = WorldLifecycleManager::new();
    let result = lifecycle.open_startup_world_from_package();
    assert!(result.accepted);

    let world = lifecycle.get_world_state().unwrap();

    // Save world
    lifecycle
        .save_world_to_path(world, world_ref, &world_path)
        .unwrap();

    // Verify world.json exists
    assert!(world_path.join("world.json").exists());

    // Load manifest and verify ID
    let manifest_json = fs::read_to_string(world_path.join("world.json")).unwrap();
    let manifest: WorldPackageManifest = serde_json::from_str(&manifest_json).unwrap();
    assert_eq!(manifest.world_id, world_ref.0);

    // Load world back
    let mut lifecycle2 = WorldLifecycleManager::new();
    let result2 = lifecycle2.open_world_from_path(&world_path);

    assert!(result2.accepted);
    assert_eq!(result2.world_ref, Some(world_ref));
}
