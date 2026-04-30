use engine_material::MaterialId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcousticFailureReason {
    MissingMaterialAcousticProfile,
    InvalidEmitter,
    InvalidListener,
    BudgetRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcousticTier {
    FullPropagation,
    ReducedPropagation,
    EventOnly,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticReceipt {
    pub material_id: MaterialId,
    pub selected_tier: AcousticTier,
    pub emitter_count: usize,
    pub used_material_fallback: bool,
    pub stream_upload_accepted: bool,
    pub material_policy_id: u64,
    pub deterministic_digest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsConfig {
    pub max_sources: usize,
    pub max_stream_upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsRequest {
    pub source_count: usize,
    pub stream_upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsResult {
    pub synthesized_frames: usize,
    pub propagated_sources: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticSource {
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticEnvironment {
    pub region_key: (i32, i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticPropagation {
    pub paths: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticOutput {
    pub frames: usize,
}
