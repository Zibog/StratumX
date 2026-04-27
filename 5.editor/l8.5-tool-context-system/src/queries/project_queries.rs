//! Project query views
//!
//! Read-only views of ProjectOwner state implementing ReadModel trait.

use crate::owners::project_owner::ProjectOwner;
use crate::queries::ReadModel;
use std::path::PathBuf;
use uuid::Uuid;

/// Project identity view
///
/// Read-only view of project identity information.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectIdentityView {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_path: PathBuf,
    pub workspace_id: Uuid,
    pub workspace_name: String,
    pub save_generation: u64,
}

impl ReadModel<ProjectOwner, ProjectIdentityView> for ProjectIdentityView {
    fn build(owner: &ProjectOwner) -> Self {
        Self {
            project_id: owner.get_project_identity().project_id,
            project_name: owner.get_project_identity().project_name.clone(),
            project_path: owner.get_project_identity().project_path.clone(),
            workspace_id: owner.get_workspace_identity().workspace_id,
            workspace_name: owner.get_workspace_identity().workspace_name.clone(),
            save_generation: owner.get_save_generation(),
        }
    }
}

/// Project snapshots view
///
/// Read-only view of content snapshots for undo/redo.
#[derive(Debug, Clone)]
pub struct ProjectSnapshotsView {
    pub snapshots: Vec<SnapshotInfo>,
    pub snapshot_count: usize,
}

impl ReadModel<ProjectOwner, ProjectSnapshotsView> for ProjectSnapshotsView {
    fn build(owner: &ProjectOwner) -> Self {
        let snapshots = owner
            .get_snapshots()
            .iter()
            .map(|s| SnapshotInfo {
                snapshot_id: s.snapshot_id,
                timestamp: s.timestamp,
                description: s.description.clone(),
                data_size: s.content_data.len(),
            })
            .collect::<Vec<_>>();

        let snapshot_count = snapshots.len();

        Self {
            snapshots,
            snapshot_count,
        }
    }
}

/// Snapshot information (without full content data)
#[derive(Debug, Clone)]
pub struct SnapshotInfo {
    pub snapshot_id: Uuid,
    pub timestamp: u64,
    pub description: String,
    pub data_size: usize,
}


