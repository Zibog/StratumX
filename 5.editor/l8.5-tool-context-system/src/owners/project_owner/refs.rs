use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Workspace identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceIdentity {
    /// Workspace ID.
    pub workspace_id: Uuid,

    /// Workspace name.
    pub workspace_name: String,

    /// Workspace root path.
    pub workspace_path: PathBuf,
}

impl WorkspaceIdentity {
    /// Creates a new workspace identity.
    pub fn new(workspace_id: Uuid, workspace_name: String, workspace_path: PathBuf) -> Self {
        Self {
            workspace_id,
            workspace_name,
            workspace_path,
        }
    }
}

/// Content snapshot for undo/redo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSnapshot {
    /// Snapshot ID.
    pub snapshot_id: Uuid,

    /// Timestamp of snapshot.
    pub timestamp: u64,

    /// Description of changes.
    pub description: String,

    /// Serialized content state.
    pub content_data: Vec<u8>,
}

impl ContentSnapshot {
    /// Creates a new content snapshot.
    pub fn new(
        snapshot_id: Uuid,
        timestamp: u64,
        description: String,
        content_data: Vec<u8>,
    ) -> Self {
        Self {
            snapshot_id,
            timestamp,
            description,
            content_data,
        }
    }
}
