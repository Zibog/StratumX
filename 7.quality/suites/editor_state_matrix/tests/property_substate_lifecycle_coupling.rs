//! Property Test: Substate Lifecycle Coupling
//!
//! Feature: editor-state-ownership-normalization
//! Property 9: Substate Lifecycle Coupling
//! Validates: Requirements 6.3, 6.4
//!
//! For any owner container C and its substate S, destroying C destroys S,
//! and S cannot exist independently of C.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, ProjectIdentity, ProjectState, StateContainerSystem, StateId,
    WorkspaceIdentity, WorkspaceState, WorldIdentity, WorldState,
};

fn create_test_system() -> StateContainerSystem {
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

    let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));

    StateContainerSystem::new(project_state, workspace_state, diagnostics_state)
        .expect("Failed to create test system")
}

/// Strategy for generating project substates
fn project_substate() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::ProjectIdentity),
        Just(StateId::WorkspaceIdentity),
        Just(StateId::SaveGeneration),
        Just(StateId::ContentSnapshots),
    ]
}

/// Strategy for generating workspace substates
fn workspace_substate() -> impl Strategy<Value = StateId> {
    prop_oneof![Just(StateId::PanelLayout), Just(StateId::DockingConfig),]
}

/// Strategy for generating diagnostics substates
fn diagnostics_substate() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::DiagnosticMessages),
        Just(StateId::TraceLineage),
    ]
}

/// Strategy for generating world substates
fn world_substate() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::WorldIdentity),
        Just(StateId::TerrainState),
        Just(StateId::EnvironmentState),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 6.3, 6.4**
    ///
    /// Property 9: Substate Lifecycle Coupling (project substates)
    ///
    /// Project substates are owned by ProjectState and cannot exist independently.
    #[test]
    fn property_substate_lifecycle_coupling_project(substate in project_substate()) {
        let system = create_test_system();

        // Verify substate is owned by ProjectState
        let owner = system.get_owner(&substate);
        prop_assert!(owner.is_some(), "Substate {:?} should have an owner", substate);
        prop_assert_eq!(owner.unwrap(), OwnerId::ProjectState, "Substate {:?} should be owned by ProjectState", substate);

        // Verify ProjectState container exists
        let project_owner = system.get_owner(&StateId::ProjectState);
        prop_assert!(project_owner.is_some(), "ProjectState container should exist");
        prop_assert_eq!(project_owner.unwrap(), OwnerId::ProjectState);
    }

    /// **Validates: Requirements 6.3, 6.4**
    ///
    /// Property 9: Substate Lifecycle Coupling (workspace substates)
    ///
    /// Workspace substates are owned by WorkspaceState and cannot exist independently.
    #[test]
    fn property_substate_lifecycle_coupling_workspace(substate in workspace_substate()) {
        let system = create_test_system();

        // Verify substate is owned by WorkspaceState
        let owner = system.get_owner(&substate);
        prop_assert!(owner.is_some(), "Substate {:?} should have an owner", substate);
        prop_assert_eq!(owner.unwrap(), OwnerId::WorkspaceState, "Substate {:?} should be owned by WorkspaceState", substate);

        // Verify WorkspaceState container exists
        let workspace_owner = system.get_owner(&StateId::WorkspaceState);
        prop_assert!(workspace_owner.is_some(), "WorkspaceState container should exist");
        prop_assert_eq!(workspace_owner.unwrap(), OwnerId::WorkspaceState);
    }

    /// **Validates: Requirements 6.3, 6.4**
    ///
    /// Property 9: Substate Lifecycle Coupling (diagnostics substates)
    ///
    /// Diagnostics substates are owned by DiagnosticsState and cannot exist independently.
    #[test]
    fn property_substate_lifecycle_coupling_diagnostics(substate in diagnostics_substate()) {
        let system = create_test_system();

        // Verify substate is owned by DiagnosticsState
        let owner = system.get_owner(&substate);
        prop_assert!(owner.is_some(), "Substate {:?} should have an owner", substate);
        prop_assert_eq!(owner.unwrap(), OwnerId::DiagnosticsState, "Substate {:?} should be owned by DiagnosticsState", substate);

        // Verify DiagnosticsState container exists
        let diagnostics_owner = system.get_owner(&StateId::DiagnosticsState);
        prop_assert!(diagnostics_owner.is_some(), "DiagnosticsState container should exist");
        prop_assert_eq!(diagnostics_owner.unwrap(), OwnerId::DiagnosticsState);
    }

    /// **Validates: Requirements 6.3, 6.4**
    ///
    /// Property 9: Substate Lifecycle Coupling (world substates)
    ///
    /// World substates are owned by WorldState and destroyed when WorldState is cleared.
    #[test]
    fn property_substate_lifecycle_coupling_world(substate in world_substate()) {
        let mut system = create_test_system();

        // Initially, world substates should not exist
        let owner_before = system.get_owner(&substate);
        prop_assert!(owner_before.is_none(), "World substate {:?} should not exist before world is opened", substate);

        // Open world
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        let world_state = Arc::new(Mutex::new(WorldState::new(world_id, "snapshot_123".to_string())));
        system.set_world_state(world_state);

        // Verify substate is now owned by WorldState
        let owner_after_open = system.get_owner(&substate);
        prop_assert!(owner_after_open.is_some(), "World substate {:?} should exist after world is opened", substate);
        prop_assert_eq!(owner_after_open.unwrap(), OwnerId::WorldState, "World substate {:?} should be owned by WorldState", substate);

        // Close world (destroy WorldState)
        system.clear_world_state();

        // Verify substate is destroyed along with WorldState
        let owner_after_close = system.get_owner(&substate);
        prop_assert!(owner_after_close.is_none(), "World substate {:?} should be destroyed when WorldState is cleared", substate);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_project_substate_coupling() {
        let system = create_test_system();

        // All project substates should be owned by ProjectState
        assert_eq!(
            system.get_owner(&StateId::ProjectIdentity),
            Some(OwnerId::ProjectState)
        );
        assert_eq!(
            system.get_owner(&StateId::WorkspaceIdentity),
            Some(OwnerId::ProjectState)
        );
        assert_eq!(
            system.get_owner(&StateId::SaveGeneration),
            Some(OwnerId::ProjectState)
        );
        assert_eq!(
            system.get_owner(&StateId::ContentSnapshots),
            Some(OwnerId::ProjectState)
        );
    }

    #[test]
    fn test_workspace_substate_coupling() {
        let system = create_test_system();

        // All workspace substates should be owned by WorkspaceState
        assert_eq!(
            system.get_owner(&StateId::PanelLayout),
            Some(OwnerId::WorkspaceState)
        );
        assert_eq!(
            system.get_owner(&StateId::DockingConfig),
            Some(OwnerId::WorkspaceState)
        );
    }

    #[test]
    fn test_diagnostics_substate_coupling() {
        let system = create_test_system();

        // All diagnostics substates should be owned by DiagnosticsState
        assert_eq!(
            system.get_owner(&StateId::DiagnosticMessages),
            Some(OwnerId::DiagnosticsState)
        );
        assert_eq!(
            system.get_owner(&StateId::TraceLineage),
            Some(OwnerId::DiagnosticsState)
        );
    }

    #[test]
    fn test_world_substate_lifecycle() {
        let mut system = create_test_system();

        // Initially no world substates
        assert!(system.get_owner(&StateId::WorldIdentity).is_none());
        assert!(system.get_owner(&StateId::TerrainState).is_none());
        assert!(system.get_owner(&StateId::EnvironmentState).is_none());

        // Open world
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        let world_state = Arc::new(Mutex::new(WorldState::new(
            world_id,
            "snapshot_123".to_string(),
        )));
        system.set_world_state(world_state);

        // World substates should now exist
        assert_eq!(
            system.get_owner(&StateId::WorldIdentity),
            Some(OwnerId::WorldState)
        );
        assert_eq!(
            system.get_owner(&StateId::TerrainState),
            Some(OwnerId::WorldState)
        );
        assert_eq!(
            system.get_owner(&StateId::EnvironmentState),
            Some(OwnerId::WorldState)
        );

        // Close world
        system.clear_world_state();

        // World substates should be destroyed
        assert!(system.get_owner(&StateId::WorldIdentity).is_none());
        assert!(system.get_owner(&StateId::TerrainState).is_none());
        assert!(system.get_owner(&StateId::EnvironmentState).is_none());
    }
}
