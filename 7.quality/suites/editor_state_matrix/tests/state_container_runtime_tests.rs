use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use stratumx_editor_state_containers::{
    DiagnosticsState, OwnerId, OwnershipViolation, ProjectIdentity, ProjectState,
    StateContainerSystem, StateId, ViolationType, WorkspaceIdentity, WorkspaceState, WorldIdentity,
    WorldState,
};
use uuid::Uuid;

fn create_system() -> StateContainerSystem {
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

    StateContainerSystem::new(project_state, workspace_state, diagnostics_state).unwrap()
}

#[test]
fn state_container_system_bootstraps_and_binds_world_scope() {
    let mut system = create_system();
    let world_id = WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );
    let world_state = Arc::new(Mutex::new(WorldState::new(
        world_id,
        "snapshot_123".to_string(),
    )));

    assert_eq!(
        system.get_owner(&StateId::ProjectState),
        Some(OwnerId::ProjectState)
    );
    assert_eq!(system.get_owner(&StateId::WorldState), None);

    system.set_world_state(world_state);
    assert_eq!(
        system.get_owner(&StateId::WorldState),
        Some(OwnerId::WorldState)
    );
    system.clear_world_state();
    assert_eq!(system.get_owner(&StateId::WorldState), None);
}

#[test]
fn state_container_system_formats_ownership_violations() {
    let violation = OwnershipViolation {
        state_id: StateId::ProjectIdentity,
        owners: vec![OwnerId::ProjectState, OwnerId::WorkspaceState],
        violation_type: ViolationType::MultipleOwners,
    };

    assert!(violation.to_user_message().contains("multiple owners"));
}
