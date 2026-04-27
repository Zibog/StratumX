// World Lifecycle DTOs
// Frozen semantic types for world open/close/restore operations

use crate::identity::{SkyEnvironmentBindingRef, StableWorldId, TerrainBindingRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpenMode {
    Startup,
    Recent,
    RestoreSession,
    Explicit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestorePolicy {
    RestoreFull,
    RestorePartial,
    FreshOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldOpenRequest {
    pub world_ref: Option<StableWorldId>,
    pub open_mode: OpenMode,
    pub restore_policy: RestorePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureClass {
    WorldNotFound,
    CorruptedData,
    IncompatibleVersion,
    MissingDependencies,
    PermissionDenied,
    EngineError,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldOpenResult {
    pub accepted: bool,
    pub world_ref: Option<StableWorldId>,
    pub world_label: Option<String>,
    pub failure_class: Option<FailureClass>,
    pub recovery_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindPosture {
    Unbound,
    Binding,
    Bound,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBindState {
    pub world_ref: StableWorldId,
    pub terrain_ref: Option<TerrainBindingRef>,
    pub environment_ref: Option<SkyEnvironmentBindingRef>,
    pub posture: BindPosture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSummaryDto {
    pub world_ref: StableWorldId,
    pub world_label: String,
    pub world_role: String,
    pub open_mode: OpenMode,
    pub bind_posture: BindPosture,
    pub active_selection_count: usize,
}
