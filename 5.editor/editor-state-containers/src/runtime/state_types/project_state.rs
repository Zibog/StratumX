//! Project state type

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub project_owner: owners::project_owner::ProjectOwner,
    pub save_generation: u64,
}

impl ProjectState {
    pub fn new(project_identity: ProjectIdentity, workspace_identity: WorkspaceIdentity) -> Self {
        Self {
            project_owner: owners::project_owner::ProjectOwner::new(
                project_identity,
                workspace_identity,
            ),
            save_generation: 0,
        }
    }

    pub fn increment_save_generation(&mut self) {
        self.save_generation = self.save_generation.saturating_add(1);
        self.project_owner.increment_save_generation();
    }

    pub fn get_save_generation(&self) -> u64 {
        self.save_generation
    }

    pub fn set_event_callback(&mut self, callback: owners::project_owner::EventCallback) {
        self.project_owner.set_event_callback(callback);
    }

    pub fn get_project_identity(&self) -> &owners::project_owner::ProjectIdentity {
        &self.project_owner.project_identity
    }

    pub fn get_workspace_identity(&self) -> &owners::project_owner::WorkspaceIdentity {
        &self.project_owner.workspace_identity
    }
}
