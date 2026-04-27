use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Project identity from registry.
///
/// Project IDs must come from a registry, never be generated at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectIdentity {
    /// Project ID from registry (not generated).
    pub project_id: Uuid,

    /// Project name.
    pub project_name: String,

    /// Project path on disk.
    pub project_path: PathBuf,
}

impl ProjectIdentity {
    /// Creates a new project identity.
    pub fn new(project_id: Uuid, project_name: String, project_path: PathBuf) -> Self {
        Self {
            project_id,
            project_name,
            project_path,
        }
    }
}
