use engine_core::{EngineCoreError, StableDigest64, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::ResidencyControlService;
use engine_transfer_control::TransferControlService;
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub type AcousticResult<T> = Result<T, AcousticFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcousticFailureReason {
    InvalidSourceCount,
    StreamUploadBytesExceeded,
    MissingMaterialAcousticProfile,
    BudgetRejected,
}

impl AcousticFailureReason {
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidSourceCount => "source count must be non-zero",
            Self::StreamUploadBytesExceeded => {
                "stream upload bytes exceed configured acoustic ceiling"
            }
            Self::MissingMaterialAcousticProfile => "material acoustic profile is missing",
            Self::BudgetRejected => "acoustic transport budget rejected",
        }
    }

    const fn code(self) -> u8 {
        match self {
            Self::InvalidSourceCount => 1,
            Self::StreamUploadBytesExceeded => 2,
            Self::MissingMaterialAcousticProfile => 3,
            Self::BudgetRejected => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticFailure {
    pub reason: AcousticFailureReason,
    pub digest: StableDigest64,
}

impl AcousticFailure {
    pub fn for_reason(reason: AcousticFailureReason) -> Self {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.acoustics.failure")
            .write_u8(reason.code());
        Self {
            reason,
            digest: digest.finish(),
        }
    }
}

impl From<AcousticFailure> for EngineCoreError {
    fn from(failure: AcousticFailure) -> Self {
        EngineCoreError::InvalidDescriptor(failure.reason.message())
    }
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
    pub synthesized_frames: usize,
    pub source_count: usize,
    pub propagated_sources: usize,
    pub stream_upload_bytes: usize,
    pub emitter_count: usize,
    pub used_material_fallback: bool,
    pub stream_upload_accepted: bool,
    pub material_policy_id: u64,
    pub deterministic_digest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsTransportOutcome {
    pub synthesized_frames: usize,
    pub propagated_sources: usize,
    pub stream_upload_bytes: usize,
    pub stream_upload_accepted: bool,
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

pub struct AcousticInputs<'a> {
    pub world: &'a WorldState,
    pub ecs: &'a EcsSubstrate,
    pub materials: &'a MaterialRegistry,
    pub residency: &'a ResidencyControlService,
    pub transfer: &'a mut TransferControlService,
    pub material_id: MaterialId,
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
