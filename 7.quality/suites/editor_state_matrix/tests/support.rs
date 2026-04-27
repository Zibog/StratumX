//! Test support utilities for editor_state_matrix tests
//!
//! Provides builder functions and test fixtures for creating
//! StateContainerSystem instances and related test objects.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    DiagnosticsState, ProjectIdentity, ProjectState, StateContainerSystem, WorkspaceIdentity,
    WorkspaceState,
};

/// Create a default test system with generated UUIDs
///
/// Returns a StateContainerSystem with default project, workspace, and diagnostics state.
pub fn create_test_system() -> StateContainerSystem {
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

/// Create test project and workspace identities
///
/// Returns a tuple of (ProjectIdentity, WorkspaceIdentity) for use in tests.
pub fn create_test_identities() -> (ProjectIdentity, WorkspaceIdentity) {
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
    (project_id, workspace_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_system() {
        let _system = create_test_system();
        // System should be created without panic
    }

    #[test]
    fn test_create_test_identities() {
        let (project_id, workspace_id) = create_test_identities();
        // Just verify they were created
        assert_eq!(project_id, project_id);
        assert_eq!(workspace_id, workspace_id);
    }
}
