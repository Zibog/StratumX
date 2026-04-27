//! Event Bus Trait for Service Communication
//!
//! Defines the interface that services use to emit events.
//! This trait is implemented by the EditorEventBus in desktop_app.
//!
//! **Requirements: 29.1, 29.5**

/// Minimal event representation for service communication
#[derive(Debug, Clone)]
pub enum ServiceEvent {
    WorldOpened { world_id: String },
    WorldClosed,
    WorldSaved,
    RuntimeStateChanged { mode: String },
    TerrainChanged { tool: String, position: [f32; 2] },
    EnvironmentChanged { scope: String, change_type: String },
    DiagnosticsUpdated { scope: String, change_type: String },
    ViewportChanged { scope: String, change_type: String },
}

/// Event bus interface for service communication
///
/// **Requirement 29.1, 29.5, 29.6, 29.7**
/// Services use this trait to emit events without direct coupling to the EventBus implementation.
pub trait EventBusInterface: Send + Sync {
    /// Emit event with circular prevention
    ///
    /// **Requirement 29.7**
    /// Returns false if circular emission is detected and blocked.
    fn emit_service_event(&self, event: ServiceEvent, source: &str) -> bool;
}
