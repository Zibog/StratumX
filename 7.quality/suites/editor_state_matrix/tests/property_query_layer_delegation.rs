//! Property Test: Query Layer Delegation
//!
//! Feature: editor-state-ownership-normalization
//! Property 8: Query Layer Delegation
//! Validates: Requirements 4.4
//!
//! For any query result from the query layer, the result is equivalent to directly
//! accessing the owner container or cache layer.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState, QueryLayer, Severity,
    StateContainerSystem, WorkspaceIdentity, WorkspaceState, WorldIdentity, WorldState,
};

// Strategy for generating project identities
fn project_identity_strategy() -> impl Strategy<Value = ProjectIdentity> {
    (any::<u128>(), "[a-zA-Z0-9 ]{5,20}", "[a-z/]{5,20}").prop_map(|(id_num, name, path)| {
        let uuid = Uuid::from_u128(id_num);
        ProjectIdentity::new(uuid, name, PathBuf::from(format!("/{}", path)))
    })
}

// Strategy for generating workspace identities
fn workspace_identity_strategy() -> impl Strategy<Value = WorkspaceIdentity> {
    (any::<u128>(), "[a-zA-Z0-9 ]{5,20}", "[a-z/]{5,20}").prop_map(|(id_num, name, path)| {
        let uuid = Uuid::from_u128(id_num);
        WorkspaceIdentity::new(uuid, name, PathBuf::from(format!("/{}", path)))
    })
}

// Strategy for generating save generations
fn save_generation_strategy() -> impl Strategy<Value = u64> {
    0u64..1000u64
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 4.4**
    ///
    /// Property 8: Query Layer Delegation (project state)
    ///
    /// Query layer results for project state match direct access to owner container.
    #[test]
    fn property_query_layer_delegation_project_state(
        project_id in project_identity_strategy(),
        workspace_id in workspace_identity_strategy(),
        save_gen in save_generation_strategy(),
    ) {
        // Create state container system
        let mut project_state_obj = ProjectState::new(project_id.clone(), workspace_id.clone());

        // Set save generation
        for _ in 0..save_gen {
            project_state_obj.increment_save_generation();
        }

        let project_state = Arc::new(Mutex::new(project_state_obj));
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

        let state_system = Arc::new(StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        ).expect("Failed to create test system"));

        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));
        let query_layer = QueryLayer::new(state_system.clone());

        // Query through query layer
        let query_project_id = query_layer.get_project_identity();
        let query_workspace_id = query_layer.get_workspace_identity();
        let query_save_gen = query_layer.get_save_generation();

        // Direct access to owner container
        let direct_project_id = {
            let ps = project_state.lock().unwrap();
            ps.get_project_identity().clone()
        };
        let direct_workspace_id = {
            let ps = project_state.lock().unwrap();
            ps.get_workspace_identity().clone()
        };
        let direct_save_gen = {
            let ps = project_state.lock().unwrap();
            ps.get_save_generation()
        };

        // Verify equivalence
        prop_assert_eq!(query_project_id.project_id, direct_project_id.project_id);
        prop_assert_eq!(query_project_id.project_name, direct_project_id.project_name);
        prop_assert_eq!(query_workspace_id.workspace_id, direct_workspace_id.workspace_id);
        prop_assert_eq!(query_workspace_id.workspace_name, direct_workspace_id.workspace_name);
        prop_assert_eq!(query_save_gen, direct_save_gen);
    }

    /// **Validates: Requirements 4.4**
    ///
    /// Property 8: Query Layer Delegation (workspace state)
    ///
    /// Query layer results for workspace state match direct access to owner container.
    #[test]
    fn property_query_layer_delegation_workspace_state(
        project_id in project_identity_strategy(),
        workspace_id in workspace_identity_strategy(),
    ) {
        let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

        let state_system = Arc::new(StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        ).expect("Failed to create test system"));

        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));
        let query_layer = QueryLayer::new(state_system.clone());

        // Query through query layer
        let query_panels = query_layer.get_open_panels();
        let query_focused = query_layer.get_focused_panel();

        // Direct access to owner container
        let direct_panels = {
            let ws = workspace_state.lock().unwrap();
            ws.get_open_panels().to_vec()
        };
        let direct_focused = {
            let ws = workspace_state.lock().unwrap();
            ws.focused_panel.clone()
        };

        // Verify equivalence
        prop_assert_eq!(query_panels, direct_panels);
        prop_assert_eq!(query_focused, direct_focused);
    }

    /// **Validates: Requirements 4.4**
    ///
    /// Property 8: Query Layer Delegation (diagnostics state)
    ///
    /// Query layer results for diagnostics state match direct access to owner container.
    #[test]
    fn property_query_layer_delegation_diagnostics_state(
        project_id in project_identity_strategy(),
        workspace_id in workspace_identity_strategy(),
        severity in prop_oneof![
            Just(Severity::Error),
            Just(Severity::Warning),
            Just(Severity::Info),
        ],
    ) {
        let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

        let state_system = Arc::new(StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        ).expect("Failed to create test system"));

        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));
        let query_layer = QueryLayer::new(state_system.clone());

        // Query through query layer
        let query_diagnostics = query_layer.get_diagnostics_by_severity(severity);
        let query_all = query_layer.get_all_diagnostics();
        let query_codes = query_layer.get_failure_codes();

        // Direct access to owner container
        let direct_diagnostics = {
            let ds = diagnostics_state.lock().unwrap();
            ds.get_messages_by_severity(severity)
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        };
        let direct_all = {
            let ds = diagnostics_state.lock().unwrap();
            ds.messages.clone()
        };
        let direct_codes = {
            let ds = diagnostics_state.lock().unwrap();
            ds.failure_codes.clone()
        };

        // Verify equivalence (compare lengths since DiagnosticMessage doesn't implement PartialEq)
        prop_assert_eq!(query_diagnostics.len(), direct_diagnostics.len());
        prop_assert_eq!(query_all.len(), direct_all.len());
        prop_assert_eq!(query_codes, direct_codes);
    }

    /// **Validates: Requirements 4.4**
    ///
    /// Property 8: Query Layer Delegation (world state)
    ///
    /// Query layer results for world state match direct access to owner container.
    #[test]
    fn property_query_layer_delegation_world_state(
        project_id in project_identity_strategy(),
        workspace_id in workspace_identity_strategy(),
        world_id_num in any::<u128>(),
        world_name in "[a-zA-Z0-9 ]{5,20}",
        snapshot_ref in "[a-z0-9_]{5,20}",
    ) {
        let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

        let mut state_system = StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        ).expect("Failed to create test system");

        // Add world state
        let world_id = WorldIdentity::new(
            Uuid::from_u128(world_id_num),
            world_name.clone(),
            PathBuf::from("/test/world"),
        );
        let world_state = Arc::new(Mutex::new(WorldState::new(world_id.clone(), snapshot_ref.clone())));
        state_system.set_world_state(world_state.clone());

        let state_system = Arc::new(state_system);
        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));
        let query_layer = QueryLayer::new(state_system.clone());

        // Query through query layer
        let query_world_id = query_layer.get_world_identity();
        let query_snapshot = query_layer.get_world_snapshot_ref();
        let query_terrain = query_layer.get_terrain_state();
        let query_environment = query_layer.get_environment_state();

        // Direct access to owner container
        let direct_world_id = {
            let ws = world_state.lock().unwrap();
            Some(ws.get_world_identity().clone())
        };
        let direct_snapshot = {
            let ws = world_state.lock().unwrap();
            Some(ws.get_snapshot_ref().to_string())
        };
        let direct_terrain = {
            let ws = world_state.lock().unwrap();
            ws.get_terrain_state().cloned()
        };
        let direct_environment = {
            let ws = world_state.lock().unwrap();
            ws.get_environment_state().cloned()
        };

        // Verify equivalence (compare is_some() since TerrainState/EnvironmentState don't implement PartialEq)
        prop_assert_eq!(query_world_id.as_ref().map(|id| id.world_id), direct_world_id.as_ref().map(|id| id.world_id));
        prop_assert_eq!(query_world_id.as_ref().map(|id| id.world_name.clone()), direct_world_id.as_ref().map(|id| id.world_name.clone()));
        prop_assert_eq!(query_snapshot, direct_snapshot);
        prop_assert_eq!(query_terrain.is_some(), direct_terrain.is_some());
        prop_assert_eq!(query_environment.is_some(), direct_environment.is_some());
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_query_layer_delegation_basic() {
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let project_state = Arc::new(Mutex::new(ProjectState::new(
            project_id.clone(),
            workspace_id.clone(),
        )));
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

        let state_system = Arc::new(
            StateContainerSystem::new(
                project_state.clone(),
                workspace_state.clone(),
                diagnostics_state.clone(),
            )
            .expect("Failed to create test system"),
        );

        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));
        let query_layer = QueryLayer::new(state_system.clone());

        // Query through query layer
        let query_project_id = query_layer.get_project_identity();

        // Direct access
        let direct_project_id = {
            let ps = project_state.lock().unwrap();
            ps.get_project_identity().clone()
        };

        // Verify equivalence
        assert_eq!(query_project_id.project_id, direct_project_id.project_id);
        assert_eq!(
            query_project_id.project_name,
            direct_project_id.project_name
        );
    }
}
