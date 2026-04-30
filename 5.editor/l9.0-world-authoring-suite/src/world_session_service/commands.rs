//! World Session Service - Command Handlers
//!
//! Handles world lifecycle commands: open, close, and save operations.

use super::session::WorldSessionService;
use crate::{EditorEvent, WorldIdentity};
use std::path::PathBuf;
use uuid::Uuid;

impl WorldSessionService {
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
}
