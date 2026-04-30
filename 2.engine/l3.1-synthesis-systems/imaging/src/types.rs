use engine_core::{EngineCoreError, StableDigest64, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::ResidencyControlService;
use engine_transfer_control::TransferControlService;
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub type ImagingResult<T> = Result<T, ImagingFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImagingFailureReason {
    InvalidRenderTarget,
    UploadBytesExceeded,
    MissingMaterialVisualPolicy,
    BudgetRejected,
}

impl ImagingFailureReason {
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidRenderTarget => "render target id exceeds configured logical bound",
            Self::UploadBytesExceeded => "upload bytes exceed configured imaging ceiling",
            Self::MissingMaterialVisualPolicy => "material visual policy is missing",
            Self::BudgetRejected => "imaging transport budget rejected",
        }
    }

    const fn code(self) -> u8 {
        match self {
            Self::InvalidRenderTarget => 1,
            Self::UploadBytesExceeded => 2,
            Self::MissingMaterialVisualPolicy => 3,
            Self::BudgetRejected => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingFailure {
    pub reason: ImagingFailureReason,
    pub digest: StableDigest64,
}

impl ImagingFailure {
    pub fn for_reason(reason: ImagingFailureReason) -> Self {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.imaging.failure")
            .write_u8(reason.code());
        Self {
            reason,
            digest: digest.finish(),
        }
    }
}

impl From<ImagingFailure> for EngineCoreError {
    fn from(failure: ImagingFailure) -> Self {
        EngineCoreError::InvalidDescriptor(failure.reason.message())
    }
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
    pub render_target_id: u64,
    pub selected_tier: ImagingTier,
    pub rendered_frames: usize,
    pub upload_bytes: usize,
    pub resource_count: usize,
    pub used_material_fallback: bool,
    pub upload_accepted: bool,
    pub material_policy_id: u64,
    pub deterministic_digest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingTransportOutcome {
    pub rendered_frames: usize,
    pub upload_bytes: usize,
    pub upload_accepted: bool,
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

pub struct ImagingInputs<'a> {
    pub world: &'a WorldState,
    pub ecs: &'a EcsSubstrate,
    pub materials: &'a MaterialRegistry,
    pub residency: &'a ResidencyControlService,
    pub transfer: &'a mut TransferControlService,
    pub material_id: MaterialId,
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
