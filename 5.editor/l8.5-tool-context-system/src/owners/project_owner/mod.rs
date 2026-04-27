//! Project owner aggregate.
//!
//! This module splits identity, refs, settings, and mutation-facing types into
//! explicit submodules while keeping the public owner API stable.

use serde::{Deserialize, Serialize};

mod identity;
mod mutation_surface;
mod refs;
mod settings;

pub use identity::ProjectIdentity;
pub use mutation_surface::{ProjectStateEvent, StateEventCallback};
pub use refs::{ContentSnapshot, WorkspaceIdentity};
pub use settings::SaveGeneration;

/// Project state container.
///
/// Owns project-level state including identity, workspace configuration,
/// and save generation tracking.
#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectOwner {
    /// Project identity from registry.
    pub project_identity: ProjectIdentity,

    /// Workspace identity.
    pub workspace_identity: WorkspaceIdentity,

    /// Current save generation number.
    pub save_generation: SaveGeneration,

    /// Content snapshots for undo/redo.
    pub content_snapshots: Vec<ContentSnapshot>,

    /// Event callback (not serialized).
    #[serde(skip)]
    pub(crate) event_callback: Option<std::sync::Arc<StateEventCallback>>,
}

impl std::fmt::Debug for ProjectOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectOwner")
            .field("project_identity", &self.project_identity)
            .field("workspace_identity", &self.workspace_identity)
            .field("save_generation", &self.save_generation)
            .field("content_snapshots", &self.content_snapshots)
            .field(
                "event_callback",
                &self.event_callback.as_ref().map(|_| "<callback>"),
            )
            .finish()
    }
}

impl ProjectOwner {
    /// Creates a new project state.
    pub fn new(project_identity: ProjectIdentity, workspace_identity: WorkspaceIdentity) -> Self {
        Self {
            project_identity,
            workspace_identity,
            save_generation: 0,
            content_snapshots: Vec::new(),
            event_callback: None,
        }
    }

    /// Creates a project state from persistence data.
    pub fn from_persistence(
        project_identity: ProjectIdentity,
        workspace_identity: WorkspaceIdentity,
        save_generation: SaveGeneration,
        content_snapshots: Vec<ContentSnapshot>,
    ) -> Self {
        Self {
            project_identity,
            workspace_identity,
            save_generation,
            content_snapshots,
            event_callback: None,
        }
    }
}
