//! Property Test: Query Layer Read-Only
//!
//! **Validates: Requirements 4.1**
//!
//! **Property 7: Query Layer Read-Only**
//! For any sequence of query layer operations, the owner container state
//! remains unchanged.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use stratumx_editor_state_containers::{
    CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState, QueryLayer, Severity,
    StateContainerSystem, WorkspaceIdentity, WorkspaceState,
};
use uuid::Uuid;

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

    /// Feature: editor-state-ownership-normalization, Property 7: Query Layer Read-Only
    ///
    /// Tests that query layer operations do not mutate owner container state.
    #[test]
    fn property_query_layer_read_only(
        project_id in project_identity_strategy(),
        workspace_id in workspace_identity_strategy(),
        save_gen in save_generation_strategy(),
    ) {
        // Create state container system
        let mut project_state = ProjectState::new(project_id.clone(), workspace_id.clone());

        // Set save generation
        for _ in 0..save_gen {
            project_state.increment_save_generation();
        }

        let project_state = Arc::new(Mutex::new(project_state));
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

        let state_system = Arc::new(StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        ).expect("Failed to create test system"));

        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));
        let query_layer = QueryLayer::new(state_system.clone());

        // Capture initial state
        let initial_project_id = {
            let ps = project_state.lock().unwrap();
            ps.get_project_identity().project_id
        };
        let initial_workspace_id = {
            let ps = project_state.lock().unwrap();
            ps.get_workspace_identity().workspace_id
        };
        let initial_save_gen = {
            let ps = project_state.lock().unwrap();
            ps.get_save_generation()
        };
        let initial_panel_count = {
            let ws = workspace_state.lock().unwrap();
            ws.get_open_panels().len()
        };
        let initial_diagnostic_count = {
            let ds = diagnostics_state.lock().unwrap();
            ds.messages.len()
        };

        // Perform multiple query operations
        let _ = query_layer.get_project_identity();
        let _ = query_layer.get_workspace_identity();
        let _ = query_layer.get_save_generation();
        let _ = query_layer.get_open_panels();
        let _ = query_layer.get_focused_panel();
        let _ = query_layer.get_world_identity();
        let _ = query_layer.get_terrain_state();
        let _ = query_layer.get_environment_state();
        let _ = query_layer.get_diagnostics_by_severity(Severity::Error);
        let _ = query_layer.get_diagnostics_by_severity(Severity::Warning);
        let _ = query_layer.get_diagnostics_by_severity(Severity::Info);
        let _ = query_layer.get_all_diagnostics();
        let _ = query_layer.get_failure_codes();

        // Verify state remains unchanged
        let final_project_id = {
            let ps = project_state.lock().unwrap();
            ps.get_project_identity().project_id
        };
        let final_workspace_id = {
            let ps = project_state.lock().unwrap();
            ps.get_workspace_identity().workspace_id
        };
        let final_save_gen = {
            let ps = project_state.lock().unwrap();
            ps.get_save_generation()
        };
        let final_panel_count = {
            let ws = workspace_state.lock().unwrap();
            ws.get_open_panels().len()
        };
        let final_diagnostic_count = {
            let ds = diagnostics_state.lock().unwrap();
            ds.messages.len()
        };

        // Assert no mutations occurred
        prop_assert_eq!(initial_project_id, final_project_id);
        prop_assert_eq!(initial_workspace_id, final_workspace_id);
        prop_assert_eq!(initial_save_gen, final_save_gen);
        prop_assert_eq!(initial_panel_count, final_panel_count);
        prop_assert_eq!(initial_diagnostic_count, final_diagnostic_count);
    }
}
