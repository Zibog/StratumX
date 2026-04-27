//! Query Layer
//!
//! Non-authoritative read-only interface for state access.
//! The query layer can be destroyed and recreated without data loss.

use std::sync::Arc;

use crate::{
    DiagnosticMessage, EnvironmentState, FailureCode, PanelGeometry, PanelId, ProjectIdentity,
    Severity, StateContainerSystem, TerrainState, WorkspaceIdentity, WorldIdentity,
};

/// Query layer for read-only state access
///
/// The query layer is non-authoritative and can be destroyed/recreated
/// without data loss. All queries delegate to owner containers.
pub struct QueryLayer {
    state_system: Arc<StateContainerSystem>,
}

impl QueryLayer {
    /// Creates a new query layer
    pub fn new(state_system: Arc<StateContainerSystem>) -> Self {
        Self { state_system }
    }

    // ========================================================================
    // Project State Queries
    // ========================================================================

    /// Gets the project identity
    pub fn get_project_identity(&self) -> ProjectIdentity {
        let project_state = self.state_system.project_state.lock().unwrap();
        project_state.get_project_identity().clone()
    }

    /// Gets the workspace identity
    pub fn get_workspace_identity(&self) -> WorkspaceIdentity {
        let project_state = self.state_system.project_state.lock().unwrap();
        project_state.get_workspace_identity().clone()
    }

    /// Gets the current save generation
    pub fn get_save_generation(&self) -> u64 {
        let project_state = self.state_system.project_state.lock().unwrap();
        project_state.get_save_generation()
    }

    // ========================================================================
    // Workspace State Queries
    // ========================================================================

    /// Gets all open panels
    pub fn get_open_panels(&self) -> Vec<PanelId> {
        let workspace_state = self.state_system.workspace_state.lock().unwrap();
        workspace_state.get_open_panels().to_vec()
    }

    /// Gets the focused panel
    pub fn get_focused_panel(&self) -> Option<PanelId> {
        let workspace_state = self.state_system.workspace_state.lock().unwrap();
        workspace_state.focused_panel.clone()
    }

    /// Gets panel geometry
    pub fn get_panel_geometry(&self, panel_id: &PanelId) -> Option<PanelGeometry> {
        let workspace_state = self.state_system.workspace_state.lock().unwrap();
        workspace_state.get_panel_geometry(panel_id).cloned()
    }

    // ========================================================================
    // World State Queries
    // ========================================================================

    /// Gets the world identity (if a world is open)
    pub fn get_world_identity(&self) -> Option<WorldIdentity> {
        if let Some(world_state) = &self.state_system.world_state {
            let world = world_state.lock().unwrap();
            Some(world.get_world_identity().clone())
        } else {
            None
        }
    }

    /// Gets the terrain state (if available)
    pub fn get_terrain_state(&self) -> Option<TerrainState> {
        if let Some(world_state) = &self.state_system.world_state {
            let world = world_state.lock().unwrap();
            world.get_terrain_state().cloned()
        } else {
            None
        }
    }

    /// Gets the environment state (if available)
    pub fn get_environment_state(&self) -> Option<EnvironmentState> {
        if let Some(world_state) = &self.state_system.world_state {
            let world = world_state.lock().unwrap();
            world.get_environment_state().cloned()
        } else {
            None
        }
    }

    /// Gets the world snapshot reference (if a world is open)
    pub fn get_world_snapshot_ref(&self) -> Option<String> {
        if let Some(world_state) = &self.state_system.world_state {
            let world = world_state.lock().unwrap();
            Some(world.get_snapshot_ref().to_string())
        } else {
            None
        }
    }

    // ========================================================================
    // Diagnostics State Queries
    // ========================================================================

    /// Gets diagnostic messages by severity
    pub fn get_diagnostics_by_severity(&self, severity: Severity) -> Vec<DiagnosticMessage> {
        let diagnostics_state = self.state_system.diagnostics_state.lock().unwrap();
        diagnostics_state
            .get_messages_by_severity(severity)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Gets all diagnostic messages
    pub fn get_all_diagnostics(&self) -> Vec<DiagnosticMessage> {
        let diagnostics_state = self.state_system.diagnostics_state.lock().unwrap();
        diagnostics_state.messages.clone()
    }

    /// Gets all failure codes
    pub fn get_failure_codes(&self) -> Vec<FailureCode> {
        let diagnostics_state = self.state_system.diagnostics_state.lock().unwrap();
        diagnostics_state.failure_codes.clone()
    }

    /// Gets recovery action for a failure code
    pub fn get_recovery_action(&self, code: &FailureCode) -> Option<String> {
        let diagnostics_state = self.state_system.diagnostics_state.lock().unwrap();
        diagnostics_state.get_recovery_action(code).cloned()
    }
}


