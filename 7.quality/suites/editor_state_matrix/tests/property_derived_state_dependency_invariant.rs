//! Property Test: Derived State Dependency Invariant
//!
//! Feature: editor-state-ownership-normalization
//! Property 4: Derived State Dependency Invariant
//! Validates: Requirements 3.1
//!
//! For any derived state D, all dependencies of D are owner container state,
//! never other derived state.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, ProjectIdentity, ProjectState, StateContainerSystem, StateId,
    StateNode, StateNodeType, WorkspaceIdentity, WorkspaceState,
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

/// Strategy for generating derived state IDs
fn derived_state_id() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::MaterialRegistryCache),
        Just(StateId::TerrainPreviewCache),
        Just(StateId::ViewportStatistics),
        Just(StateId::WorldTreeView),
        Just(StateId::DiagnosticsSummary),
    ]
}

/// Strategy for generating owner container state IDs
fn owner_container_id() -> impl Strategy<Value = StateId> {
    prop_oneof![
        Just(StateId::ProjectState),
        Just(StateId::WorkspaceState),
        Just(StateId::DiagnosticsState),
        Just(StateId::WorldState),
    ]
}

/// Check if a state ID is an owner container
fn is_owner_container(state_id: &StateId) -> bool {
    matches!(
        state_id,
        StateId::ProjectState
            | StateId::WorkspaceState
            | StateId::WorldState
            | StateId::DiagnosticsState
    )
}

/// Check if a state ID is a substate (owned by a container)
fn is_substate(state_id: &StateId) -> bool {
    matches!(
        state_id,
        StateId::ProjectIdentity
            | StateId::WorkspaceIdentity
            | StateId::SaveGeneration
            | StateId::ContentSnapshots
            | StateId::PanelLayout
            | StateId::DockingConfig
            | StateId::WorldIdentity
            | StateId::TerrainState
            | StateId::EnvironmentState
            | StateId::DiagnosticMessages
            | StateId::TraceLineage
    )
}

/// Check if a state ID is derived state
fn is_derived_state(state_id: &StateId) -> bool {
    matches!(
        state_id,
        StateId::MaterialRegistryCache
            | StateId::TerrainPreviewCache
            | StateId::ViewportStatistics
            | StateId::WorldTreeView
            | StateId::DiagnosticsSummary
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 3.1**
    ///
    /// Property 4: Derived State Dependency Invariant
    ///
    /// For any derived state in the state graph, all its dependencies are owner container
    /// state or substates, never other derived state.
    #[test]
    fn property_derived_state_dependency_invariant(
        derived_state in derived_state_id(),
        owner_state in owner_container_id()
    ) {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add owner container node
        graph.add_node(StateNode::new(
            owner_state.clone(),
            StateNodeType::OwnerContainer,
            Some(match owner_state {
                StateId::ProjectState => OwnerId::ProjectState,
                StateId::WorkspaceState => OwnerId::WorkspaceState,
                StateId::DiagnosticsState => OwnerId::DiagnosticsState,
                StateId::WorldState => OwnerId::WorldState,
                _ => unreachable!(),
            }),
        ));

        // Add derived state node
        graph.add_node(StateNode::new(
            derived_state.clone(),
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add dependency from derived to owner
        let result = graph.add_dependency(derived_state.clone(), owner_state.clone());
        prop_assert!(result.is_ok(), "Should be able to add dependency from derived to owner: {:?}", result);

        // Verify all dependencies of the derived state are owner containers or substates
        let dependencies = graph.get_dependencies(&derived_state);
        for dep in dependencies {
            prop_assert!(
                is_owner_container(&dep) || is_substate(&dep),
                "Derived state {:?} depends on {:?} which is neither an owner container nor a substate",
                derived_state,
                dep
            );

            // Ensure it's not another derived state
            prop_assert!(
                !is_derived_state(&dep),
                "Derived state {:?} depends on another derived state {:?}, violating the invariant",
                derived_state,
                dep
            );
        }
    }

    /// **Validates: Requirements 3.1**
    ///
    /// Property 4: Derived State Dependency Invariant (rejection test)
    ///
    /// Attempting to create a dependency from one derived state to another derived state
    /// should be prevented or detected as a violation.
    #[test]
    fn property_derived_state_dependency_invariant_rejection(
        derived_state1 in derived_state_id(),
        derived_state2 in derived_state_id()
    ) {
        // Skip if both are the same (self-dependency is a different issue)
        prop_assume!(derived_state1 != derived_state2);

        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add both derived state nodes
        graph.add_node(StateNode::new(
            derived_state1.clone(),
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        graph.add_node(StateNode::new(
            derived_state2.clone(),
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Try to add dependency from one derived state to another
        // This should either fail or be detectable as a violation
        let result = graph.add_dependency(derived_state1.clone(), derived_state2.clone());

        // This should fail - derived state cannot depend on another derived state
        prop_assert!(
            result.is_err(),
            "Should not be able to add dependency from derived state {:?} to another derived state {:?}",
            derived_state1,
            derived_state2
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_derived_state_depends_on_owner() {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add owner node
        graph.add_node(StateNode::new(
            StateId::ProjectState,
            StateNodeType::OwnerContainer,
            Some(OwnerId::ProjectState),
        ));

        // Add derived node
        graph.add_node(StateNode::new(
            StateId::MaterialRegistryCache,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add dependency
        let result = graph.add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState);
        assert!(result.is_ok());

        // Verify dependency
        let deps = graph.get_dependencies(&StateId::MaterialRegistryCache);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], StateId::ProjectState);
        assert!(is_owner_container(&deps[0]));
    }

    #[test]
    fn test_derived_state_cannot_depend_on_derived() {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add two derived nodes
        graph.add_node(StateNode::new(
            StateId::MaterialRegistryCache,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        graph.add_node(StateNode::new(
            StateId::ViewportStatistics,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Try to add dependency from one derived to another
        let result =
            graph.add_dependency(StateId::ViewportStatistics, StateId::MaterialRegistryCache);

        // This should fail - derived state cannot depend on another derived state
        assert!(
            result.is_err(),
            "Should not be able to add dependency from derived to derived"
        );
        assert!(result
            .unwrap_err()
            .contains("cannot depend on another derived state"));
    }

    #[test]
    fn test_is_owner_container() {
        assert!(is_owner_container(&StateId::ProjectState));
        assert!(is_owner_container(&StateId::WorkspaceState));
        assert!(is_owner_container(&StateId::DiagnosticsState));
        assert!(is_owner_container(&StateId::WorldState));
        assert!(!is_owner_container(&StateId::MaterialRegistryCache));
    }

    #[test]
    fn test_is_derived_state() {
        assert!(is_derived_state(&StateId::MaterialRegistryCache));
        assert!(is_derived_state(&StateId::TerrainPreviewCache));
        assert!(is_derived_state(&StateId::ViewportStatistics));
        assert!(!is_derived_state(&StateId::ProjectState));
    }
}
