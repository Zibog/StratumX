//! Project projections — derived views, summaries, and inspector data.

use super::project_owner::{ContentSnapshot, ProjectIdentity, ProjectOwner, WorkspaceIdentity};

impl ProjectOwner {
    /// Gets the current save generation
    pub fn get_save_generation(&self) -> u64 {
        self.save_generation
    }

    /// Gets the project identity
    pub fn get_project_identity(&self) -> &ProjectIdentity {
        &self.project_identity
    }

    /// Gets the workspace identity
    pub fn get_workspace_identity(&self) -> &WorkspaceIdentity {
        &self.workspace_identity
    }

    /// Gets all content snapshots
    pub fn get_snapshots(&self) -> &[ContentSnapshot] {
        &self.content_snapshots
    }

    /// Gets the project identity (alias for compatibility)
    pub fn get_identity(&self) -> &ProjectIdentity {
        self.get_project_identity()
    }

    /// Converts to persistence view for serialization
    ///
    /// Excludes runtime-only fields like event_callback.
    pub fn to_persistence_view(&self) -> crate::persistence::ProjectPersistenceView {
        crate::persistence::ProjectPersistenceView::new(
            self.project_identity.clone(),
            self.workspace_identity.clone(),
            self.save_generation,
            self.content_snapshots.clone(),
        )
    }
}
