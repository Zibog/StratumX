use engine_core::EngineCoreError;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type ContentPipelineResult<T> = Result<T, ContentFailure>;

/// Failure reasons for content operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentFailureReason {
    EmptyContent,
    InvalidDescriptor,
    InvalidLocator,
    DuplicateContentRejected,
    ContentIdConflict,
    LocatorConflict,
    ManifestPackCeiling,
    ChunkSizeZero,
    DependencyCycle,
    MissingDependency,
    RuntimePackIncompatible,
}

impl ContentFailureReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyContent => "empty content",
            Self::InvalidDescriptor => "invalid descriptor",
            Self::InvalidLocator => "invalid locator",
            Self::DuplicateContentRejected => "duplicate content rejected",
            Self::ContentIdConflict => "content id conflict",
            Self::LocatorConflict => "locator conflict",
            Self::ManifestPackCeiling => "manifest pack ceiling exceeded",
            Self::ChunkSizeZero => "chunk size zero",
            Self::DependencyCycle => "dependency cycle",
            Self::MissingDependency => "missing dependency",
            Self::RuntimePackIncompatible => "runtime pack incompatible",
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::EmptyContent => "content ingest requires non-empty bytes",
            Self::InvalidDescriptor => "content descriptor is invalid",
            Self::InvalidLocator => "content locator is invalid",
            Self::DuplicateContentRejected => {
                "duplicate content digest requires canonical reuse of existing pack id"
            }
            Self::ContentIdConflict => "content id already bound to different content",
            Self::LocatorConflict => "content locator already bound to different content",
            Self::ManifestPackCeiling => "content manifest exceeds configured pack ceiling",
            Self::ChunkSizeZero => "chunk size cannot be zero",
            Self::DependencyCycle => "runtime pack cannot depend on itself",
            Self::MissingDependency => "runtime pack dependencies must be unique",
            Self::RuntimePackIncompatible => "runtime pack manifest is incompatible",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentFailure {
    pub reason: ContentFailureReason,
    message: &'static str,
}

impl ContentFailure {
    pub const fn for_reason(reason: ContentFailureReason) -> Self {
        Self {
            reason,
            message: reason.message(),
        }
    }

    pub const fn new(reason: ContentFailureReason, message: &'static str) -> Self {
        Self { reason, message }
    }

    pub const fn reason(self) -> ContentFailureReason {
        self.reason
    }

    pub const fn message(self) -> &'static str {
        self.message
    }

    pub fn into_engine_core_error(self) -> EngineCoreError {
        EngineCoreError::InvalidDescriptor(self.message)
    }
}

impl From<ContentFailure> for EngineCoreError {
    fn from(failure: ContentFailure) -> Self {
        failure.into_engine_core_error()
    }
}

impl PartialEq<ContentFailureReason> for ContentFailure {
    fn eq(&self, other: &ContentFailureReason) -> bool {
        self.reason == *other
    }
}

impl PartialEq<ContentFailure> for ContentFailureReason {
    fn eq(&self, other: &ContentFailure) -> bool {
        *self == other.reason
    }
}

impl fmt::Display for ContentFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.reason.as_str(), self.message)
    }
}

impl std::error::Error for ContentFailure {}
