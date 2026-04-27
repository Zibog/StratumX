//! Property Test: Ownership Transfer Atomicity
//!
//! Feature: editor-state-ownership-normalization
//! Property 2: Ownership Transfer Atomicity
//! Validates: Requirements 2.4
//!
//! For any state ownership transfer operation, at no point during the transfer does the state
//! have zero owners or multiple owners - the previous owner releases authority atomically with
//! the new owner claiming it.

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

/// Strategy for generating world state IDs that can be transferred
fn world_state_id() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::WorldState),
        Just(StateId::WorldIdentity),
        Just(StateId::TerrainState),
        Just(StateId::EnvironmentState),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 2.4**
    ///
    /// Property 2: Ownership Transfer Atomicity
    ///
    /// When world state is added (ownership transfer), the state transitions atomically
    /// from no owner to having an owner. At no point does the state have multiple owners.
    #[test]
    fn property_ownership_transfer_atomicity_world_open(state_id in world_state_id()) {
        let mut system = create_test_system();

        // Before world is opened, world state should have no owner
        let owner_before = system.get_owner(&state_id);
        prop_assert!(owner_before.is_none(), "State {:?} should have no owner before world is opened", state_id);

        // Open world (ownership transfer)
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        let world_state = Arc::new(Mutex::new(WorldState::new(world_id, "snapshot_123".to_string())));
        system.set_world_state(world_state);

        // After world is opened, world state should have exactly one owner
        let owner_after = system.get_owner(&state_id);
        prop_assert!(owner_after.is_some(), "State {:?} should have an owner after world is opened", state_id);
        prop_assert_eq!(owner_after.unwrap(), OwnerId::WorldState, "State {:?} should be owned by WorldState", state_id);

        // Validate ownership uniqueness (no multiple owners)
        let validation_result = system.validate_ownership_uniqueness();
        prop_assert!(validation_result.is_ok(), "Ownership validation should pass after world open: {:?}", validation_result);
    }

    /// **Validates: Requirements 2.4**
    ///
    /// Property 2: Ownership Transfer Atomicity (world close)
    ///
    /// When world state is removed (ownership transfer), the state transitions atomically
    /// from having an owner to no owner. At no point does the state have multiple owners.
    #[test]
    fn property_ownership_transfer_atomicity_world_close(state_id in world_state_id()) {
        let mut system = create_test_system();

        // Open world first
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        let world_state = Arc::new(Mutex::new(WorldState::new(world_id, "snapshot_123".to_string())));
        system.set_world_state(world_state);

        // Verify state has owner
        let owner_before = system.get_owner(&state_id);
        prop_assert!(owner_before.is_some(), "State {:?} should have an owner before world is closed", state_id);
        prop_assert_eq!(owner_before.unwrap(), OwnerId::WorldState);

        // Close world (ownership transfer)
        system.clear_world_state();

        // After world is closed, world state should have no owner
        let owner_after = system.get_owner(&state_id);
        prop_assert!(owner_after.is_none(), "State {:?} should have no owner after world is closed", state_id);

        // Validate ownership uniqueness (no multiple owners)
        let validation_result = system.validate_ownership_uniqueness();
        prop_assert!(validation_result.is_ok(), "Ownership validation should pass after world close: {:?}", validation_result);
    }

    /// **Validates: Requirements 2.4**
    ///
    /// Property 2: Ownership Transfer Atomicity (derived state registration)
    ///
    /// When derived state is registered, it atomically gains an owner.
    /// At no point does it have multiple owners.
    #[test]
    fn property_ownership_transfer_atomicity_derived_state(
        derived_state in prop_oneof![
            Just(StateId::MaterialRegistryCache),
            Just(StateId::TerrainPreviewCache),
            Just(StateId::ViewportStatistics),
        ],
        owner in prop_oneof![
            Just(OwnerId::ProjectState),
            Just(OwnerId::WorkspaceState),
            Just(OwnerId::DiagnosticsState),
        ]
    ) {
        let mut system = create_test_system();

        // Before registration, derived state should have no owner
        let owner_before = system.get_owner(&derived_state);
        prop_assert!(owner_before.is_none(), "Derived state {:?} should have no owner before registration", derived_state);

        // Register derived state (ownership transfer)
        system.register_derived_state(derived_state.clone(), owner);

        // After registration, derived state should have exactly one owner
        let owner_after = system.get_owner(&derived_state);
        prop_assert!(owner_after.is_some(), "Derived state {:?} should have an owner after registration", derived_state);
        prop_assert_eq!(owner_after.unwrap(), owner, "Derived state {:?} should be owned by {:?}", derived_state, owner);

        // Validate ownership uniqueness (no multiple owners)
        let validation_result = system.validate_ownership_uniqueness();
        prop_assert!(validation_result.is_ok(), "Ownership validation should pass after derived state registration: {:?}", validation_result);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_ownership_transfer_world_open_close() {
        let mut system = create_test_system();

        // Initially no world state
        assert!(system.get_owner(&StateId::WorldState).is_none());

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

        // World state should have owner
        assert_eq!(
            system.get_owner(&StateId::WorldState),
            Some(OwnerId::WorldState)
        );
        assert!(system.validate_ownership_uniqueness().is_ok());

        // Close world
        system.clear_world_state();

        // World state should have no owner
        assert!(system.get_owner(&StateId::WorldState).is_none());
        assert!(system.validate_ownership_uniqueness().is_ok());
    }

    #[test]
    fn test_ownership_transfer_derived_state() {
        let mut system = create_test_system();

        // Initially no owner
        assert!(system.get_owner(&StateId::MaterialRegistryCache).is_none());

        // Register derived state
        system.register_derived_state(StateId::MaterialRegistryCache, OwnerId::ProjectState);

        // Should have owner
        assert_eq!(
            system.get_owner(&StateId::MaterialRegistryCache),
            Some(OwnerId::ProjectState)
        );
        assert!(system.validate_ownership_uniqueness().is_ok());
    }
}
