//! Property-Based Tests for Workspace Layout Persistence
//!
//! **Validates: Requirements 8.5, 8.6**
//!
//! This module contains property-based tests that verify workspace layout
//! serialization and deserialization maintains data integrity.

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::collections::HashMap;
    use stratumx_editor_state_containers::{
        DockPosition, DockingConfig, PanelGeometry, PanelId, WorkspaceOwner, WorkspaceState,
    };
    use tempfile::NamedTempFile;

    // ========================================================================
    // Property 10: Workspace Layout Persistence Round Trip
    // ========================================================================
    //
    // **Validates: Requirements 8.5, 8.6**
    //
    // Property 10: Workspace Layout Persistence Round Trip
    //
    // *For any* workspace layout, serializing the layout to a file and then
    // deserializing it must produce an equivalent layout with the same open
    // panels, positions, and docking configuration.
    //
    // This test validates that:
    // - All open panel IDs are preserved
    // - All panel positions and geometries are preserved
    // - Focused panel is preserved
    // - Docking configuration is preserved
    // - Schema version is preserved
    // - No data loss during serialization/deserialization
    //
    // The test generates random workspace layouts with varying numbers of
    // panels, positions, and configurations, then verifies that the round
    // trip through file serialization maintains perfect equivalence.

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn property_workspace_layout_round_trip(
            layout in workspace_state_strategy()
        ) {
            // Create a temporary file for serialization
            let temp_file = NamedTempFile::new()
                .expect("Failed to create temporary file");
            let path = temp_file.path();

            // Serialize the workspace layout to file
            layout.serialize_to_file(path)
                .expect("Failed to serialize workspace layout");

            // Deserialize the workspace layout from file
            let restored_layout = WorkspaceState::deserialize_from_file(path)
                .expect("Failed to deserialize workspace layout");

            // CRITICAL PROPERTY: All fields must be preserved exactly

            // Verify schema version is preserved
            prop_assert_eq!(
                layout.schema_version,
                restored_layout.schema_version,
                "Schema version not preserved during round trip"
            );

            // Verify open panel IDs are preserved (order matters)
            prop_assert_eq!(
                &layout.open_panel_ids,
                &restored_layout.open_panel_ids,
                "Open panel IDs not preserved during round trip"
            );

            // Verify panel positions are preserved
            prop_assert_eq!(
                layout.panel_positions.len(),
                restored_layout.panel_positions.len(),
                "Panel positions count not preserved during round trip"
            );

            for (panel_id, geometry) in &layout.panel_positions {
                let restored_geometry = restored_layout.panel_positions.get(panel_id);
                prop_assert!(
                    restored_geometry.is_some(),
                    "Panel {:?} geometry missing after round trip",
                    panel_id
                );

                let restored_geometry = restored_geometry.unwrap();

                // Verify all geometry fields are preserved
                prop_assert_eq!(
                    geometry.x,
                    restored_geometry.x,
                    "Panel {:?} x position not preserved",
                    panel_id
                );
                prop_assert_eq!(
                    geometry.y,
                    restored_geometry.y,
                    "Panel {:?} y position not preserved",
                    panel_id
                );
                prop_assert_eq!(
                    geometry.width,
                    restored_geometry.width,
                    "Panel {:?} width not preserved",
                    panel_id
                );
                prop_assert_eq!(
                    geometry.height,
                    restored_geometry.height,
                    "Panel {:?} height not preserved",
                    panel_id
                );
            }

            // Verify focused panel is preserved
            prop_assert_eq!(
                &layout.focused_panel,
                &restored_layout.focused_panel,
                "Focused panel not preserved during round trip"
            );

            // Verify docking configuration is preserved
            prop_assert_eq!(
                &layout.docking_configuration,
                &restored_layout.docking_configuration,
                "Docking configuration not preserved"
            );

            // Final verification: Complete equality check
            prop_assert_eq!(
                &layout,
                &restored_layout,
                "Workspace layouts not equal after round trip"
            );
        }
    }

    // ========================================================================
    // Test Strategies
    // ========================================================================

    /// Strategy for generating workspace states
    fn workspace_state_strategy() -> impl Strategy<Value = WorkspaceState> {
        (
            schema_version_strategy(),
            open_panel_ids_strategy(),
            docking_config_strategy(),
        )
            .prop_flat_map(|(schema_version, open_panel_ids, docking_config)| {
                // Generate panel positions for the open panels
                let panel_positions_strategy =
                    panel_positions_for_ids_strategy(open_panel_ids.clone());

                // Generate optional focused panel from open panels
                let focused_panel_strategy = if open_panel_ids.is_empty() {
                    Just(None).boxed()
                } else {
                    prop::option::of(prop::sample::select(open_panel_ids.clone())).boxed()
                };

                (
                    Just(schema_version),
                    Just(open_panel_ids),
                    panel_positions_strategy,
                    focused_panel_strategy,
                    Just(docking_config),
                )
            })
            .prop_map(
                |(
                    schema_version,
                    open_panel_ids,
                    panel_positions,
                    focused_panel,
                    docking_configuration,
                )| {
                    let mut workspace_owner = WorkspaceOwner::new();
                    workspace_owner.schema_version = schema_version;
                    workspace_owner.open_panel_ids = open_panel_ids.clone();
                    workspace_owner.panel_positions = panel_positions.clone();
                    workspace_owner.focused_panel = focused_panel.clone();
                    workspace_owner.docking_configuration = docking_configuration.clone();
                    
                    WorkspaceState {
                        workspace_owner,
                        schema_version,
                        open_panel_ids,
                        panel_positions,
                        focused_panel,
                        docking_configuration,
                    }
                },
            )
    }

    /// Strategy for generating schema versions
    fn schema_version_strategy() -> impl Strategy<Value = u32> {
        // Use current schema version to ensure valid states
        Just(WorkspaceState::CURRENT_SCHEMA_VERSION)
    }

    /// Strategy for generating open panel IDs
    fn open_panel_ids_strategy() -> impl Strategy<Value = Vec<PanelId>> {
        prop::collection::vec(panel_id_strategy(), 0..10)
    }

    /// Strategy for generating panel IDs
    fn panel_id_strategy() -> impl Strategy<Value = PanelId> {
        prop_oneof![
            Just(PanelId::new("viewport")),
            Just(PanelId::new("outliner")),
            Just(PanelId::new("content_browser")),
            Just(PanelId::new("inspector")),
            Just(PanelId::new("material")),
            Just(PanelId::new("terrain")),
            Just(PanelId::new("world")),
            Just(PanelId::new("sky_lighting")),
            Just(PanelId::new("audio")),
            Just(PanelId::new("diagnostics")),
            Just(PanelId::new("build_release")),
            // Also generate some random panel IDs
            "[a-z_]{3,15}".prop_map(PanelId::new),
        ]
    }

    /// Strategy for generating panel positions for specific panel IDs
    fn panel_positions_for_ids_strategy(
        panel_ids: Vec<PanelId>,
    ) -> impl Strategy<Value = HashMap<PanelId, PanelGeometry>> {
        // Generate geometries for each panel ID
        let geometries: Vec<_> = panel_ids
            .into_iter()
            .map(|id| (Just(id), panel_geometry_strategy()))
            .collect();

        if geometries.is_empty() {
            Just(HashMap::new()).boxed()
        } else {
            geometries.into_iter().fold(
                Just(HashMap::new()).boxed(),
                |acc, (id_strategy, geom_strategy)| {
                    (acc, id_strategy, geom_strategy)
                        .prop_map(|(mut map, id, geom)| {
                            map.insert(id, geom);
                            map
                        })
                        .boxed()
                },
            )
        }
    }

    /// Strategy for generating panel geometries
    fn panel_geometry_strategy() -> impl Strategy<Value = PanelGeometry> {
        (
            0.0f32..2000.0f32,   // x position
            0.0f32..2000.0f32,   // y position
            100.0f32..1920.0f32, // width (min 100, max typical screen width)
            100.0f32..1080.0f32, // height (min 100, max typical screen height)
        )
            .prop_map(|(x, y, width, height)| PanelGeometry::new(x, y, width, height))
    }

    /// Strategy for generating docking configurations
    fn docking_config_strategy() -> impl Strategy<Value = DockingConfig> {
        (
            any::<bool>(),                            // docking_enabled
            1.0f32..50.0f32,                          // snap_distance
            (100.0f32..500.0f32, 100.0f32..500.0f32), // min_panel_size
        )
            .prop_map(
                |(docking_enabled, snap_distance, min_panel_size)| DockingConfig {
                    docking_enabled,
                    snap_distance,
                    min_panel_size,
                },
            )
    }

    // ========================================================================
    // Unit Tests for Workspace Persistence Edge Cases
    // ========================================================================

    #[test]
    fn test_empty_workspace_round_trip() {
        let layout = WorkspaceState::new();

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
    }

    #[test]
    fn test_single_panel_round_trip() {
        let mut layout = WorkspaceState::new();
        let panel_id = PanelId::new("viewport");
        let geometry = PanelGeometry::floating(100.0, 200.0, 800.0, 600.0);

        layout.add_panel(panel_id.clone(), geometry);
        layout.set_focused_panel(Some(panel_id));

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
    }

    #[test]
    fn test_multiple_panels_round_trip() {
        let mut layout = WorkspaceState::new();

        let panels = vec![
            (
                PanelId::new("viewport"),
                PanelGeometry::floating(0.0, 0.0, 800.0, 600.0),
            ),
            (
                PanelId::new("outliner"),
                PanelGeometry::docked(800.0, 0.0, 300.0, 600.0, DockPosition::Right),
            ),
            (
                PanelId::new("inspector"),
                PanelGeometry::docked(0.0, 600.0, 1100.0, 300.0, DockPosition::Bottom),
            ),
        ];

        for (panel_id, geometry) in panels {
            layout.add_panel(panel_id, geometry);
        }

        layout.set_focused_panel(Some(PanelId::new("viewport")));

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
    }

    #[test]
    fn test_all_dock_positions_round_trip() {
        let mut layout = WorkspaceState::new();

        let dock_positions = vec![
            DockPosition::Left,
            DockPosition::Right,
            DockPosition::Top,
            DockPosition::Bottom,
            DockPosition::Center,
        ];

        for (i, dock_pos) in dock_positions.into_iter().enumerate() {
            let panel_id = PanelId::new(format!("panel_{}", i));
            let geometry = PanelGeometry::docked(
                (i as f32) * 100.0,
                (i as f32) * 100.0,
                300.0,
                200.0,
                dock_pos,
            );
            layout.add_panel(panel_id, geometry);
        }

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
    }

    #[test]
    fn test_custom_docking_config_round_trip() {
        let mut layout = WorkspaceState::new();
        layout.docking_configuration = DockingConfig {
            docking_enabled: false,
            snap_distance: 25.0,
            min_panel_size: (150.0, 100.0),
        };

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
    }

    #[test]
    fn test_no_focused_panel_round_trip() {
        let mut layout = WorkspaceState::new();
        let panel_id = PanelId::new("viewport");
        let geometry = PanelGeometry::floating(0.0, 0.0, 800.0, 600.0);

        layout.add_panel(panel_id, geometry);
        // Explicitly set no focused panel
        layout.set_focused_panel(None);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
        assert!(restored.focused_panel.is_none());
    }

    #[test]
    fn test_extreme_panel_positions_round_trip() {
        let mut layout = WorkspaceState::new();

        // Test with extreme but valid values
        let panel_id = PanelId::new("viewport");
        let geometry = PanelGeometry::floating(
            f32::MAX / 2.0, // Large but not overflow
            f32::MAX / 2.0,
            f32::MAX / 4.0,
            f32::MAX / 4.0,
        );

        layout.add_panel(panel_id, geometry);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        layout.serialize_to_file(path).unwrap();
        let restored = WorkspaceState::deserialize_from_file(path).unwrap();

        assert_eq!(layout, restored);
    }
}
