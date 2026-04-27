//! World Session Service
//!
//! Domain service for world lifecycle management including world open/close/save operations.
//! Abstraction Level: L3 (World Operations)

use crate::{EditorEvent, EventBus, ProjectState, WorldIdentity, WorldState};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// World session service for managing world lifecycle
pub struct WorldSessionService {
    world_state: Arc<Mutex<WorldState>>,
    project_state: Arc<Mutex<ProjectState>>,
    event_bus: Arc<dyn EventBus>,
}

/// Data transfer object for world summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSummaryDto {
    pub world_identity: Option<WorldIdentity>,
    pub world_snapshot_ref: Option<String>,
    pub has_terrain: bool,
    pub has_environment: bool,
    pub diagnostic_count: usize,
}

impl WorldSessionService {
    /// Create a new world session service
    pub fn new(
        world_state: Arc<Mutex<WorldState>>,
        project_state: Arc<Mutex<ProjectState>>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            world_state,
            project_state,
            event_bus,
        }
    }

    /// Open a world by identity
    pub fn open_world(&mut self, world_identity: WorldIdentity) -> Result<(), String> {
        let world_id = world_identity.world_id;

        // Update world state
        {
            let mut world = self.world_state.lock().unwrap();
            world.world_identity = world_identity.clone();
            world.world_snapshot_ref = format!("snapshot_{}", world_id);
        }

        // Emit event
        self.event_bus.emit(EditorEvent::WorldOpened { world_id });

        Ok(())
    }

    /// Close the currently open world
    pub fn close_world(&mut self) -> Result<(), String> {
        // Clear world state by creating a new empty world identity
        {
            let mut world = self.world_state.lock().unwrap();
            // Create a nil UUID world identity to represent "no world"
            world.world_identity = WorldIdentity::new(Uuid::nil(), String::new(), PathBuf::new());
            world.world_snapshot_ref = String::new();
            world.terrain_state = None;
            world.environment_state = None;
            world.world_diagnostics.clear();
        }

        // Emit event
        self.event_bus.emit(EditorEvent::WorldClosed);

        Ok(())
    }

    /// Save the currently open world
    pub fn save_world(&mut self) -> Result<(), String> {
        // Increment save generation in project state
        let generation = {
            let mut project = self.project_state.lock().unwrap();
            project.save_generation += 1;
            project.save_generation
        };

        // Emit event
        self.event_bus.emit(EditorEvent::WorldSaved { generation });

        Ok(())
    }

    /// Get a summary of the current world state
    pub fn get_world_summary(&self) -> WorldSummaryDto {
        let world = self.world_state.lock().unwrap();

        WorldSummaryDto {
            world_identity: if world.world_identity.world_id.is_nil() {
                None
            } else {
                Some(world.world_identity.clone())
            },
            world_snapshot_ref: if world.world_snapshot_ref.is_empty() {
                None
            } else {
                Some(world.world_snapshot_ref.clone())
            },
            has_terrain: world.terrain_state.is_some(),
            has_environment: world.environment_state.is_some(),
            diagnostic_count: world.world_diagnostics.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BasicEventBus, ProjectIdentity, WorkspaceIdentity};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use uuid::Uuid;

    fn create_test_service() -> WorldSessionService {
        let world_identity = WorldIdentity::new(Uuid::nil(), String::new(), PathBuf::new());
        let world_state = Arc::new(Mutex::new(WorldState::new(world_identity, String::new())));

        let project_identity = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_identity = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );
        let project_state = Arc::new(Mutex::new(ProjectState::new(
            project_identity,
            workspace_identity,
        )));
        let event_bus = Arc::new(BasicEventBus::new());

        WorldSessionService::new(world_state, project_state, event_bus)
    }

    #[test]
    fn test_open_world() {
        let mut service = create_test_service();
        let world_id = Uuid::new_v4();
        let world_identity = WorldIdentity::new(
            world_id,
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let result = service.open_world(world_identity.clone());
        assert!(result.is_ok());

        let summary = service.get_world_summary();
        assert_eq!(summary.world_identity.unwrap().world_id, world_id);
    }

    #[test]
    fn test_close_world() {
        let mut service = create_test_service();
        let world_identity = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        service.open_world(world_identity).unwrap();
        let result = service.close_world();
        assert!(result.is_ok());

        let summary = service.get_world_summary();
        assert!(summary.world_identity.is_none());
    }

    #[test]
    fn test_save_world() {
        let mut service = create_test_service();

        let result = service.save_world();
        assert!(result.is_ok());

        let project = service.project_state.lock().unwrap();
        assert_eq!(project.save_generation, 1);
    }

    #[test]
    fn test_event_emission() {
        let world_identity = WorldIdentity::new(Uuid::nil(), String::new(), PathBuf::new());
        let world_state = Arc::new(Mutex::new(WorldState::new(world_identity, String::new())));

        let project_identity = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_identity = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );
        let project_state = Arc::new(Mutex::new(ProjectState::new(
            project_identity,
            workspace_identity,
        )));
        let event_bus = Arc::new(BasicEventBus::new());

        let event_count = Arc::new(AtomicUsize::new(0));
        let event_count_clone = event_count.clone();

        event_bus.subscribe(Box::new(move |_event| {
            event_count_clone.fetch_add(1, Ordering::SeqCst);
        }));

        let mut service = WorldSessionService::new(world_state, project_state, event_bus);

        let world_identity = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        service.open_world(world_identity).unwrap();
        service.save_world().unwrap();
        service.close_world().unwrap();

        assert_eq!(event_count.load(Ordering::SeqCst), 3);
    }
}
