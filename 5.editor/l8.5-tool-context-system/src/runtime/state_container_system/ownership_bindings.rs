use super::{OwnerId, StateId};

pub(crate) const BASE_OWNERSHIP_BINDINGS: &[(StateId, OwnerId)] = &[
    (StateId::ProjectState, OwnerId::ProjectState),
    (StateId::WorkspaceState, OwnerId::WorkspaceState),
    (StateId::DiagnosticsState, OwnerId::DiagnosticsState),
    (StateId::ProjectIdentity, OwnerId::ProjectState),
    (StateId::WorkspaceIdentity, OwnerId::ProjectState),
    (StateId::SaveGeneration, OwnerId::ProjectState),
    (StateId::ContentSnapshots, OwnerId::ProjectState),
    (StateId::PanelLayout, OwnerId::WorkspaceState),
    (StateId::DockingConfig, OwnerId::WorkspaceState),
    (StateId::DiagnosticMessages, OwnerId::DiagnosticsState),
    (StateId::TraceLineage, OwnerId::DiagnosticsState),
];

pub(crate) const WORLD_OWNERSHIP_BINDINGS: &[(StateId, OwnerId)] = &[
    (StateId::WorldState, OwnerId::WorldState),
    (StateId::WorldIdentity, OwnerId::WorldState),
    (StateId::TerrainState, OwnerId::WorldState),
    (StateId::EnvironmentState, OwnerId::WorldState),
];

pub(crate) const ALWAYS_REQUIRED_STATES: &[StateId] = &[
    StateId::ProjectState,
    StateId::WorkspaceState,
    StateId::DiagnosticsState,
    StateId::ProjectIdentity,
    StateId::WorkspaceIdentity,
    StateId::SaveGeneration,
    StateId::ContentSnapshots,
    StateId::PanelLayout,
    StateId::DockingConfig,
    StateId::DiagnosticMessages,
    StateId::TraceLineage,
];

pub(crate) const WORLD_REQUIRED_STATES: &[StateId] = &[
    StateId::WorldState,
    StateId::WorldIdentity,
    StateId::TerrainState,
    StateId::EnvironmentState,
];
