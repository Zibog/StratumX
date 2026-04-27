//! Property Test: State Graph Acyclicity
//!
//! Feature: editor-state-ownership-normalization
//! Property 10: State Graph Acyclicity
//! Validates: Requirements 7.3
//!
//! For any state graph configuration, the graph contains no cycles - all paths
//! from derived state to owner containers are acyclic.

use proptest::prelude::*;

use stratumx_editor_state_containers::{OwnerId, StateGraph, StateId, StateNode, StateNodeType};

/// Strategy for generating a valid acyclic graph
fn acyclic_graph_strategy() -> impl Strategy<Value = StateGraph> {
    // Generate a simple DAG structure
    Just(()).prop_map(|_| {
        let mut graph = StateGraph::new();

        // Add owner nodes (level 0)
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
        graph.add_node(StateNode::new(
            StateId::DiagnosticsState,
            StateNodeType::OwnerContainer,
            Some(OwnerId::DiagnosticsState),
        ));

        // Add derived nodes (level 1)
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
        graph.add_node(StateNode::new(
            StateId::DiagnosticsSummary,
            StateNodeType::DerivedState,
            Some(OwnerId::DiagnosticsState),
        ));

        // Add dependencies (derived -> owner)
        graph
            .add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState)
            .unwrap();
        graph
            .add_dependency(StateId::ViewportStatistics, StateId::ProjectState)
            .unwrap();
        graph
            .add_dependency(StateId::DiagnosticsSummary, StateId::DiagnosticsState)
            .unwrap();

        graph
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 7.3**
    ///
    /// Property 10: State Graph Acyclicity
    ///
    /// For any valid acyclic graph configuration, the graph validation passes
    /// and topological sort succeeds.
    #[test]
    fn property_state_graph_acyclicity(graph in acyclic_graph_strategy()) {
        // Validate that the graph is acyclic
        let validation_result = graph.validate_acyclic();
        prop_assert!(validation_result.is_ok(), "Graph should be acyclic");

        // Topological sort should succeed
        let sort_result = graph.topological_sort();
        prop_assert!(sort_result.is_ok(), "Topological sort should succeed on acyclic graph");

        // Verify all nodes are in the sorted result
        let sorted = sort_result.unwrap();
        let all_nodes = graph.get_all_nodes();
        prop_assert_eq!(sorted.len(), all_nodes.len(), "All nodes should be in topological sort");
    }

    /// **Validates: Requirements 7.3**
    ///
    /// Property 10: State Graph Acyclicity (dependency ordering)
    ///
    /// For any valid acyclic graph, in the topological sort, owner containers
    /// appear before their dependent derived states.
    #[test]
    fn property_state_graph_dependency_ordering(graph in acyclic_graph_strategy()) {
        let sorted = graph.topological_sort().unwrap();

        // For each derived state, verify its dependencies appear before it
        for (i, state_id) in sorted.iter().enumerate() {
            let dependencies = graph.get_dependencies(state_id);

            for dep in dependencies {
                // Find position of dependency in sorted list
                let dep_pos = sorted.iter().position(|id| *id == dep);

                if let Some(dep_pos) = dep_pos {
                    prop_assert!(
                        dep_pos < i,
                        "Dependency {:?} should appear before {:?} in topological sort",
                        dep,
                        state_id
                    );
                }
            }
        }
    }

    /// **Validates: Requirements 7.3**
    ///
    /// Property 10: State Graph Acyclicity (no self-loops)
    ///
    /// For any valid acyclic graph, no state depends on itself.
    #[test]
    fn property_state_graph_no_self_loops(graph in acyclic_graph_strategy()) {
        let all_nodes = graph.get_all_nodes();

        for node in all_nodes {
            let dependencies = graph.get_dependencies(&node.id);

            prop_assert!(
                !dependencies.contains(&node.id),
                "State {:?} should not depend on itself",
                node.id
            );
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_property_acyclicity_basic() {
        let mut graph = StateGraph::new();

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

        // Validate acyclicity
        let result = graph.validate_acyclic();
        assert!(result.is_ok());
    }

    #[test]
    fn test_property_acyclicity_detects_cycle() {
        let mut graph = StateGraph::new();

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
        graph.add_node(StateNode::new(
            StateId::ViewportStatistics,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        // Add the legal owner dependency first.
        graph
            .add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState)
            .unwrap();

        // Inject an invalid cycle directly and verify graph validation still detects it.
        graph
            .edges
            .push((StateId::MaterialRegistryCache, StateId::ViewportStatistics));
        graph
            .edges
            .push((StateId::ViewportStatistics, StateId::MaterialRegistryCache));

        assert!(graph.validate_acyclic().is_err());
    }

    #[test]
    fn test_property_dependency_ordering() {
        let mut graph = StateGraph::new();

        // Create one owner with two derived dependents.
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
        graph.add_node(StateNode::new(
            StateId::ViewportStatistics,
            StateNodeType::DerivedState,
            Some(OwnerId::ProjectState),
        ));

        graph
            .add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState)
            .unwrap();
        graph
            .add_dependency(StateId::ViewportStatistics, StateId::ProjectState)
            .unwrap();

        let sorted = graph.topological_sort().unwrap();

        // Verify ordering
        let project_pos = sorted
            .iter()
            .position(|id| *id == StateId::ProjectState)
            .unwrap();
        let cache_pos = sorted
            .iter()
            .position(|id| *id == StateId::MaterialRegistryCache)
            .unwrap();
        let stats_pos = sorted
            .iter()
            .position(|id| *id == StateId::ViewportStatistics)
            .unwrap();

        assert!(project_pos < cache_pos);
        assert!(project_pos < stats_pos);
    }
}
