/// Event emitted when world state changes.
#[derive(Debug, Clone, PartialEq)]
pub enum WorldStateEvent {
    /// World snapshot reference updated.
    SnapshotRefUpdated { new_ref: String },

    /// Terrain state updated.
    TerrainStateUpdated,

    /// Environment state updated.
    EnvironmentStateUpdated,

    /// Diagnostic added.
    DiagnosticAdded { message: String },

    /// Diagnostics cleared.
    DiagnosticsCleared,

    /// World identity updated.
    WorldIdentityUpdated,

    /// Runtime mode updated.
    RuntimeModeUpdated,
}

/// Callback type for state change events.
pub type WorldStateEventCallback = Box<dyn Fn(WorldStateEvent) + Send + Sync>;
