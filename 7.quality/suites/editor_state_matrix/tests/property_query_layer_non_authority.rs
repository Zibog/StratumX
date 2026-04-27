//! Property Test: Query Layer Non-Authority
//!
//! **Validates: Requirements 4.2, 4.3**
//!
//! **Property 6: Query Layer Non-Authority**
//! For any query layer instance, destroying and recreating it produces
//! equivalent query results, and the state container system continues
//! operating without data loss.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use stratumx_editor_state_containers::{
    CacheLayer, DiagnosticsState, ProjectIdentity, ProjectState, QueryLayer, StateContainerSystem,
    WorkspaceIdentity, WorkspaceState,
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

    /// Feature: editor-state-ownership-normalization, Property 6: Query Layer Non-Authority
    ///
    /// Tests that destroying and recreating the query layer produces equivalent results
    /// and the state container system continues operating without data loss.
    #[test]
    fn property_query_layer_non_authority(
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
        ).expect("Failed to create state system"));

        let _cache_layer = Arc::new(CacheLayer::new(state_system.clone()));

        // Create first query layer
        let query_layer_1 = QueryLayer::new(state_system.clone());

        // Query state through first query layer
        let project_identity_1 = query_layer_1.get_project_identity();
        let workspace_identity_1 = query_layer_1.get_workspace_identity();
        let save_generation_1 = query_layer_1.get_save_generation();
        let open_panels_1 = query_layer_1.get_open_panels();
        let focused_panel_1 = query_layer_1.get_focused_panel();

        // Destroy first query layer (drop it)
        drop(query_layer_1);

        // Verify state container system still operates
        {
            let mut ps = project_state.lock().unwrap();
            ps.increment_save_generation();
        }

        // Create second query layer
        let query_layer_2 = QueryLayer::new(state_system.clone());

        // Query state through second query layer
        let project_identity_2 = query_layer_2.get_project_identity();
        let workspace_identity_2 = query_layer_2.get_workspace_identity();
        let save_generation_2 = query_layer_2.get_save_generation();
        let open_panels_2 = query_layer_2.get_open_panels();
        let focused_panel_2 = query_layer_2.get_focused_panel();

        // Verify project identity is preserved
        prop_assert_eq!(project_identity_1.project_id, project_identity_2.project_id);
        prop_assert_eq!(project_identity_1.project_name, project_identity_2.project_name);
        prop_assert_eq!(project_identity_1.project_path, project_identity_2.project_path);

        // Verify workspace identity is preserved
        prop_assert_eq!(workspace_identity_1.workspace_id, workspace_identity_2.workspace_id);
        prop_assert_eq!(workspace_identity_1.workspace_name, workspace_identity_2.workspace_name);
        prop_assert_eq!(workspace_identity_1.workspace_path, workspace_identity_2.workspace_path);

        // Verify save generation increased (state system continued operating)
        prop_assert_eq!(save_generation_2, save_generation_1 + 1);

        // Verify workspace state is preserved
        prop_assert_eq!(open_panels_1, open_panels_2);
        prop_assert_eq!(focused_panel_1, focused_panel_2);

        // Verify no data loss - state container system still has all data
        let final_project_state = project_state.lock().unwrap();
        prop_assert_eq!(final_project_state.get_project_identity().project_id, project_id.project_id);
        prop_assert_eq!(final_project_state.get_save_generation(), save_gen + 1);
    }
}
