//! Material Authoring Service
//!
//! Domain service for material creation and binding operations.
//! Abstraction Level: L4 (Authoring Tools)

use crate::{EditorEvent, EventBus, MaterialProfile, MaterialProfileId, ProjectState};
use std::sync::{Arc, Mutex};

/// Material authoring service for material management operations
pub struct MaterialAuthoringService {
    project_state: Arc<Mutex<ProjectState>>,
    event_bus: Arc<dyn EventBus>,
}

impl MaterialAuthoringService {
    /// Create a new material authoring service
    pub fn new(project_state: Arc<Mutex<ProjectState>>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            project_state,
            event_bus,
        }
    }

    /// Create a new material profile
    pub fn create_material_profile(
        &mut self,
        profile: MaterialProfile,
    ) -> Result<MaterialProfileId, String> {
        let profile_id = profile.profile_id.clone();

        // Store material profile in project state
        {
            let mut project = self.project_state.lock().unwrap();
            // Material profile storage is not modeled in ProjectState yet.
            // Mark the project dirty so upstream persistence and diagnostics can observe the mutation.
            project.save_generation += 1;
        }

        // Emit event
        self.event_bus.emit(EditorEvent::MaterialProfileCreated {
            profile_id: *profile_id.as_uuid(),
        });

        Ok(profile_id)
    }

    /// Bind a material to an entity
    pub fn bind_material(
        &mut self,
        entity_id: u32,
        profile_id: MaterialProfileId,
    ) -> Result<(), String> {
        // Update project state
        {
            let mut project = self.project_state.lock().unwrap();
            // Material binding storage is not modeled in ProjectState yet.
            // Mark the project dirty so upstream persistence and diagnostics can observe the mutation.
            project.save_generation += 1;
        }

        // Emit event
        self.event_bus.emit(EditorEvent::MaterialBound {
            entity_id,
            profile_id: *profile_id.as_uuid(),
        });

        Ok(())
    }
}
