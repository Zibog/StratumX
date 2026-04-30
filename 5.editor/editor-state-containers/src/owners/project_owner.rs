//! Project owner types

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectIdentity {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_path: PathBuf,
}

impl ProjectIdentity {
    pub fn new(project_id: Uuid, project_name: String, project_path: PathBuf) -> Self {
        Self {
            project_id,
            project_name,
            project_path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceIdentity {
    pub workspace_id: Uuid,
    pub workspace_name: String,
    pub workspace_path: PathBuf,
}

impl WorkspaceIdentity {
    pub fn new(workspace_id: Uuid, workspace_name: String, workspace_path: PathBuf) -> Self {
        Self {
            workspace_id,
            workspace_name,
            workspace_path,
        }
    }
}

pub type EventCallback = Box<dyn Fn(String) + Send + Sync>;

#[derive(Serialize, Deserialize)]
pub struct ProjectOwner {
    pub project_identity: ProjectIdentity,
    pub workspace_identity: WorkspaceIdentity,
    pub save_generation: u64,
    pub content_snapshots: Vec<String>,
    #[serde(skip)]
    pub event_callback: Option<EventCallback>,
}

impl std::fmt::Debug for ProjectOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectOwner")
            .field("project_identity", &self.project_identity)
            .field("workspace_identity", &self.workspace_identity)
            .field("save_generation", &self.save_generation)
            .field("content_snapshots", &self.content_snapshots)
            .finish_non_exhaustive()
    }
}

impl Clone for ProjectOwner {
    fn clone(&self) -> Self {
        Self {
            project_identity: self.project_identity.clone(),
            workspace_identity: self.workspace_identity.clone(),
            save_generation: self.save_generation,
            content_snapshots: self.content_snapshots.clone(),
            event_callback: None,
        }
    }
}

impl PartialEq for ProjectOwner {
    fn eq(&self, other: &Self) -> bool {
        self.project_identity == other.project_identity
            && self.workspace_identity == other.workspace_identity
            && self.save_generation == other.save_generation
            && self.content_snapshots == other.content_snapshots
    }
}

impl ProjectOwner {
    pub fn new(project_identity: ProjectIdentity, workspace_identity: WorkspaceIdentity) -> Self {
        Self {
            project_identity,
            workspace_identity,
            save_generation: 0,
            content_snapshots: Vec::new(),
            event_callback: None,
        }
    }
    pub fn set_event_callback(&mut self, cb: EventCallback) {
        self.event_callback = Some(cb);
    }
    pub fn increment_save_generation(&mut self) {
        self.save_generation += 1;
        if let Some(ref cb) = self.event_callback {
            cb(format!("SaveGenerationIncremented:{}", self.save_generation));
        }
    }
    pub fn get_save_generation(&self) -> u64 {
        self.save_generation
    }
    pub fn add_snapshot(&mut self, snapshot: String) {
        self.content_snapshots.push(snapshot);
    }
    pub fn get_snapshots(&self) -> &[String] {
        &self.content_snapshots
    }
    pub fn to_persistence_view(&self) -> crate::persistence::ProjectPersistenceView {
        crate::persistence::ProjectPersistenceView {
            project_identity: self.project_identity.clone(),
            workspace_identity: self.workspace_identity.clone(),
            save_generation: self.save_generation,
            content_snapshots: self.content_snapshots.clone(),
        }
    }
}
