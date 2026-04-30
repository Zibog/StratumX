use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, OwnershipValidator, ProjectIdentity, ProjectState,
    StateContainerSystem, StateGraph, StateId, StateNode, StateNodeType, ViolationType,
    WorkspaceIdentity, WorkspaceState,
};
use uuid::Uuid;

fn create_system() -> Arc<StateContainerSystem> {
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

    Arc::new(StateContainerSystem::new(project_state, workspace_state, diagnostics_state).unwrap())
}

#[test]
fn ownership_validator_accepts_sound_system_without_graph() {
    let validator = OwnershipValidator::new(create_system());
    assert!(validator.validate().is_ok());
}

#[test]
fn ownership_validator_reports_cycles_in_dependency_graph() {
    let mut graph = StateGraph::new();
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
        .edges
        .push((StateId::MaterialRegistryCache, StateId::ViewportStatistics));
    graph
        .edges
        .push((StateId::ViewportStatistics, StateId::MaterialRegistryCache));

    let validator = OwnershipValidator::new(create_system()).with_state_graph(Arc::new(graph));
    let violations = validator.check_acyclic().unwrap_err();

    assert!(violations
        .iter()
        .all(|violation| violation.violation_type == ViolationType::CyclicDependency));
}
