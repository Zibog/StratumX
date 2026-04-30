use engine_core::{EngineCoreError, StableDigest64, StableDigestBuilder};
use serde::{Deserialize, Serialize};

pub type ModelBoundaryResult<T> = Result<T, ModelBoundaryFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelBoundaryFailureReason {
    MissingModelId,
    EmptyPrompt,
    InvalidBatchSize,
}

impl ModelBoundaryFailureReason {
    pub const fn message(self) -> &'static str {
        match self {
            Self::MissingModelId => "inference service requires non-empty model id",
            Self::EmptyPrompt => "inference request requires non-empty prompt",
            Self::InvalidBatchSize => "inference batch size is illegal for configured ceiling",
        }
    }

    const fn code(self) -> u8 {
        match self {
            Self::MissingModelId => 1,
            Self::EmptyPrompt => 2,
            Self::InvalidBatchSize => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelBoundaryFailure {
    pub reason: ModelBoundaryFailureReason,
    pub digest: StableDigest64,
}

impl ModelBoundaryFailure {
    pub fn for_reason(reason: ModelBoundaryFailureReason) -> Self {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.inference.failure")
            .write_u8(reason.code());
        Self {
            reason,
            digest: digest.finish(),
        }
    }
}

impl From<ModelBoundaryFailure> for EngineCoreError {
    fn from(failure: ModelBoundaryFailure) -> Self {
        EngineCoreError::InvalidDescriptor(failure.reason.message())
    }
}
