//! World Session Service - Session Management
//!
//! Manages session lifecycle and state for world operations.

use crate::{EventBus, ProjectState, WorldState};
use std::sync::{Arc, Mutex};

/// World session service for managing world lifecycle
pub struct WorldSessionService {
    pub(super) world_state: Arc<Mutex<WorldState>>,
    pub(super) project_state: Arc<Mutex<ProjectState>>,
    pub(super) event_bus: Arc<dyn EventBus>,
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
}
