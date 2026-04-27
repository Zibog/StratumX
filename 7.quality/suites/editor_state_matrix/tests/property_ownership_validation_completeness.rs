//! Property Test: Ownership Validation Completeness
//!
//! Feature: editor-state-ownership-normalization
//! Property 16: Ownership Validation Completeness
//! Validates: Requirements 11.1
//!
//! For any state item in the system, the ownership validation function correctly
//! identifies whether the item has zero owners, one owner, or multiple owners.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, ProjectIdentity, ProjectState, StateContainerSystem, StateId,
    WorkspaceIdentity, WorkspaceState,
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

/// Strategy for generating state IDs that should have owners
fn state_with_owner() -> impl Strategy<Value = StateId> {
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

/// Strategy for generating state IDs that should not have owners (world state when no world is open)
fn state_without_owner() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::WorldState),
        Just(StateId::WorldIdentity),
        Just(StateId::TerrainState),
        Just(StateId::EnvironmentState),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 11.1**
    ///
    /// Property 16: Ownership Validation Completeness (one owner)
    ///
    /// For any state that should have an owner, the validation function correctly
    /// identifies that it has exactly one owner.
    #[test]
    fn property_ownership_validation_completeness_one_owner(state_id in state_with_owner()) {
        let system = create_test_system();

        // Validate ownership
        let validation_result = system.validate_ownership_uniqueness();
        prop_assert!(validation_result.is_ok(), "Validation should pass for state with one owner: {:?}", validation_result);

        // Verify the state has exactly one owner
        let owner = system.get_owner(&state_id);
        prop_assert!(owner.is_some(), "State {:?} should have an owner", state_id);

        // Verify the owner is one of the 4 defined containers
        let owner = owner.unwrap();
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

    /// **Validates: Requirements 11.1**
    ///
    /// Property 16: Ownership Validation Completeness (zero owners)
    ///
    /// For any state that should not have an owner (e.g., world state when no world is open),
    /// the validation function correctly identifies that it has zero owners.
    #[test]
    fn property_ownership_validation_completeness_zero_owners(state_id in state_without_owner()) {
        let system = create_test_system();

        // Verify the state has no owner
        let owner = system.get_owner(&state_id);
        prop_assert!(owner.is_none(), "State {:?} should not have an owner when world is not open", state_id);

        // Validation should still pass (zero owners is valid for world state when no world is open)
        let validation_result = system.validate_ownership_uniqueness();
        prop_assert!(validation_result.is_ok(), "Validation should pass even with zero owners for world state: {:?}", validation_result);
    }

    /// **Validates: Requirements 11.1**
    ///
    /// Property 16: Ownership Validation Completeness (consistency)
    ///
    /// Running validation multiple times produces the same result.
    #[test]
    fn property_ownership_validation_completeness_consistency(state_id in state_with_owner()) {
        let system = create_test_system();

        // Run validation multiple times
        let result1 = system.validate_ownership_uniqueness();
        let result2 = system.validate_ownership_uniqueness();
        let result3 = system.validate_ownership_uniqueness();

        // All results should be the same
        prop_assert_eq!(result1.is_ok(), result2.is_ok(), "Validation results should be consistent");
        prop_assert_eq!(result2.is_ok(), result3.is_ok(), "Validation results should be consistent");

        // Owner lookup should also be consistent
        let owner1 = system.get_owner(&state_id);
        let owner2 = system.get_owner(&state_id);
        let owner3 = system.get_owner(&state_id);

        prop_assert_eq!(owner1, owner2, "Owner lookup should be consistent");
        prop_assert_eq!(owner2, owner3, "Owner lookup should be consistent");
    }

    /// **Validates: Requirements 11.1**
    ///
    /// Property 16: Ownership Validation Completeness (derived state registration)
    ///
    /// When derived state is registered, validation correctly identifies it has one owner.
    #[test]
    fn property_ownership_validation_completeness_derived_state(
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

        // Initially, derived state has no owner
        let owner_before = system.get_owner(&derived_state);
        prop_assert!(owner_before.is_none(), "Derived state should have no owner before registration");

        // Register derived state
        system.register_derived_state(derived_state.clone(), owner);

        // After registration, validation should pass
        let validation_result = system.validate_ownership_uniqueness();
        prop_assert!(validation_result.is_ok(), "Validation should pass after derived state registration: {:?}", validation_result);

        // Verify derived state has exactly one owner
        let owner_after = system.get_owner(&derived_state);
        prop_assert!(owner_after.is_some(), "Derived state should have an owner after registration");
        prop_assert_eq!(owner_after.unwrap(), owner, "Derived state should have the correct owner");
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_validation_completeness_one_owner() {
        let system = create_test_system();

        // Validation should pass
        let result = system.validate_ownership_uniqueness();
        assert!(result.is_ok());

        // All default states should have owners
        assert!(system.get_owner(&StateId::ProjectState).is_some());
        assert!(system.get_owner(&StateId::WorkspaceState).is_some());
        assert!(system.get_owner(&StateId::DiagnosticsState).is_some());
        assert!(system.get_owner(&StateId::ProjectIdentity).is_some());
    }

    #[test]
    fn test_validation_completeness_zero_owners() {
        let system = create_test_system();

        // World state should have no owner when no world is open
        assert!(system.get_owner(&StateId::WorldState).is_none());
        assert!(system.get_owner(&StateId::TerrainState).is_none());

        // Validation should still pass
        let result = system.validate_ownership_uniqueness();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_completeness_consistency() {
        let system = create_test_system();

        // Run validation multiple times
        let result1 = system.validate_ownership_uniqueness();
        let result2 = system.validate_ownership_uniqueness();
        let result3 = system.validate_ownership_uniqueness();

        // All should pass
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }

    #[test]
    fn test_validation_completeness_derived_state() {
        let mut system = create_test_system();

        // Register derived state
        system.register_derived_state(StateId::MaterialRegistryCache, OwnerId::ProjectState);

        // Validation should pass
        let result = system.validate_ownership_uniqueness();
        assert!(result.is_ok());

        // Derived state should have owner
        assert_eq!(
            system.get_owner(&StateId::MaterialRegistryCache),
            Some(OwnerId::ProjectState)
        );
    }
}
