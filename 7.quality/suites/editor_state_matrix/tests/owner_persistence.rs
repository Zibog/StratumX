//! Tests for owner to_persistence_view methods
//!
//! Verifies that all owner containers can convert to persistence views correctly.

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use stratumx_editor_state_containers::owners::*;
    use stratumx_editor_state_containers::persistence::*;
    use uuid::Uuid;

    #[test]
    fn test_project_owner_to_persistence_view() {
        let project_id = project_owner::ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = project_owner::WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let owner = ProjectOwner::new(project_id.clone(), workspace_id.clone());
        let view = owner.to_persistence_view();

        assert_eq!(view.project_identity, project_id);
        assert_eq!(view.workspace_identity, workspace_id);
        assert_eq!(view.save_generation, 0);
        assert!(view.content_snapshots.is_empty());
    }

    #[test]
    fn test_workspace_owner_to_persistence_view() {
        let mut owner = WorkspaceOwner::new();
        let panel_id = workspace_owner::PanelId::new("viewport");
        let geometry = workspace_owner::PanelGeometry::floating(0.0, 0.0, 800.0, 600.0);

        owner.add_panel(panel_id.clone(), geometry.clone());
        owner.set_focused_panel(Some(panel_id.clone()));

        let view = owner.to_persistence_view();

        assert_eq!(view.schema_version, WorkspaceOwner::CURRENT_SCHEMA_VERSION);
        assert_eq!(view.open_panel_ids.len(), 1);
        assert_eq!(view.focused_panel, Some(panel_id));
    }

    #[test]
    fn test_world_owner_to_persistence_view() {
        let world_id = world_owner::WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let mut owner = WorldOwner::new(world_id.clone(), "snapshot_123".to_string());

        let terrain =
            world_owner::TerrainState::new((1024, 1024), (1000.0, 1000.0), "default".to_string());
        owner.set_terrain_state(Some(terrain));

        let view = owner.to_persistence_view();

        assert_eq!(view.world_identity, world_id);
        assert_eq!(view.world_snapshot_ref, "snapshot_123");
        assert!(view.terrain_state.is_some());
        assert!(view.environment_state.is_none());
    }

    #[test]
    fn test_persistence_view_excludes_runtime_fields() {
        // ProjectOwner has event_callback which should not be in persistence view
        let project_id = project_owner::ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = project_owner::WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let mut owner = ProjectOwner::new(project_id, workspace_id);

        // Set event callback (runtime-only field)
        owner.set_event_callback(Box::new(|_| {}));

        // Convert to persistence view
        let view = owner.to_persistence_view();

        // Verify we can serialize the view (would fail if runtime fields were included)
        let json = serde_json::to_string(&view).unwrap();
        assert!(!json.is_empty());

        // Verify we can deserialize it back
        let _deserialized: ProjectPersistenceView = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_all_owners_have_to_persistence_view() {
        // This test ensures all owner types implement to_persistence_view

        let project_id = project_owner::ProjectIdentity::new(
            Uuid::new_v4(),
            "Test".to_string(),
            PathBuf::from("/test"),
        );
        let workspace_id = project_owner::WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test".to_string(),
            PathBuf::from("/test"),
        );
        let world_id = world_owner::WorldIdentity::new(
            Uuid::new_v4(),
            "Test".to_string(),
            PathBuf::from("/test"),
        );

        let project_owner = ProjectOwner::new(project_id, workspace_id);
        let workspace_owner = WorkspaceOwner::new();
        let world_owner = WorldOwner::new(world_id, "snapshot".to_string());

        // All should have to_persistence_view method
        let _project_view = project_owner.to_persistence_view();
        let _workspace_view = workspace_owner.to_persistence_view();
        let _world_view = world_owner.to_persistence_view();
    }
}
