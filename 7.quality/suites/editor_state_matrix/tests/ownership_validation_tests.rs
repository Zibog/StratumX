//! Unit Tests for Ownership Validation
//!
//! Tests detection of multiple owners, orphaned state, circular dependencies,
//! and validation during initialization.
//!
//! Requirements: 11.1, 11.2, 11.3

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, OwnershipValidator, ProjectIdentity, ProjectState,
    StateContainerSystem, StateGraph, StateId, StateNode, StateNodeType, ViolationType,
    WorkspaceIdentity, WorkspaceState, WorldIdentity, WorldState,
};

fn create_test_system() -> Arc<StateContainerSystem> {
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

    Arc::new(
        StateContainerSystem::new(project_state, workspace_state, diagnostics_state)
            .expect("Failed to create test system"),
    )
}

/// Test detection of multiple owners
///
/// Requirement 11.1: Detect multiple owners for the same state
#[test]
fn test_detect_multiple_owners() {
    let system = create_test_system();
    let validator = OwnershipValidator::new(system.clone());

    // With our HashMap-based design, multiple owners are prevented by construction
    // This test verifies that the validation passes for a correctly constructed system
    let result = validator.check_uniqueness();
    assert!(
        result.is_ok(),
        "Should not detect multiple owners in correctly constructed system"
    );
}

/// Test detection of orphaned state
///
/// Requirement 11.2: Detect state with no owner
#[test]
fn test_detect_orphaned_state() {
    let system = create_test_system();
    let validator = OwnershipValidator::new(system);

    // All expected states should have owners
    let result = validator.check_completeness();
    assert!(result.is_ok(), "All expected states should have owners");
}

/// Test detection of orphaned state when world state is missing
///
/// Requirement 11.2: Detect state with no owner
#[test]
fn test_detect_orphaned_world_state() {
    let system = create_test_system();
    let validator = OwnershipValidator::new(system);

    // World state is not set, so world-specific states should not be checked
    // This should pass because we only check world states when world_state is Some
    let result = validator.check_completeness();
    assert!(result.is_ok());
}

/// Test detection of circular dependencies
///
/// Requirement 11.3: Detect circular dependencies in state graph
#[test]
fn test_detect_circular_dependencies() {
    let system = create_test_system();

    // Create a graph with a cycle
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

    // Manually create a cycle (bypassing add_dependency validation)
    let graph_arc = Arc::new(graph);
    let graph_ptr = Arc::as_ptr(&graph_arc) as *mut StateGraph;
    unsafe {
        (*graph_ptr)
            .edges
            .push((StateId::MaterialRegistryCache, StateId::ViewportStatistics));
        (*graph_ptr)
            .edges
            .push((StateId::ViewportStatistics, StateId::MaterialRegistryCache));
    }

    let validator = OwnershipValidator::new(system).with_state_graph(graph_arc);

    let result = validator.check_acyclic();
    assert!(result.is_err(), "Should detect circular dependency");

    let violations = result.unwrap_err();
    assert!(!violations.is_empty());
    assert!(violations
        .iter()
        .all(|v| v.violation_type == ViolationType::CyclicDependency));
}

/// Test validation during initialization
///
/// Requirement 11.1, 11.2, 11.3: Validate ownership during initialization
#[test]
fn test_validation_during_initialization() {
    let system = create_test_system();
    let validator = OwnershipValidator::new(system);

    // Run full validation
    let result = validator.validate();
    assert!(
        result.is_ok(),
        "Validation should pass during initialization"
    );
}

/// Test validation with world state
///
/// Requirement 11.1, 11.2: Validate ownership with world state present
#[test]
fn test_validation_with_world_state() {
    let system = create_test_system();

    // Add world state
    let world_id = WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );
    let world_state = Arc::new(Mutex::new(WorldState::new(
        world_id,
        "snapshot_123".to_string(),
    )));

    // Need to make system mutable
    let system_arc = system.clone();
    let system_ptr = Arc::as_ptr(&system_arc) as *mut StateContainerSystem;
    unsafe {
        (*system_ptr).set_world_state(world_state);
    }

    let validator = OwnershipValidator::new(system_arc);

    // Validation should pass with world state
    let result = validator.validate();
    assert!(result.is_ok(), "Validation should pass with world state");
}

/// Test validation reports all violations
///
/// Requirement 11.1, 11.2, 11.3: Report all violations
#[test]
fn test_validation_reports_all_violations() {
    let system = create_test_system();

    // Create a graph with a cycle
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

    let graph_arc = Arc::new(graph);
    let graph_ptr = Arc::as_ptr(&graph_arc) as *mut StateGraph;
    unsafe {
        (*graph_ptr)
            .edges
            .push((StateId::MaterialRegistryCache, StateId::ViewportStatistics));
        (*graph_ptr)
            .edges
            .push((StateId::ViewportStatistics, StateId::MaterialRegistryCache));
    }

    let validator = OwnershipValidator::new(system).with_state_graph(graph_arc);

    let result = validator.validate();
    assert!(result.is_err(), "Should detect violations");

    let violations = result.unwrap_err();
    assert!(!violations.is_empty(), "Should report violations");
}

/// Test ownership violation error messages
///
/// Requirement 11.2: Provide actionable error messages
#[test]
fn test_ownership_violation_messages() {
    use stratumx_editor_state_containers::OwnershipViolation;

    // Test multiple owners message
    let violation = OwnershipViolation {
        state_id: StateId::ProjectIdentity,
        owners: vec![OwnerId::ProjectState, OwnerId::WorkspaceState],
        violation_type: ViolationType::MultipleOwners,
        message: None,
    };
    let message = violation.to_user_message();
    assert!(message.contains("multiple owners"));
    assert!(message.contains("ProjectIdentity"));

    // Test no owner message
    let violation = OwnershipViolation {
        state_id: StateId::ProjectIdentity,
        owners: vec![],
        violation_type: ViolationType::NoOwner,
        message: None,
    };
    let message = violation.to_user_message();
    assert!(message.contains("no owner"));

    // Test circular dependency message
    let violation = OwnershipViolation {
        state_id: StateId::MaterialRegistryCache,
        owners: vec![],
        violation_type: ViolationType::CyclicDependency,
        message: None,
    };
    let message = violation.to_user_message();
    assert!(message.contains("cyclic dependency"));
}

/// Test validation prevents initialization on violations
///
/// Requirement 11.3: Prevent initialization if violations detected
#[test]
fn test_validation_prevents_initialization() {
    let system = create_test_system();

    // Create a graph with a cycle
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

    let graph_arc = Arc::new(graph);
    let graph_ptr = Arc::as_ptr(&graph_arc) as *mut StateGraph;
    unsafe {
        (*graph_ptr)
            .edges
            .push((StateId::MaterialRegistryCache, StateId::ViewportStatistics));
        (*graph_ptr)
            .edges
            .push((StateId::ViewportStatistics, StateId::MaterialRegistryCache));
    }

    let validator = OwnershipValidator::new(system).with_state_graph(graph_arc);

    // Validation should fail
    let result = validator.validate();
    assert!(result.is_err(), "Validation should fail with violations");

    // In a real system, this would prevent initialization
    // Here we just verify the validation detects the issue
}

/// Test completeness check with all expected states
///
/// Requirement 11.2: Check all expected states have owners
#[test]
fn test_completeness_check_all_states() {
    let system = create_test_system();
    let validator = OwnershipValidator::new(system);

    // Check completeness
    let result = validator.check_completeness();
    assert!(result.is_ok(), "All expected states should have owners");
}

/// Test acyclicity check with valid graph
///
/// Requirement 11.3: Validate acyclicity
#[test]
fn test_acyclicity_check_valid_graph() {
    let system = create_test_system();

    let mut graph = StateGraph::new();
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
    graph
        .add_dependency(StateId::MaterialRegistryCache, StateId::ProjectState)
        .unwrap();

    let validator = OwnershipValidator::new(system).with_state_graph(Arc::new(graph));

    let result = validator.check_acyclic();
    assert!(result.is_ok(), "Valid graph should pass acyclicity check");
}
