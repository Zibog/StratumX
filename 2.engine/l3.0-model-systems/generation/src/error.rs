use engine_core::{EngineCoreError, StableDigest64, StableDigestBuilder};
use serde::{Deserialize, Serialize};

pub type ModelBoundaryResult<T> = Result<T, ModelBoundaryFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelBoundaryFailureReason {
    MissingModelFamily,
    MissingWeightsChecksum,
    EmptyPrompt,
    OutputTooLarge,
    InferenceUnavailable,
}

impl ModelBoundaryFailureReason {
    pub const fn message(self) -> &'static str {
        match self {
            Self::MissingModelFamily => "generation context requires model family",
            Self::MissingWeightsChecksum => "generation context requires weights checksum",
            Self::EmptyPrompt => "generation request requires non-empty prompt",
            Self::OutputTooLarge => "generation artifact exceeds configured ceiling",
            Self::InferenceUnavailable => "generation inference dependency rejected request",
        }
    }

    const fn code(self) -> u8 {
        match self {
            Self::MissingModelFamily => 1,
            Self::MissingWeightsChecksum => 2,
            Self::EmptyPrompt => 3,
            Self::OutputTooLarge => 4,
            Self::InferenceUnavailable => 5,
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
            .write_bytes(b"engine.generation.failure")
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
