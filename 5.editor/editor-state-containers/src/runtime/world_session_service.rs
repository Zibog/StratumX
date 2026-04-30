//! World session service

use crate::runtime::state_types::{ProjectState, WorldState};
use crate::runtime::EventBus;
use crate::WorldIdentity;
use std::sync::{Arc, Mutex};

pub struct WorldSessionService {
    world_state: Arc<Mutex<WorldState>>,
    project_state: Arc<Mutex<ProjectState>>,
    event_bus: Arc<dyn EventBus>,
}

impl WorldSessionService {
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

    pub fn open_world(&mut self, world_id: WorldIdentity) -> Result<(), String> {
        {
            let mut world_state = self.world_state.lock().unwrap();
            world_state.identity = world_id.clone();
            world_state.world_identity = world_id.clone();
            world_state.snapshot_ref = format!("snapshot:{}", world_id.world_id);
            world_state.terrain_state = None;
            world_state.environment_state = None;
            world_state.runtime_mode = Default::default();
            world_state.audio_source_count = 0;
            world_state.diagnostics.clear();
        }

        self.project_state.lock().unwrap().increment_save_generation();
        self.event_bus.publish("world.session.opened");
        Ok(())
    }
}
