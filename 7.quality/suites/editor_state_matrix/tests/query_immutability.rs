//! Property-Based Tests for Query Layer Immutability
//!
//! **Feature: editor-state-truth-normalization-phase3, Property 3: Query Immutability**
//!
//! **Validates: Requirements 2.8, 7.5, 11.3**
//!
//! This module contains property-based tests that verify all query methods in the
//! queries/ directory are read-only (take &Owner, not &mut Owner), ensuring queries
//! cannot mutate underlying state.

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::path::PathBuf;
    use stratumx_editor_state_containers::{
        DiagnosticMessage, DiagnosticSource, DockPosition, EnvironmentState, FailureCode,
        Severity, TraceId, TraceLineage,
    };
    use stratumx_editor_state_containers::owners::diagnostics_owner::DiagnosticsOwner;
    use stratumx_editor_state_containers::owners::project_owner::{
        ProjectIdentity, ProjectOwner, WorkspaceIdentity,
    };
    use stratumx_editor_state_containers::owners::workspace_owner::{
        PanelGeometry, PanelId, WorkspaceOwner,
    };
    use stratumx_editor_state_containers::owners::world_owner::{
        TerrainState, WorldIdentity, WorldOwner,
    };
    use stratumx_editor_state_containers::queries::diagnostics_queries::{
        DiagnosticsFailureCodesView, DiagnosticsMessagesView, DiagnosticsSummaryView,
        DiagnosticsTraceLineageView,
    };
    use stratumx_editor_state_containers::queries::project_queries::{
        ProjectIdentityView, ProjectSnapshotsView,
    };
    use stratumx_editor_state_containers::queries::workspace_queries::{
        WorkspaceDockingSummaryView, WorkspaceLayoutView, WorkspacePanelPositionsView,
    };
    use stratumx_editor_state_containers::queries::world_queries::{
        WorldDiagnosticsSummaryView, WorldEnvironmentSummaryView, WorldIdentityView,
        WorldTerrainSummaryView,
    };
    use stratumx_editor_state_containers::queries::ReadModel;
    use uuid::Uuid;

    // ========================================================================
    // Property 3: Query Immutability
    // ========================================================================
    //
    // **Feature: editor-state-truth-normalization-phase3, Property 3: Query Immutability**
    //
    // **Validates: Requirements 2.8, 7.5, 11.3**
    //
    // Property 3: Query Immutability
    //
    // *For any* query method in the queries/ directory, the method signature is
    // read-only (takes &Owner, not &mut Owner), ensuring queries cannot mutate
    // underlying state.
    //
    // This test validates that:
    // - All query methods take &Owner (immutable reference)
    // - Queries return immutable views or copied data
    // - Owner state remains unchanged after queries
    // - Multiple queries return consistent results

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn property_project_queries_are_immutable(
            project_name in "[a-z]{3,10}",
            workspace_name in "[a-z]{3,10}",
            save_generation in 0u64..1000,
            snapshot_count in 0usize..10,
        ) {
            // Create owner
            let project_identity = ProjectIdentity::new(
                Uuid::new_v4(),
                project_name.clone(),
                PathBuf::from("/test/project"),
            );
            let workspace_identity = WorkspaceIdentity::new(
                Uuid::new_v4(),
                workspace_name.clone(),
                PathBuf::from("/test/workspace"),
            );

            let mut owner = ProjectOwner::new(project_identity, workspace_identity);

            // Set save generation
            for _ in 0..save_generation {
                owner.increment_save_generation();
            }

            // Add snapshots
            for i in 0..snapshot_count {
                owner.add_snapshot(format!("Snapshot {}", i));
            }

            // Capture initial state
            let initial_generation = owner.get_save_generation();
            let initial_snapshot_count = owner.get_snapshots().len();

            // Execute queries (all take &owner, not &mut owner)
            let view1 = ProjectIdentityView::build(&owner);
            let view2 = ProjectSnapshotsView::build(&owner);

            // Verify queries return correct data
            prop_assert_eq!(&view1.project_name, &project_name);
            prop_assert_eq!(&view1.workspace_name, &workspace_name);
            prop_assert_eq!(view1.save_generation, save_generation);
            prop_assert_eq!(view2.snapshot_count, snapshot_count);

            // Critical: Verify owner state unchanged after queries
            prop_assert_eq!(owner.get_save_generation(), initial_generation);
            prop_assert_eq!(owner.get_snapshots().len(), initial_snapshot_count);

            // Verify multiple queries return consistent results
            let view1_again = ProjectIdentityView::build(&owner);
            prop_assert_eq!(&view1_again.project_name, &view1.project_name);
            prop_assert_eq!(view1_again.save_generation, view1.save_generation);
        }

        #[test]
        fn property_workspace_queries_are_immutable(
            panel_count in 0usize..10,
            docked_count in 0usize..5,
        ) {
            // Create owner
            let mut owner = WorkspaceOwner::new();

            // Add panels
            for i in 0..panel_count {
                let panel_id = PanelId::new(format!("panel_{}", i));
                let geometry = if i < docked_count {
                    PanelGeometry::docked(
                        i as f32 * 100.0,
                        i as f32 * 100.0,
                        800.0,
                        600.0,
                        DockPosition::Left,
                    )
                } else {
                    PanelGeometry::floating(
                        i as f32 * 100.0,
                        i as f32 * 100.0,
                        800.0,
                        600.0,
                    )
                };
                owner.add_panel(panel_id, geometry);
            }

            // Set focused panel if any panels exist
            if panel_count > 0 {
                owner.set_focused_panel(Some(PanelId::new("panel_0".to_string())));
            }

            // Capture initial state
            let initial_panel_count = owner.open_panel_ids.len();
            let initial_focused = owner.focused_panel.as_ref().map(|p| p.0.clone());

            // Execute queries (all take &owner, not &mut owner)
            let view1 = WorkspaceLayoutView::build(&owner);
            let view2 = WorkspacePanelPositionsView::build(&owner);
            let view3 = WorkspaceDockingSummaryView::build(&owner);

            // Verify queries return correct data
            prop_assert_eq!(view1.panel_count, panel_count);
            prop_assert_eq!(view2.panel_positions.len(), panel_count);
            prop_assert_eq!(view3.total_panels, panel_count);

            // Critical: Verify owner state unchanged after queries
            prop_assert_eq!(owner.open_panel_ids.len(), initial_panel_count);
            prop_assert_eq!(owner.focused_panel.as_ref().map(|p| p.0.clone()), initial_focused);

            // Verify multiple queries return consistent results
            let view1_again = WorkspaceLayoutView::build(&owner);
            prop_assert_eq!(view1_again.panel_count, view1.panel_count);
        }

        #[test]
        fn property_world_queries_are_immutable(
            world_name in "[a-z]{3,10}",
            has_terrain in any::<bool>(),
            has_environment in any::<bool>(),
            diagnostic_count in 0usize..10,
        ) {
            // Create owner
            let world_identity = WorldIdentity::new(
                Uuid::new_v4(),
                world_name.clone(),
                PathBuf::from("/test/world"),
            );

            let mut owner = WorldOwner::new(world_identity, "snapshot_123".to_string());

            // Add terrain if requested
            if has_terrain {
                let terrain = TerrainState::new(
                    (1024, 1024),
                    (1000.0, 1000.0),
                    "default".to_string(),
                );
                owner.set_terrain_state(Some(terrain));
            }

            // Add environment if requested
            if has_environment {
                let environment = EnvironmentState::new();
                owner.set_environment_state(Some(environment));
            }

            // Add diagnostics
            for i in 0..diagnostic_count {
                owner.add_diagnostic(DiagnosticMessage::new(
                    Severity::Error,
                    format!("Error {}", i),
                    DiagnosticSource::EditorComponent("World".to_string()),
                ));
            }

            // Capture initial state
            let initial_terrain = owner.get_terrain_state().is_some();
            let initial_environment = owner.get_environment_state().is_some();
            let initial_diagnostic_count = owner.get_diagnostics().len();

            // Execute queries (all take &owner, not &mut owner)
            let view1 = WorldIdentityView::build(&owner);
            let view2 = WorldTerrainSummaryView::build(&owner);
            let view3 = WorldEnvironmentSummaryView::build(&owner);
            let view4 = WorldDiagnosticsSummaryView::build(&owner);

            // Verify queries return correct data
            prop_assert_eq!(&view1.world_name, &world_name);
            prop_assert_eq!(view2.has_terrain, has_terrain);
            prop_assert_eq!(view3.has_environment, has_environment);
            prop_assert_eq!(view4.diagnostic_count, diagnostic_count);

            // Critical: Verify owner state unchanged after queries
            prop_assert_eq!(owner.get_terrain_state().is_some(), initial_terrain);
            prop_assert_eq!(owner.get_environment_state().is_some(), initial_environment);
            prop_assert_eq!(owner.get_diagnostics().len(), initial_diagnostic_count);

            // Verify multiple queries return consistent results
            let view1_again = WorldIdentityView::build(&owner);
            prop_assert_eq!(&view1_again.world_name, &view1.world_name);
        }

        #[test]
        fn property_diagnostics_queries_are_immutable(
            error_count in 0usize..10,
            warning_count in 0usize..10,
            failure_code_count in 0usize..5,
            trace_count in 0usize..5,
        ) {
            // Create owner
            let mut owner = DiagnosticsOwner::new();

            // Add error messages
            for i in 0..error_count {
                owner.add_message(DiagnosticMessage::new(
                    Severity::Error,
                    format!("Error {}", i),
                    DiagnosticSource::CommandSpine,
                ));
            }

            // Add warning messages
            for i in 0..warning_count {
                owner.add_message(DiagnosticMessage::new(
                    Severity::Warning,
                    format!("Warning {}", i),
                    DiagnosticSource::CommandSpine,
                ));
            }

            // Add failure codes
            let failure_codes = vec![
                FailureCode::WorldOpenFailed,
                FailureCode::MaterialBindingFailed,
                FailureCode::TerrainSimulationFailed,
                FailureCode::BuildFailed,
                FailureCode::RuntimeInitFailed,
            ];
            for i in 0..failure_code_count.min(failure_codes.len()) {
                owner.add_failure_code(failure_codes[i]);
            }

            // Add trace lineage
            for i in 0..trace_count {
                let trace_id = TraceId::new();
                let lineage = TraceLineage::new(
                    trace_id.clone(),
                    format!("operation_{}", i),
                    i as u64,
                );
                owner.add_trace_lineage(trace_id, lineage);
            }

            // Capture initial state
            let initial_message_count = owner.messages.len();
            let initial_failure_count = owner.failure_codes.len();
            let initial_trace_count = owner.trace_lineage.len();

            // Execute queries (all take &owner, not &mut owner)
            let view1 = DiagnosticsSummaryView::build(&owner);
            let view2 = DiagnosticsMessagesView::build(&owner);
            let view3 = DiagnosticsFailureCodesView::build(&owner);
            let view4 = DiagnosticsTraceLineageView::build(&owner);

            // Verify queries return correct data
            prop_assert_eq!(view1.error_count, error_count);
            prop_assert_eq!(view1.warning_count, warning_count);
            prop_assert_eq!(view1.total_messages, error_count + warning_count);
            prop_assert_eq!(view2.messages.len(), error_count + warning_count);
            prop_assert_eq!(view3.failure_count, failure_code_count.min(failure_codes.len()));
            prop_assert_eq!(view4.trace_count, trace_count);

            // Critical: Verify owner state unchanged after queries
            prop_assert_eq!(owner.messages.len(), initial_message_count);
            prop_assert_eq!(owner.failure_codes.len(), initial_failure_count);
            prop_assert_eq!(owner.trace_lineage.len(), initial_trace_count);

            // Verify multiple queries return consistent results
            let view1_again = DiagnosticsSummaryView::build(&owner);
            prop_assert_eq!(view1_again.error_count, view1.error_count);
            prop_assert_eq!(view1_again.warning_count, view1.warning_count);
        }
    }

    // ========================================================================
    // Unit Tests for Query Immutability
    // ========================================================================

    #[test]
    fn test_readmodel_trait_enforces_immutable_reference() {
        // This test verifies that the ReadModel trait signature enforces
        // immutable references at compile time

        let project_identity = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_identity = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let owner = ProjectOwner::new(project_identity, workspace_identity);

        // This compiles because ReadModel::build takes &Owner
        let _view = ProjectIdentityView::build(&owner);

        // The following would NOT compile (cannot pass &mut to &):
        // let mut owner_mut = owner;
        // let _view = ProjectIdentityView::build(&mut owner_mut);

        // Verify owner is still usable (not moved or mutated)
        assert_eq!(owner.get_save_generation(), 0);
    }

    #[test]
    fn test_all_query_methods_take_immutable_references() {
        // This test verifies that all query implementations follow the
        // ReadModel trait contract of taking immutable references

        // Project queries
        let project_identity =
            ProjectIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test"));
        let workspace_identity =
            WorkspaceIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test"));
        let project_owner = ProjectOwner::new(project_identity, workspace_identity);
        let _view = ProjectIdentityView::build(&project_owner);
        let _view = ProjectSnapshotsView::build(&project_owner);

        // Workspace queries
        let workspace_owner = WorkspaceOwner::new();
        let _view = WorkspaceLayoutView::build(&workspace_owner);
        let _view = WorkspacePanelPositionsView::build(&workspace_owner);
        let _view = WorkspaceDockingSummaryView::build(&workspace_owner);

        // World queries
        let world_identity =
            WorldIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test"));
        let world_owner = WorldOwner::new(world_identity, "snapshot".to_string());
        let _view = WorldIdentityView::build(&world_owner);
        let _view = WorldTerrainSummaryView::build(&world_owner);
        let _view = WorldEnvironmentSummaryView::build(&world_owner);
        let _view = WorldDiagnosticsSummaryView::build(&world_owner);

        // Diagnostics queries
        let diagnostics_owner = DiagnosticsOwner::new();
        let _view = DiagnosticsSummaryView::build(&diagnostics_owner);
        let _view = DiagnosticsMessagesView::build(&diagnostics_owner);
        let _view = DiagnosticsFailureCodesView::build(&diagnostics_owner);
        let _view = DiagnosticsTraceLineageView::build(&diagnostics_owner);

        // All owners are still usable (not moved or mutated)
        assert_eq!(project_owner.get_save_generation(), 0);
        assert_eq!(workspace_owner.open_panel_ids.len(), 0);
        assert_eq!(world_owner.get_diagnostics().len(), 0);
        assert_eq!(diagnostics_owner.messages.len(), 0);
    }

    #[test]
    fn test_query_results_are_independent_of_owner_lifetime() {
        // This test verifies that query results can outlive the owner
        // (because they return owned data, not references to owner internals)

        let view = {
            let project_identity = ProjectIdentity::new(
                Uuid::new_v4(),
                "Test Project".to_string(),
                PathBuf::from("/test/project"),
            );
            let workspace_identity = WorkspaceIdentity::new(
                Uuid::new_v4(),
                "Test Workspace".to_string(),
                PathBuf::from("/test/workspace"),
            );

            let owner = ProjectOwner::new(project_identity, workspace_identity);

            // Build view while owner is in scope
            ProjectIdentityView::build(&owner)

            // owner goes out of scope here
        };

        // View is still valid and usable
        assert_eq!(view.project_name, "Test Project");
        assert_eq!(view.workspace_name, "Test Workspace");
    }
}
