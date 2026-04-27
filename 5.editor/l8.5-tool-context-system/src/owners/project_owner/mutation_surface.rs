/// Event emitted when project state changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStateEvent {
    /// Save generation incremented.
    SaveGenerationIncremented { new_generation: u64 },

    /// Content snapshot added.
    SnapshotAdded { snapshot_id: uuid::Uuid },

    /// Project identity updated.
    ProjectIdentityUpdated,

    /// Workspace identity updated.
    WorkspaceIdentityUpdated,
}

/// Callback type for state change events.
pub type StateEventCallback = Box<dyn Fn(ProjectStateEvent) + Send + Sync>;
