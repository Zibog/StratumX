//! Editor Services Container
//!
//! This module provides the EditorServices struct that contains all 7 domain services
//! and validates service dependency constraints during initialization.

use crate::{
    AudioAuthoringService, DiagnosticsService, DiagnosticsState, EnvironmentAuthoringService,
    EventBus, MaterialAuthoringService, ProjectState, RuntimeModeService, TerrainAuthoringService,
    WorkspaceState, WorldSessionService, WorldState,
};
use std::sync::{Arc, Mutex};

/// Container for all domain services in the editor
pub struct EditorServices {
    pub world_session: WorldSessionService,
    pub terrain_authoring: TerrainAuthoringService,
    pub material_authoring: MaterialAuthoringService,
    pub audio_authoring: AudioAuthoringService,
    pub environment_authoring: EnvironmentAuthoringService,
    pub runtime_mode: RuntimeModeService,
    pub diagnostics: DiagnosticsService,
}

impl EditorServices {
    /// Create a new EditorServices container with all domain services
    ///
    /// # Arguments
    ///
    /// * `project_state` - Shared reference to ProjectState
    /// * `workspace_state` - Shared reference to WorkspaceState
    /// * `world_state` - Optional shared reference to WorldState
    /// * `diagnostics_state` - Shared reference to DiagnosticsState
    /// * `event_bus` - Shared event bus for service communication
    ///
    /// # Returns
    ///
    /// Result containing EditorServices or error message if validation fails
    pub fn new(
        project_state: Arc<Mutex<ProjectState>>,
        workspace_state: Arc<Mutex<WorkspaceState>>,
        world_state: Option<Arc<Mutex<WorldState>>>,
        diagnostics_state: Arc<Mutex<DiagnosticsState>>,
        event_bus: Arc<dyn EventBus>,
    ) -> Result<Self, String> {
        // Validate service dependency constraints
        Self::validate_dependencies(
            &project_state,
            &workspace_state,
            &world_state,
            &diagnostics_state,
        )?;

        // Create a default world state if none provided (services can handle None)
        let default_world_state = world_state.unwrap_or_else(|| {
            Arc::new(Mutex::new(WorldState::new(
                crate::WorldIdentity::new(
                    uuid::Uuid::new_v4(),
                    "default-world".to_string(),
                    std::path::PathBuf::from("default-world"),
                ),
                "default-snapshot".to_string(),
            )))
        });

        // Initialize all services with proper dependencies
        let world_session = WorldSessionService::new(
            default_world_state.clone(),
            project_state.clone(),
            event_bus.clone(),
        );

        let terrain_authoring =
            TerrainAuthoringService::new(default_world_state.clone(), event_bus.clone());

        let material_authoring =
            MaterialAuthoringService::new(project_state.clone(), event_bus.clone());

        let audio_authoring =
            AudioAuthoringService::new(default_world_state.clone(), event_bus.clone());

        let environment_authoring =
            EnvironmentAuthoringService::new(default_world_state.clone(), event_bus.clone());

        let runtime_mode = RuntimeModeService::new(
            default_world_state.clone(),
            diagnostics_state.clone(),
            event_bus.clone(),
        );

        let diagnostics = DiagnosticsService::new(diagnostics_state.clone(), event_bus.clone());

        Ok(Self {
            world_session,
            terrain_authoring,
            material_authoring,
            audio_authoring,
            environment_authoring,
            runtime_mode,
            diagnostics,
        })
    }

    /// Validate service dependency constraints
    ///
    /// Ensures that all required state containers are available and properly configured
    fn validate_dependencies(
        project_state: &Arc<Mutex<ProjectState>>,
        _workspace_state: &Arc<Mutex<WorkspaceState>>,
        world_state: &Option<Arc<Mutex<WorldState>>>,
        diagnostics_state: &Arc<Mutex<DiagnosticsState>>,
    ) -> Result<(), String> {
        // Validate project state is accessible
        drop(
            project_state
                .lock()
                .map_err(|e| format!("Failed to lock ProjectState: {}", e))?,
        );

        // Validate diagnostics state is accessible
        drop(
            diagnostics_state
                .lock()
                .map_err(|e| format!("Failed to lock DiagnosticsState: {}", e))?,
        );

        // World state is optional but if present, must be accessible
        if let Some(ws) = world_state {
            drop(
                ws.lock()
                    .map_err(|e| format!("Failed to lock WorldState: {}", e))?,
            );
        }

        Ok(())
    }
}

