//! Query layer

use crate::*;
use std::sync::Arc;

pub struct QueryLayer {
    state_system: Arc<StateContainerSystem>,
}

impl Default for QueryLayer {
    fn default() -> Self {
        Self::new(Arc::new(StateContainerSystem::default()))
    }
}

impl QueryLayer {
    pub fn new(state_system: Arc<StateContainerSystem>) -> Self {
        Self { state_system }
    }

    pub fn get_open_panels(&self) -> Vec<PanelId> {
        self.state_system
            .workspace_state
            .lock()
            .unwrap()
            .get_open_panels()
    }

    pub fn get_focused_panel(&self) -> Option<PanelId> {
        self.state_system
            .workspace_state
            .lock()
            .unwrap()
            .focused_panel
            .clone()
    }

    pub fn get_project_identity(&self) -> ProjectIdentity {
        self.state_system
            .project_state
            .lock()
            .unwrap()
            .get_project_identity()
            .clone()
    }

    pub fn get_workspace_identity(&self) -> WorkspaceIdentity {
        self.state_system
            .project_state
            .lock()
            .unwrap()
            .get_workspace_identity()
            .clone()
    }

    pub fn get_save_generation(&self) -> u64 {
        self.state_system
            .project_state
            .lock()
            .unwrap()
            .get_save_generation()
    }

    pub fn get_world_identity(&self) -> Option<WorldIdentity> {
        self.state_system.world_state.as_ref().map(|world_state| {
            world_state
                .lock()
                .unwrap()
                .get_world_identity()
                .clone()
        })
    }

    pub fn get_world_snapshot_ref(&self) -> Option<String> {
        self.state_system.world_state.as_ref().map(|world_state| {
            world_state
                .lock()
                .unwrap()
                .get_snapshot_ref()
                .to_string()
        })
    }

    pub fn get_terrain_state(&self) -> Option<crate::owners::world_owner::TerrainState> {
        self.state_system.world_state.as_ref().and_then(|world_state| {
            world_state
                .lock()
                .unwrap()
                .get_terrain_state()
                .clone()
        })
    }

    pub fn get_environment_state(&self) -> Option<EnvironmentState> {
        self.state_system.world_state.as_ref().and_then(|world_state| {
            world_state
                .lock()
                .unwrap()
                .get_environment_state()
                .clone()
        })
    }

    pub fn get_all_diagnostics(&self) -> Vec<DiagnosticMessage> {
        self.state_system
            .diagnostics_state
            .lock()
            .unwrap()
            .messages
            .clone()
    }

    pub fn get_diagnostics_by_severity(&self, severity: Severity) -> Vec<DiagnosticMessage> {
        self.state_system
            .diagnostics_state
            .lock()
            .unwrap()
            .get_messages_by_severity(severity)
    }

    pub fn get_failure_codes(&self) -> Vec<FailureCode> {
        self.state_system
            .diagnostics_state
            .lock()
            .unwrap()
            .get_failure_codes()
    }
}
