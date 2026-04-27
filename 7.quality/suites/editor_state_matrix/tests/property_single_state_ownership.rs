//! Property Test: Single State Ownership
//!
//! Feature: editor-state-ownership-normalization
//! Property 1: Single State Ownership
//! Validates: Requirements 1.2, 1.3, 2.1
//!
//! For any state item in the system, querying its owner returns exactly one owner container,
//! and this owner is one of the 4 defined containers (project_state, workspace_state,
//! world_state, diagnostics_state).

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, ProjectIdentity, ProjectState, StateContainerSystem, StateId,
    WorkspaceIdentity, WorkspaceState, WorldIdentity, WorldState,
};

/// Strategy for generating StateId values
fn any_state_id() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::ProjectState),
        Just(StateId::WorkspaceState),
        Just(StateId::DiagnosticsState),
        Just(StateId::ProjectIdentity),
        Just(StateId::WorkspaceIdentity),
        Just(StateId::SaveGeneration),
        Just(StateId::ContentSnapshots),
        Just(StateId::PanelLayout),
        Just(StateId::DockingConfig),
        Just(StateId::DiagnosticMessages),
        Just(StateId::TraceLineage),
    ]
}

/// Strategy for generating StateId values including world state
fn any_state_id_with_world() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::ProjectState),
        Just(StateId::WorkspaceState),
        Just(StateId::DiagnosticsState),
        Just(StateId::WorldState),
        Just(StateId::ProjectIdentity),
        Just(StateId::WorkspaceIdentity),
        Just(StateId::SaveGeneration),
        Just(StateId::ContentSnapshots),
        Just(StateId::PanelLayout),
        Just(StateId::DockingConfig),
        Just(StateId::DiagnosticMessages),
        Just(StateId::TraceLineage),
        Just(StateId::WorldIdentity),
        Just(StateId::TerrainState),
        Just(StateId::EnvironmentState),
    ]
}

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

fn create_test_system_with_world() -> StateContainerSystem {
    let mut system = create_test_system();

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

    system
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 1.2, 1.3, 2.1**
    ///
    /// Property 1: Single State Ownership
    ///
    /// For any state item in the system (without world state), querying its owner
    /// returns exactly one owner container, and this owner is one of the 3 defined
    /// containers (project_state, workspace_state, diagnostics_state).
    #[test]
    fn property_single_state_ownership_without_world(state_id in any_state_id()) {
        let system = create_test_system();

        // For any state item, it has exactly one owner
        let owner = system.get_owner(&state_id);
        prop_assert!(owner.is_some(), "State {:?} should have an owner", state_id);

        let owner = owner.unwrap();

        // Owner must be one of the 3 defined containers (no world state)
        prop_assert!(
            matches!(owner,
                OwnerId::ProjectState |
                OwnerId::WorkspaceState |
                OwnerId::DiagnosticsState
            ),
            "State {:?} has invalid owner {:?}",
            state_id,
            owner
        );
    }

    /// **Validates: Requirements 1.2, 1.3, 2.1**
    ///
    /// Property 1: Single State Ownership (with world state)
    ///
    /// For any state item in the system (with world state), querying its owner
    /// returns exactly one owner container, and this owner is one of the 4 defined
    /// containers (project_state, workspace_state, world_state, diagnostics_state).
    #[test]
    fn property_single_state_ownership_with_world(state_id in any_state_id_with_world()) {
        let system = create_test_system_with_world();

        // For any state item, it has exactly one owner
        let owner = system.get_owner(&state_id);
        prop_assert!(owner.is_some(), "State {:?} should have an owner", state_id);

        let owner = owner.unwrap();

        // Owner must be one of the 4 defined containers
        prop_assert!(
            matches!(owner,
                OwnerId::ProjectState |
                OwnerId::WorkspaceState |
                OwnerId::WorldState |
                OwnerId::DiagnosticsState
            ),
            "State {:?} has invalid owner {:?}",
            state_id,
            owner
        );
    }

    /// **Validates: Requirements 1.2, 1.3, 2.1**
    ///
    /// Property 1: Single State Ownership (consistency)
    ///
    /// For any state item, querying its owner multiple times returns the same result.
    #[test]
    fn property_single_state_ownership_consistency(state_id in any_state_id()) {
        let system = create_test_system();

        // Query owner multiple times
        let owner1 = system.get_owner(&state_id);
        let owner2 = system.get_owner(&state_id);
        let owner3 = system.get_owner(&state_id);

        // All queries should return the same result
        prop_assert_eq!(owner1, owner2, "Owner query should be consistent");
        prop_assert_eq!(owner2, owner3, "Owner query should be consistent");
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_property_single_state_ownership_basic() {
        let system = create_test_system();

        // Test a few specific states
        let owner = system.get_owner(&StateId::ProjectState);
        assert!(owner.is_some());
        assert_eq!(owner.unwrap(), OwnerId::ProjectState);

        let owner = system.get_owner(&StateId::WorkspaceState);
        assert!(owner.is_some());
        assert_eq!(owner.unwrap(), OwnerId::WorkspaceState);

        let owner = system.get_owner(&StateId::DiagnosticsState);
        assert!(owner.is_some());
        assert_eq!(owner.unwrap(), OwnerId::DiagnosticsState);
    }

    #[test]
    fn test_property_single_state_ownership_with_world_basic() {
        let system = create_test_system_with_world();

        // Test world-specific states
        let owner = system.get_owner(&StateId::WorldState);
        assert!(owner.is_some());
        assert_eq!(owner.unwrap(), OwnerId::WorldState);

        let owner = system.get_owner(&StateId::TerrainState);
        assert!(owner.is_some());
        assert_eq!(owner.unwrap(), OwnerId::WorldState);
    }
}
