//! Project Persistence View
//!
//! Separate persistence type for ProjectOwner that excludes runtime-only fields.

use crate::owners::project_owner::{
    ContentSnapshot, ProjectIdentity, ProjectOwner, WorkspaceIdentity,
};
use serde::{Deserialize, Serialize};

/// Project persistence view - contains only persistable state
///
/// This is separate from ProjectOwner to exclude runtime-only fields like event_bus.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectPersistenceView {
    /// Project identity from registry
    pub project_identity: ProjectIdentity,

    /// Workspace identity
    pub workspace_identity: WorkspaceIdentity,

    /// Current save generation number
    pub save_generation: u64,

    /// Content snapshots for undo/redo
    pub content_snapshots: Vec<ContentSnapshot>,
}

impl ProjectPersistenceView {
    /// Creates a new project persistence view
    pub fn new(
        project_identity: ProjectIdentity,
        workspace_identity: WorkspaceIdentity,
        save_generation: u64,
        content_snapshots: Vec<ContentSnapshot>,
    ) -> Self {
        Self {
            project_identity,
            workspace_identity,
            save_generation,
            content_snapshots,
        }
    }
}

/// Convert from ProjectOwner reference to ProjectPersistenceView
impl From<&ProjectOwner> for ProjectPersistenceView {
    fn from(owner: &ProjectOwner) -> Self {
        Self {
            project_identity: owner.project_identity.clone(),
            workspace_identity: owner.workspace_identity.clone(),
            save_generation: owner.save_generation,
            content_snapshots: owner.content_snapshots.clone(),
        }
    }
}

/// Convert from ProjectPersistenceView to ProjectOwner
///
/// Note: Runtime-only fields like event_callback are not restored and must be set separately.
impl TryFrom<ProjectPersistenceView> for ProjectOwner {
    type Error = String;

    fn try_from(view: ProjectPersistenceView) -> Result<Self, Self::Error> {
        Ok(ProjectOwner::from_persistence(
            view.project_identity,
            view.workspace_identity,
            view.save_generation,
            view.content_snapshots,
        ))
    }
}

