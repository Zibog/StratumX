//! Editor Coordinator - Business Logic Delegation
//!
//! Coordinates domain services and manages editor lifecycle.
//! EditorHost delegates all business logic to this coordinator.

use super::editor_services::EditorServices;
use editor_dto_law::Posture;

/// Severity level for diagnostic messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// Editor coordinator - handles business logic delegation
pub struct EditorCoordinator;

impl EditorCoordinator {
    pub fn new() -> Self {
        Self
    }

    /// Initialize all domain services
    pub fn initialize_services(&mut self, services: &mut EditorServices) -> Result<(), String> {
        services.world_lifecycle.initialize()?;
        services.runtime_control.initialize()?;
        services.terrain.initialize()?;
        services.environment.initialize()?;
        services.diagnostics.initialize()?;
        services.viewport.initialize()?;
        Ok(())
    }

    /// Shutdown all domain services
    pub fn shutdown_services(&mut self, services: &mut EditorServices) -> Result<(), String> {
        services.viewport.shutdown()?;
        services.diagnostics.shutdown()?;
        services.environment.shutdown()?;
        services.terrain.shutdown()?;
        services.runtime_control.shutdown()?;
        services.world_lifecycle.shutdown()?;
        Ok(())
    }

    pub fn posture_to_severity(posture: &Posture) -> Severity {
        match posture {
            Posture::Healthy => Severity::Info,
            Posture::Degraded => Severity::Warning,
            Posture::Failed => Severity::Error,
            Posture::Unknown => Severity::Warning,
        }
    }
}

impl Default for EditorCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
