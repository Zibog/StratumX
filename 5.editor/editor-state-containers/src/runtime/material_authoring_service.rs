//! Material authoring service

use crate::runtime::state_types::ProjectState;
use crate::runtime::EventBus;
use crate::MaterialProfile;
use std::sync::{Arc, Mutex};

pub struct MaterialAuthoringService {
    project_state: Arc<Mutex<ProjectState>>,
    event_bus: Arc<dyn EventBus>,
}

impl MaterialAuthoringService {
    pub fn new(project_state: Arc<Mutex<ProjectState>>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            project_state,
            event_bus,
        }
    }

    pub fn create_material_profile(&mut self, profile: MaterialProfile) -> Result<(), String> {
        let mut project_state = self.project_state.lock().unwrap();
        project_state.increment_save_generation();
        project_state
            .project_owner
            .add_snapshot(format!("material:{}", profile.profile_name));
        self.event_bus.publish("material.profile.created");
        Ok(())
    }
}
