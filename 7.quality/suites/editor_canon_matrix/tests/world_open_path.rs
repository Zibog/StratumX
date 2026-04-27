// Tests for path-based world opening

use editor_dto_law::{StableWorldId, WorldPackageManifest};
use std::fs;
use stratumx_editor_l8_0_editor_shell::WorldLifecycleManager;
use tempfile::TempDir;
use uuid::Uuid;

#[test]
fn world_open_from_path_loads_world_json() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("test_world");

    fs::create_dir_all(&world_path).unwrap();
    let manifest = WorldPackageManifest {
        world_id: Uuid::new_v4(),
        world_label: "Test World".to_string(),
        world_role: editor_dto_law::WorldRole::Content,
        version: "1.0.0".to_string(),
        terrain_root_ref: None,
        environment_root_ref: None,
        streaming_profile_ref: None,
        source_lineage: editor_dto_law::SourceLineage {
            created_at: "2026-04-06T12:00:00Z".to_string(),
            created_by: "test".to_string(),
            import_source: None,
            last_modified: "2026-04-06T12:00:00Z".to_string(),
        },
    };

    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    fs::write(world_path.join("world.json"), manifest_json).unwrap();

    let mut lifecycle = WorldLifecycleManager::new();
    let result = lifecycle.open_world_from_path(&world_path);

    assert!(result.accepted);
    assert_eq!(result.world_label, Some("Test World".to_string()));
}

#[test]
fn test_world_identity_preserved() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("identity_test");

    fs::create_dir_all(&world_path).unwrap();
    let original_id = Uuid::new_v4();
    let manifest = WorldPackageManifest {
        world_id: original_id,
        world_label: "Identity Test".to_string(),
        world_role: editor_dto_law::WorldRole::Content,
        version: "1.0.0".to_string(),
        terrain_root_ref: None,
        environment_root_ref: None,
        streaming_profile_ref: None,
        source_lineage: editor_dto_law::SourceLineage {
            created_at: "2026-04-06T12:00:00Z".to_string(),
            created_by: "test".to_string(),
            import_source: None,
            last_modified: "2026-04-06T12:00:00Z".to_string(),
        },
    };

    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    fs::write(world_path.join("world.json"), manifest_json).unwrap();

    let mut lifecycle = WorldLifecycleManager::new();
    let result = lifecycle.open_world_from_path(&world_path);

    assert!(result.accepted);
    assert_eq!(result.world_ref, Some(StableWorldId(original_id)));
}

#[test]
fn test_clean_world_not_demo_based() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("clean_test");

    fs::create_dir_all(&world_path).unwrap();
    let manifest = WorldPackageManifest {
        world_id: Uuid::new_v4(),
        world_label: "Clean World".to_string(),
        world_role: editor_dto_law::WorldRole::Content,
        version: "1.0.0".to_string(),
        terrain_root_ref: None,
        environment_root_ref: None,
        streaming_profile_ref: None,
        source_lineage: editor_dto_law::SourceLineage {
            created_at: "2026-04-06T12:00:00Z".to_string(),
            created_by: "test".to_string(),
            import_source: None,
            last_modified: "2026-04-06T12:00:00Z".to_string(),
        },
    };

    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    fs::write(world_path.join("world.json"), manifest_json).unwrap();

    let mut lifecycle = WorldLifecycleManager::new();
    let result = lifecycle.open_world_from_path(&world_path);

    assert!(result.accepted);

    let scene = lifecycle
        .get_world_state()
        .unwrap()
        .vertical_slice_scene()
        .unwrap();
    assert_eq!(scene.scene_name, "Clean World");
    assert_eq!(scene.terrain.resolution, [256, 256]);
}

#[test]
fn test_missing_manifest_fails() {
    let temp_dir = TempDir::new().unwrap();
    let world_path = temp_dir.path().join("missing_manifest");

    let mut lifecycle = WorldLifecycleManager::new();
    let result = lifecycle.open_world_from_path(&world_path);

    assert!(!result.accepted);
    assert!(result.failure_class.is_some());
}
