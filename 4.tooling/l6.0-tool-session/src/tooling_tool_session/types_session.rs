// Tooling Session Types - Session-specific types

use super::types_core::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolObject {
    pub handle: ObjectHandle,
    pub label: String,
    pub class: ObjectClass,
    pub active: bool,
    pub fields: BTreeMap<String, String>,
    pub tags: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToolSnapshot {
    pub objects: Vec<ToolObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildManifest {
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub digest: String,
    pub object_count: usize,
    pub manifest: BuildManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleasePackage {
    pub build_digest: String,
    pub channel: String,
    pub signed: bool,
    pub manifest: ReleaseManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewResult {
    pub handle: ObjectHandle,
    pub summary: String,
}

/// Audio preview mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreviewMode {
    /// Preview single audio source in isolation
    Isolated,
    /// Preview audio source in world context with occlusion/obstruction
    Contextual,
}

/// Audio preview session
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewSession {
    pub session_id: String,
    pub source_handle: ObjectHandle,
    pub mode: PreviewMode,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedGoal {
    pub goal: String,
    pub suggested_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolTransaction {
    pub order: u64,
    pub origin: CommandOrigin,
    pub budget: BudgetClass,
    pub summary: String,
    pub mutation_set: Vec<Mutation>,
    pub rollback_binding: RollbackBinding,
    pub committed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mutation {
    ObjectCreated {
        handle: ObjectHandle,
        class: ObjectClass,
    },
    ObjectDeleted {
        handle: ObjectHandle,
    },
    FieldSet {
        handle: ObjectHandle,
        field: String,
        value: Option<String>,
    },
    TagAdded {
        handle: ObjectHandle,
        tag: String,
    },
    TagRemoved {
        handle: ObjectHandle,
        tag: String,
    },
}

/// Branch Coverage - tracks completeness of material bindings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchCoverage {
    pub physical_complete: bool,
    pub visual_complete: bool,
    pub acoustic_complete: bool,
    pub light_complete: bool,
    pub runtime_complete: bool,
    pub missing_bindings: Vec<String>,
    pub invalid_combinations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackBinding {
    pub operations: Vec<RollbackOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackOperation {
    DeleteObject {
        handle: ObjectHandle,
    },
    RestoreObject {
        handle: ObjectHandle,
        object: ToolObject,
    },
    RestoreField {
        handle: ObjectHandle,
        field: String,
        value: Option<String>,
    },
    RemoveTag {
        handle: ObjectHandle,
        tag: String,
    },
    AddTag {
        handle: ObjectHandle,
        tag: String,
    },
}
