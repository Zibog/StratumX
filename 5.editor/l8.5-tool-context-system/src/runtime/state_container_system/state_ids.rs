/// State identifier enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateId {
    // Owner container IDs
    ProjectState,
    WorkspaceState,
    WorldState,
    DiagnosticsState,

    // Substate IDs - ProjectState
    ProjectIdentity,
    WorkspaceIdentity,
    SaveGeneration,
    ContentSnapshots,

    // Substate IDs - WorkspaceState
    PanelLayout,
    DockingConfig,

    // Substate IDs - WorldState
    WorldIdentity,
    TerrainState,
    EnvironmentState,

    // Substate IDs - DiagnosticsState
    DiagnosticMessages,
    TraceLineage,

    // Derived state IDs
    MaterialRegistryCache,
    TerrainPreviewCache,
    ViewportStatistics,
    WorldTreeView,
    DiagnosticsSummary,
}

/// Owner identifier enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerId {
    ProjectState,
    WorkspaceState,
    WorldState,
    DiagnosticsState,
}
