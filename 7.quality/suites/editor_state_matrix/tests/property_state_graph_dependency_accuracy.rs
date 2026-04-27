//! Property Test: State Graph Dependency Accuracy
//!
//! Feature: editor-state-ownership-normalization
//! Property 11: State Graph Dependency Accuracy
//! Validates: Requirements 7.2
//!
//! For any derived state D in the state graph, the dependencies recorded in the graph
//! match the actual owner containers that D depends on for its computation.

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
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 7.2**
    ///
    /// Property 11: State Graph Dependency Accuracy
    ///
    /// When a derived state is registered with dependencies, the state graph
    /// accurately records those dependencies and they can be retrieved.
    #[test]
    fn property_state_graph_dependency_accuracy(
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
        prop_assert!(result.is_ok(), "Should be able to add dependency: {:?}", result);

        // Retrieve dependencies from the graph
        let dependencies = graph.get_dependencies(&derived_state);

        // Verify the dependency is accurately recorded
        prop_assert!(
            dependencies.contains(&owner_state),
            "State graph should record dependency from {:?} to {:?}, but got {:?}",
            derived_state,
            owner_state,
            dependencies
        );

        // Verify the dependency count is correct
        prop_assert_eq!(
            dependencies.len(), 1,
            "Expected exactly 1 dependency, but got {}",
            dependencies.len()
        );
    }

    /// **Validates: Requirements 7.2**
    ///
    /// Property 11: State Graph Dependency Accuracy (multiple dependencies)
    ///
    /// When a derived state depends on multiple owner containers, all dependencies
    /// are accurately recorded in the state graph.
    #[test]
    fn property_state_graph_dependency_accuracy_multiple(
        derived_state in derived_state_id(),
    ) {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add owner container nodes
        let owners = vec![
            StateId::ProjectState,
            StateId::WorkspaceState,
            StateId::DiagnosticsState,
        ];

        for owner in &owners {
            graph.add_node(StateNode::new(
                owner.clone(),
                StateNodeType::OwnerContainer,
                Some(match owner {
                    StateId::ProjectState => OwnerId::ProjectState,
                    StateId::WorkspaceState => OwnerId::WorkspaceState,
                    StateId::DiagnosticsState => OwnerId::DiagnosticsState,
                    _ => unreachable!(),
                }),
            ));
        }

        // Add derived state node
        graph.add_node(StateNode::new(
            derived_state.clone(),
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add dependencies from derived to all owners
        for owner in &owners {
            let result = graph.add_dependency(derived_state.clone(), owner.clone());
            prop_assert!(result.is_ok(), "Should be able to add dependency to {:?}: {:?}", owner, result);
        }

        // Retrieve dependencies from the graph
        let dependencies = graph.get_dependencies(&derived_state);

        // Verify all dependencies are accurately recorded
        prop_assert_eq!(
            dependencies.len(), owners.len(),
            "Expected {} dependencies, but got {}",
            owners.len(),
            dependencies.len()
        );

        for owner in &owners {
            prop_assert!(
                dependencies.contains(owner),
                "State graph should record dependency from {:?} to {:?}",
                derived_state,
                owner
            );
        }
    }

    /// **Validates: Requirements 7.2**
    ///
    /// Property 11: State Graph Dependency Accuracy (reverse lookup)
    ///
    /// When a derived state depends on an owner, the owner's dependents list
    /// accurately includes the derived state.
    #[test]
    fn property_state_graph_dependency_accuracy_reverse(
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
        prop_assert!(result.is_ok(), "Should be able to add dependency: {:?}", result);

        // Retrieve dependents of the owner (reverse lookup)
        let dependents = graph.get_dependents(&owner_state);

        // Verify the derived state is in the dependents list
        prop_assert!(
            dependents.contains(&derived_state),
            "Owner {:?} should have {:?} in its dependents list, but got {:?}",
            owner_state,
            derived_state,
            dependents
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_dependency_accuracy_single() {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add nodes
        graph.add_node(StateNode::new(
            StateId::ProjectState,
            StateNodeType::OwnerContainer,
            Some(OwnerId::ProjectState),
        ));

        graph.add_node(StateNode::new(
            StateId::MaterialRegistryCache,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add dependency
        graph
            .add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState)
            .unwrap();

        // Verify dependency is recorded
        let deps = graph.get_dependencies(&StateId::MaterialRegistryCache);
        assert_eq!(deps.len(), 1);
        assert!(deps.contains(&StateId::ProjectState));
    }

    #[test]
    fn test_dependency_accuracy_multiple() {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add owner nodes
        graph.add_node(StateNode::new(
            StateId::ProjectState,
            StateNodeType::OwnerContainer,
            Some(OwnerId::ProjectState),
        ));

        graph.add_node(StateNode::new(
            StateId::WorkspaceState,
            StateNodeType::OwnerContainer,
            Some(OwnerId::WorkspaceState),
        ));

        // Add derived node
        graph.add_node(StateNode::new(
            StateId::ViewportStatistics,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add dependencies
        graph
            .add_dependency(StateId::ViewportStatistics, StateId::ProjectState)
            .unwrap();
        graph
            .add_dependency(StateId::ViewportStatistics, StateId::WorkspaceState)
            .unwrap();

        // Verify dependencies are recorded
        let deps = graph.get_dependencies(&StateId::ViewportStatistics);
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&StateId::ProjectState));
        assert!(deps.contains(&StateId::WorkspaceState));
    }

    #[test]
    fn test_dependency_accuracy_reverse_lookup() {
        let mut system = create_test_system();
        let graph = system.get_state_graph_mut();

        // Add nodes
        graph.add_node(StateNode::new(
            StateId::ProjectState,
            StateNodeType::OwnerContainer,
            Some(OwnerId::ProjectState),
        ));

        graph.add_node(StateNode::new(
            StateId::MaterialRegistryCache,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add dependency
        graph
            .add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState)
            .unwrap();

        // Verify reverse lookup
        let dependents = graph.get_dependents(&StateId::ProjectState);
        assert_eq!(dependents.len(), 1);
        assert!(dependents.contains(&StateId::MaterialRegistryCache));
    }
}
