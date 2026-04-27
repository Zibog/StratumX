//! Unit tests for directory structure validation
//!
//! This test suite validates that the expected directory structure exists
//! for editor-state-containers and app_state as specified in the design.
//!
//! Requirements validated:
//! - 2.1: editor-state-containers directory structure
//! - 2.2, 2.3, 2.4, 2.5, 2.6, 2.7: Expected files in each directory
//! - 4.1: app_state directory structure
//! - 4.2, 4.3, 4.4, 4.5: Expected files in app_state directories

use std::path::PathBuf;

/// Find the workspace root by walking up from current directory
fn get_workspace_root() -> PathBuf {
    let mut current = std::env::current_dir().expect("Failed to get current directory");

    // Walk up until we find the workspace root (contains 7.quality directory)
    loop {
        let quality_dir = current.join("7.quality");
        if quality_dir.exists() {
            return current;
        }

        if !current.pop() {
            panic!("Could not find workspace root");
        }
    }
}

/// Test that owners/ directory exists and contains expected files
#[test]
fn test_editor_state_containers_owners_directory_exists() {
    let root = get_workspace_root();
    let owners_dir = root.join("5.editor/editor-state-containers/src/owners");
    assert!(
        owners_dir.exists(),
        "owners/ directory should exist at {}",
        owners_dir.display()
    );
    assert!(
        owners_dir.is_dir(),
        "owners/ should be a directory, not a file"
    );
}

#[test]
fn test_editor_state_containers_owners_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src/owners");

    let expected_files = vec![
        "mod.rs",
        "diagnostics_owner.rs",
        "diagnostics_mutations.rs",
        "diagnostics_projections.rs",
        "diagnostics_validation.rs",
        "project_mutations.rs",
        "project_projections.rs",
        "project_validation.rs",
        "workspace_mutations.rs",
        "workspace_projections.rs",
        "workspace_validation.rs",
        "world_mutations.rs",
        "world_projections.rs",
        "world_validation.rs",
        "project_owner/mod.rs",
        "project_owner/identity.rs",
        "project_owner/mutation_surface.rs",
        "project_owner/refs.rs",
        "project_owner/settings.rs",
        "workspace_owner/mod.rs",
        "workspace_owner/active_surfaces.rs",
        "workspace_owner/layout_truth.rs",
        "workspace_owner/workspace_refs.rs",
        "world_owner/mod.rs",
        "world_owner/world_bindings.rs",
        "world_owner/world_diagnostics_link.rs",
        "world_owner/world_event_link.rs",
        "world_owner/world_identity.rs",
        "world_owner/world_runtime_posture.rs",
        "world_owner/world_snapshot_ref.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in owners/ directory",
            file
        );
    }
}

/// Test that queries/ directory exists and contains expected files
#[test]
fn test_editor_state_containers_queries_directory_exists() {
    let root = get_workspace_root();
    let queries_dir = root.join("5.editor/editor-state-containers/src/queries");
    assert!(
        queries_dir.exists(),
        "queries/ directory should exist at {}",
        queries_dir.display()
    );
    assert!(
        queries_dir.is_dir(),
        "queries/ should be a directory, not a file"
    );
}

#[test]
fn test_editor_state_containers_queries_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src/queries");

    let expected_files = vec![
        "mod.rs",
        "project_queries.rs",
        "workspace_queries.rs",
        "diagnostics_queries.rs",
        "world_queries/mod.rs",
        "world_queries/diagnostics_summary_view.rs",
        "world_queries/environment_summary_view.rs",
        "world_queries/identity_view.rs",
        "world_queries/material_coverage_view.rs",
        "world_queries/terrain_summary_view.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in queries/ directory",
            file
        );
    }
}

/// Test that cache/ directory exists and contains expected files
#[test]
fn test_editor_state_containers_cache_directory_exists() {
    let root = get_workspace_root();
    let cache_dir = root.join("5.editor/editor-state-containers/src/cache");
    assert!(
        cache_dir.exists(),
        "cache/ directory should exist at {}",
        cache_dir.display()
    );
    assert!(
        cache_dir.is_dir(),
        "cache/ should be a directory, not a file"
    );
}

#[test]
fn test_editor_state_containers_cache_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src/cache");

    let expected_files = vec![
        "mod.rs",
        "rebuildable_caches.rs",
        "cache_keys.rs",
        "cache_invalidation.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in cache/ directory",
            file
        );
    }
}

/// Test that persistence/ directory exists and contains expected files
#[test]
fn test_editor_state_containers_persistence_directory_exists() {
    let root = get_workspace_root();
    let persistence_dir = root.join("5.editor/editor-state-containers/src/persistence");
    assert!(
        persistence_dir.exists(),
        "persistence/ directory should exist at {}",
        persistence_dir.display()
    );
    assert!(
        persistence_dir.is_dir(),
        "persistence/ should be a directory, not a file"
    );
}

#[test]
fn test_editor_state_containers_persistence_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src/persistence");

    let expected_files = vec![
        "mod.rs",
        "project_persistence_view.rs",
        "workspace_persistence_view.rs",
        "world_persistence_view.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in persistence/ directory",
            file
        );
    }
}

/// Test that api/ directory exists
#[test]
fn test_editor_state_containers_api_directory_exists() {
    let root = get_workspace_root();
    let api_dir = root.join("5.editor/editor-state-containers/src/api");
    assert!(
        api_dir.exists(),
        "api/ directory should exist at {}",
        api_dir.display()
    );
    assert!(api_dir.is_dir(), "api/ should be a directory, not a file");
}

#[test]
fn test_editor_state_containers_api_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src/api");

    let expected_files = vec!["mod.rs"];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in api/ directory",
            file
        );
    }
}

/// Test that model/ directory exists and contains expected files
#[test]
fn test_editor_state_containers_model_directory_exists() {
    let root = get_workspace_root();
    let model_dir = root.join("5.editor/editor-state-containers/src/model");
    assert!(
        model_dir.exists(),
        "model/ directory should exist at {}",
        model_dir.display()
    );
    assert!(
        model_dir.is_dir(),
        "model/ should be a directory, not a file"
    );
}

#[test]
fn test_editor_state_containers_model_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src/model");

    let expected_files = vec![
        "mod.rs",
        "project_types.rs",
        "workspace_types.rs",
        "world_types.rs",
        "diagnostics_types.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in model/ directory",
            file
        );
    }
}

/// Test that all expected subdirectories exist in editor-state-containers
#[test]
fn test_editor_state_containers_all_subdirectories_exist() {
    let root = get_workspace_root();
    let base_path = root.join("5.editor/editor-state-containers/src");

    let expected_dirs = vec!["owners", "model", "queries", "cache", "persistence", "api"];

    for dir in expected_dirs {
        let dir_path = base_path.join(dir);
        assert!(
            dir_path.exists(),
            "Expected directory {} should exist in editor-state-containers/src/",
            dir
        );
        assert!(
            dir_path.is_dir(),
            "{} should be a directory, not a file",
            dir
        );
    }
}

// ============================================================================
// Desktop App Structure Tests
// ============================================================================

/// Test that desktop_app/ directory exists and contains expected files
#[test]
fn test_desktop_app_directory_exists() {
    let root = get_workspace_root();
    let runtime_dir = root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    assert!(
        runtime_dir.exists(),
        "desktop_app/ directory should exist at {}",
        runtime_dir.display()
    );
    assert!(
        runtime_dir.is_dir(),
        "desktop_app/ should be a directory, not a file"
    );
}

#[test]
fn test_desktop_app_root_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");

    let expected_files = vec![
        "mod.rs",
        "app_helpers.rs",
        "app_state.rs",
        "command_flush.rs",
        "editor_app.rs",
        "update_loop.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in desktop_app/ directory",
            file
        );
    }
}

/// Test that action_adapters/ directory exists and contains expected files
#[test]
fn test_desktop_app_action_adapters_directory_exists() {
    let root = get_workspace_root();
    let state_dir = root.join("6.apps/editor/stratumx_editor_app/src/desktop_app/action_adapters");
    assert!(
        state_dir.exists(),
        "action_adapters/ directory should exist at {}",
        state_dir.display()
    );
    assert!(
        state_dir.is_dir(),
        "action_adapters/ should be a directory, not a file"
    );
}

#[test]
fn test_desktop_app_action_adapters_files_exist() {
    let root = get_workspace_root();
    let base_path = root.join("6.apps/editor/stratumx_editor_app/src/desktop_app/action_adapters");

    let expected_files = vec![
        "mod.rs",
        "environment_actions.rs",
        "runtime_actions.rs",
        "world_actions.rs",
    ];

    for file in expected_files {
        let file_path = base_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file {} should exist in action_adapters/ directory",
            file
        );
    }
}

/// Test that all expected subdirectories exist in desktop_app
#[test]
fn test_desktop_app_all_subdirectories_exist() {
    let root = get_workspace_root();
    let base_path = root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");

    let expected_dirs = vec!["action_adapters"];

    for dir in expected_dirs {
        let dir_path = base_path.join(dir);
        assert!(
            dir_path.exists(),
            "Expected directory {} should exist in desktop_app/",
            dir
        );
        assert!(
            dir_path.is_dir(),
            "{} should be a directory, not a file",
            dir
        );
    }
}

/// Comprehensive test that validates the entire directory structure at once
#[test]
fn test_complete_directory_structure() {
    let root = get_workspace_root();

    // Editor state containers structure
    let editor_state_dirs = vec![
        "5.editor/editor-state-containers/src/owners",
        "5.editor/editor-state-containers/src/model",
        "5.editor/editor-state-containers/src/queries",
        "5.editor/editor-state-containers/src/cache",
        "5.editor/editor-state-containers/src/persistence",
        "5.editor/editor-state-containers/src/api",
    ];

    for dir in editor_state_dirs {
        let path = root.join(dir);
        assert!(
            path.exists() && path.is_dir(),
            "Directory {} should exist",
            dir
        );
    }

    // Desktop app structure
    let app_state_dirs = vec![
        "6.apps/editor/stratumx_editor_app/src/desktop_app",
        "6.apps/editor/stratumx_editor_app/src/desktop_app/action_adapters",
    ];

    for dir in app_state_dirs {
        let path = root.join(dir);
        assert!(
            path.exists() && path.is_dir(),
            "Directory {} should exist",
            dir
        );
    }
}
