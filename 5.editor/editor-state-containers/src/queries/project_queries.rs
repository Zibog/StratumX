//! Project query types

use super::ReadModel;
use crate::owners::project_owner::ProjectOwner;

#[derive(Debug, Clone)]
pub struct ProjectIdentityView {
    pub project_name: String,
    pub project_path: String,
    pub workspace_name: String,
    pub save_generation: u64,
}

impl ReadModel<ProjectOwner> for ProjectIdentityView {
    fn build(owner: &ProjectOwner) -> Self {
        Self {
            project_name: owner.project_identity.project_name.clone(),
            project_path: owner
                .project_identity
                .project_path
                .to_string_lossy()
                .to_string(),
            workspace_name: owner.workspace_identity.workspace_name.clone(),
            save_generation: owner.save_generation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectWorkspaceView {
    pub workspace_name: String,
}

impl ReadModel<ProjectOwner> for ProjectWorkspaceView {
    fn build(owner: &ProjectOwner) -> Self {
        Self {
            workspace_name: owner.workspace_identity.workspace_name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectSnapshotsView {
    pub snapshot_count: usize,
}

impl ReadModel<ProjectOwner> for ProjectSnapshotsView {
    fn build(owner: &ProjectOwner) -> Self {
        Self {
            snapshot_count: owner.get_snapshots().len(),
        }
    }
}
