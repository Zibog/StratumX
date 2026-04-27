use crate::bridge_types::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandOrigin {
    User,
    Automation,
    Assistant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalClass {
    None,
    ReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetClass {
    Interactive,
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactClass {
    Build,
    Release,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCommand {
    CreateObject {
        label: String,
        class: ObjectClass,
    },
    SetLabel {
        handle: ObjectHandle,
        label: String,
    },
    UpsertField {
        handle: ObjectHandle,
        key: String,
        value: String,
    },
    AddTag {
        handle: ObjectHandle,
        tag: String,
    },
    SelectObject {
        handle: ObjectHandle,
    },
    DeleteObject {
        handle: ObjectHandle,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCommandResult {
    Created(ObjectHandle),
    Modified,
    Tagged,
    Deleted,
    Selected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    pub severity: String,
    pub message: String,
    pub handle: Option<ObjectHandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewResult {
    pub handle: ObjectHandle,
    pub preview_data: Vec<u8>,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    pub kind: ArtifactClass,
    pub digest: String,
    pub invalidation_roots: Vec<String>,
    pub artifact_count: usize,
    pub total_size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub manifest: BuildManifest,
    pub object_count: usize,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEvidence {
    pub proposal_id: u64,
    pub goal: String,
    pub commands: Vec<ToolCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub channel: String,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleasePackage {
    pub manifest: ReleaseManifest,
    pub build_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolTransaction {
    pub command: ToolCommand,
    pub result: ToolCommandResult,
    pub order: u64,
    pub origin: CommandOrigin,
    pub approval: ApprovalClass,
    pub budget: BudgetClass,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSnapshot {
    pub generation: u64,
    pub objects: Vec<ToolObject>,
    pub transactions: Vec<ToolTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolObject {
    pub handle: ObjectHandle,
    pub class: ObjectClass,
    pub label: String,
    pub fields: BTreeMap<String, String>,
    pub tags: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub focused_view: String,
    pub open_views: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantPlan {
    pub goal: String,
    pub steps: Vec<String>,
    pub suggested_commands: Vec<ToolCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantProposal {
    pub id: u64,
    pub goal: String,
    pub commands: Vec<ToolCommand>,
    pub approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedSummary {
    pub active_objects: usize,
    pub retired_objects: usize,
    pub object_count: usize,
    pub transaction_count: usize,
}

impl DerivedSummary {
    pub fn from_snapshot(snapshot: &ToolSnapshot) -> Self {
        Self {
            active_objects: snapshot.objects.len(),
            retired_objects: 0,
            object_count: snapshot.objects.len(),
            transaction_count: snapshot.transactions.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolingIndex {
    pub objects: Vec<ToolObject>,
    pub summary: DerivedSummary,
    pub by_tag: BTreeMap<String, Vec<ObjectHandle>>,
}
