use engine_material::MaterialId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImagingFailureReason {
    MissingMaterialVisualPolicy,
    MissingTextureResource,
    InvalidImagingTier,
    BudgetRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImagingTier {
    Full,
    Reduced,
    Impostor,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingReceipt {
    pub material_id: MaterialId,
    pub selected_tier: ImagingTier,
    pub resource_count: usize,
    pub used_material_fallback: bool,
    pub upload_accepted: bool,
    pub material_policy_id: u64,
    pub deterministic_digest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingConfig {
    pub max_render_targets: usize,
    pub max_upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingRequest {
    pub render_target_id: u64,
    pub view_region: (i32, i32, i32),
    pub upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingResult {
    pub rendered_frames: usize,
    pub upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderTarget {
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderView {
    pub region_key: (i32, i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameResource {
    pub target: RenderTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadStage {
    pub bytes: usize,
}
