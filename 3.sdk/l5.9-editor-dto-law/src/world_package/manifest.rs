use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldPackageManifest {
    pub world_id: Uuid,
    pub world_label: String,
    pub world_role: WorldRole,
    pub version: String,
    pub terrain_root_ref: Option<String>,
    pub environment_root_ref: Option<String>,
    pub streaming_profile_ref: Option<String>,
    pub source_lineage: SourceLineage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldRole {
    Startup,
    Reference,
    Demo,
    Content,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceLineage {
    pub created_at: String,
    pub created_by: String,
    pub import_source: Option<String>,
    pub last_modified: String,
}
