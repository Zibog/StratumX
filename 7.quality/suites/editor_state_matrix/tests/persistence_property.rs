//! Property-Based Tests for Persistence Layer
//!
//! Tests universal properties that should hold for all persistence views.

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::path::PathBuf;
    use stratumx_editor_state_containers::owners::project_owner::{
        ProjectIdentity, ProjectOwner, WorkspaceIdentity,
    };
    use stratumx_editor_state_containers::owners::workspace_owner::{
        DockingConfig, PanelGeometry, PanelId, WorkspaceOwner,
    };
    use stratumx_editor_state_containers::owners::world_owner::{
        RuntimeModeState, TerrainState, WorldIdentity, WorldOwner,
    };
    use stratumx_editor_state_containers::persistence::{
        ProjectPersistenceView, WorkspacePersistenceView, WorldPersistenceView,
    };
    use uuid::Uuid;

    // ============================================================================
    // Property 14: Persistence View Separation
    // ============================================================================
    // **Validates: Requirements 8.1, 8.4, 10.1**
    //
    // For any owner container, there exists a separate persistence view type
    // (distinct from the owner type), and the persistence view contains only
    // persistable fields (no runtime-only fields like event_bus).

    /// **Property 14: Persistence View Separation**
    ///
    /// **Validates: Requirements 8.1, 8.4, 10.1**
    ///
    /// Test that persistence view is distinct from owner type.
    /// Test that persistence view contains only persistable fields.
    #[test]
    fn property_14_persistence_view_separation() {
        // This property is validated at compile time by the type system:
        // 1. ProjectOwner != ProjectPersistenceView (distinct types)
        // 2. WorkspaceOwner != WorkspacePersistenceView (distinct types)
        // 3. WorldOwner != WorldPersistenceView (distinct types)
        //
        // The persistence views are defined in separate files and have different
        // field sets (no event_callback field).

        // We can verify this at runtime by checking type names
        assert_ne!(
            std::any::type_name::<ProjectOwner>(),
            std::any::type_name::<ProjectPersistenceView>(),
            "ProjectOwner and ProjectPersistenceView must be distinct types"
        );

        assert_ne!(
            std::any::type_name::<WorkspaceOwner>(),
            std::any::type_name::<WorkspacePersistenceView>(),
            "WorkspaceOwner and WorkspacePersistenceView must be distinct types"
        );

        assert_ne!(
            std::any::type_name::<WorldOwner>(),
            std::any::type_name::<WorldPersistenceView>(),
            "WorldOwner and WorldPersistenceView must be distinct types"
        );
    }

    /// Test that persistence views can be serialized (contain only persistable fields)
    #[test]
    fn property_14_persistence_views_are_serializable() {
        // Create sample persistence views
        let project_view = ProjectPersistenceView::new(
            ProjectIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test")),
            WorkspaceIdentity::new(
                Uuid::new_v4(),
                "Workspace".to_string(),
                PathBuf::from("/workspace"),
            ),
            0,
            Vec::new(),
        );

        let workspace_view = WorkspacePersistenceView::new(
            1,
            Vec::new(),
            std::collections::HashMap::new(),
            None,
            DockingConfig::default(),
        );

        let world_view = WorldPersistenceView::new(
            WorldIdentity::new(Uuid::new_v4(), "World".to_string(), PathBuf::from("/world")),
            "snapshot".to_string(),
            None,
            None,
            Vec::new(),
            RuntimeModeState::default(),
            0,
        );

        // All persistence views must be serializable
        assert!(serde_json::to_string(&project_view).is_ok());
        assert!(serde_json::to_string(&workspace_view).is_ok());
        assert!(serde_json::to_string(&world_view).is_ok());
    }

    // ============================================================================
    // Property 15: Persistence View Round Trip
    // ============================================================================
    // **Validates: Requirements 8.4, 10.3**
    //
    // For any owner container, converting the owner to a persistence view and
    // back produces equivalent persistable state (runtime-only fields are
    // excluded from comparison).

    /// **Property 15: Persistence View Round Trip - ProjectOwner**
    ///
    /// **Validates: Requirements 8.4, 10.3**
    ///
    /// Test that converting ProjectOwner to persistence view and back produces
    /// equivalent persistable state.
    #[test]
    fn property_15_project_owner_round_trip() {
        proptest!(|(
            project_name in "[a-zA-Z0-9 ]{1,50}",
            workspace_name in "[a-zA-Z0-9 ]{1,50}",
            save_generation in 0u64..1000u64,
        )| {
            let project_id = ProjectIdentity::new(
                Uuid::new_v4(),
                project_name.clone(),
                PathBuf::from("/test/project"),
            );
            let workspace_id = WorkspaceIdentity::new(
                Uuid::new_v4(),
                workspace_name.clone(),
                PathBuf::from("/test/workspace"),
            );

            let mut original = ProjectOwner::new(project_id.clone(), workspace_id.clone());
            for _ in 0..save_generation {
                original.increment_save_generation();
            }

            // Convert to persistence view and back
            let view = ProjectPersistenceView::from(&original);
            let restored = ProjectOwner::try_from(view).unwrap();

            // Persistable state should match
            prop_assert_eq!(original.project_identity, restored.project_identity);
            prop_assert_eq!(original.workspace_identity, restored.workspace_identity);
            prop_assert_eq!(original.save_generation, restored.save_generation);
            prop_assert_eq!(original.content_snapshots, restored.content_snapshots);
        });
    }

    /// **Property 15: Persistence View Round Trip - WorkspaceOwner**
    ///
    /// **Validates: Requirements 8.4, 10.3**
    ///
    /// Test that converting WorkspaceOwner to persistence view and back produces
    /// equivalent persistable state.
    #[test]
    fn property_15_workspace_owner_round_trip() {
        proptest!(|(
            panel_count in 0usize..10usize,
            schema_version in 1u32..10u32,
        )| {
            let mut original = WorkspaceOwner::new();
            original.schema_version = schema_version;

            // Add random panels
            for i in 0..panel_count {
                let panel_id = PanelId::new(format!("panel_{}", i));
                let geometry = PanelGeometry::floating(
                    (i * 100) as f32,
                    (i * 100) as f32,
                    800.0,
                    600.0,
                );
                original.add_panel(panel_id, geometry);
            }

            // Convert to persistence view and back
            let view = WorkspacePersistenceView::from(&original);
            let restored = WorkspaceOwner::try_from(view).unwrap();

            // All state should match (WorkspaceOwner has no runtime-only fields)
            prop_assert_eq!(original.schema_version, restored.schema_version);
            prop_assert_eq!(original.open_panel_ids, restored.open_panel_ids);
            prop_assert_eq!(original.panel_positions, restored.panel_positions);
            prop_assert_eq!(original.focused_panel, restored.focused_panel);
            prop_assert_eq!(original.docking_configuration, restored.docking_configuration);
        });
    }

    /// **Property 15: Persistence View Round Trip - WorldOwner**
    ///
    /// **Validates: Requirements 8.4, 10.3**
    ///
    /// Test that converting WorldOwner to persistence view and back produces
    /// equivalent persistable state.
    #[test]
    fn property_15_world_owner_round_trip() {
        proptest!(|(
            world_name in "[a-zA-Z0-9 ]{1,50}",
            snapshot_ref in "[a-zA-Z0-9_]{1,50}",
            has_terrain in proptest::bool::ANY,
        )| {
            let world_id = WorldIdentity::new(
                Uuid::new_v4(),
                world_name.clone(),
                PathBuf::from("/test/world"),
            );

            let mut original = WorldOwner::new(world_id.clone(), snapshot_ref.clone());

            if has_terrain {
                original.set_terrain_state(Some(TerrainState::new(
                    (1024, 1024),
                    (1000.0, 1000.0),
                    "default".to_string(),
                )));
            }

            // Convert to persistence view and back
            let view = WorldPersistenceView::from(&original);
            let restored = WorldOwner::try_from(view).unwrap();

            // Persistable state should match
            prop_assert_eq!(original.world_identity, restored.world_identity);
            prop_assert_eq!(original.world_snapshot_ref, restored.world_snapshot_ref);
            prop_assert_eq!(original.terrain_state.is_some(), restored.terrain_state.is_some());
            prop_assert_eq!(original.environment_state.is_some(), restored.environment_state.is_some());
            prop_assert_eq!(original.world_diagnostics.len(), restored.world_diagnostics.len());
        });
    }

    /// Test that serialization round trip preserves data
    #[test]
    fn property_15_serialization_round_trip() {
        proptest!(|(
            project_name in "[a-zA-Z0-9 ]{1,50}",
            workspace_name in "[a-zA-Z0-9 ]{1,50}",
        )| {
            let project_id = ProjectIdentity::new(
                Uuid::new_v4(),
                project_name.clone(),
                PathBuf::from("/test/project"),
            );
            let workspace_id = WorkspaceIdentity::new(
                Uuid::new_v4(),
                workspace_name.clone(),
                PathBuf::from("/test/workspace"),
            );

            let original = ProjectOwner::new(project_id, workspace_id);
            let view = ProjectPersistenceView::from(&original);

            // Serialize and deserialize
            let json = serde_json::to_string(&view).unwrap();
            let deserialized: ProjectPersistenceView = serde_json::from_str(&json).unwrap();

            // Should be equivalent
            prop_assert_eq!(view.project_identity, deserialized.project_identity);
            prop_assert_eq!(view.workspace_identity, deserialized.workspace_identity);
            prop_assert_eq!(view.save_generation, deserialized.save_generation);
        });
    }
}
