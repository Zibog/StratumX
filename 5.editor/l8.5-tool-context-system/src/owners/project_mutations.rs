//! Project mutations — methods that change project state.

use super::project_owner::{
    ContentSnapshot, ProjectIdentity, ProjectOwner, ProjectStateEvent, StateEventCallback,
    WorkspaceIdentity,
};

impl ProjectOwner {
    /// Sets the event callback for state mutations
    pub fn set_event_callback(&mut self, callback: StateEventCallback) {
        self.event_callback = Some(std::sync::Arc::new(callback));
    }

    /// Emits an event if a callback is registered
    fn emit_event(&self, event: ProjectStateEvent) {
        if let Some(callback) = &self.event_callback {
            callback(event);
        }
    }

    /// Increments the save generation and emits event
    pub fn increment_save_generation(&mut self) {
        self.save_generation += 1;
        self.emit_event(ProjectStateEvent::SaveGenerationIncremented {
            new_generation: self.save_generation,
        });
    }

    /// Adds a content snapshot and emits event
    pub fn add_snapshot(&mut self, snapshot: ContentSnapshot) {
        let snapshot_id = snapshot.snapshot_id;
        self.content_snapshots.push(snapshot);
        self.emit_event(ProjectStateEvent::SnapshotAdded { snapshot_id });
    }

    /// Updates project identity and emits event
    pub fn update_project_identity(&mut self, identity: ProjectIdentity) {
        self.project_identity = identity;
        self.emit_event(ProjectStateEvent::ProjectIdentityUpdated);
    }

    /// Updates workspace identity and emits event
    pub fn update_workspace_identity(&mut self, identity: WorkspaceIdentity) {
        self.workspace_identity = identity;
        self.emit_event(ProjectStateEvent::WorkspaceIdentityUpdated);
    }

    /// Clears old snapshots, keeping only the most recent N
    pub fn prune_snapshots(&mut self, keep_count: usize) {
        if self.content_snapshots.len() > keep_count {
            let remove_count = self.content_snapshots.len() - keep_count;
            self.content_snapshots.drain(0..remove_count);
        }
    }
}
