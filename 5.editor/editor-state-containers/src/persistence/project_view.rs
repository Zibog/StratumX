//! Project persistence view

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPersistenceView {
    pub project_identity: crate::owners::project_owner::ProjectIdentity,
    pub workspace_identity: crate::owners::project_owner::WorkspaceIdentity,
    pub save_generation: u64,
    pub content_snapshots: Vec<String>,
}
