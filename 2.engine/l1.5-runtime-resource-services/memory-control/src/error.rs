use engine_core::EngineCoreError;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type MemoryControlResult<T> = Result<T, MemoryFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryFailureReason {
    ZeroAllocation,
    DuplicateAllocationId,
    UnknownAllocation,
    AlreadyReleased,
    PoolMismatch,
    SizeMismatch,
    OverBudget,
}

impl MemoryFailureReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ZeroAllocation => "zero allocation",
            Self::DuplicateAllocationId => "duplicate allocation id",
            Self::UnknownAllocation => "unknown allocation",
            Self::AlreadyReleased => "already released",
            Self::PoolMismatch => "pool mismatch",
            Self::SizeMismatch => "size mismatch",
            Self::OverBudget => "over budget",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryFailure {
    pub reason: MemoryFailureReason,
    message: &'static str,
}

impl MemoryFailure {
    pub const fn new(reason: MemoryFailureReason, message: &'static str) -> Self {
        Self { reason, message }
    }

    pub const fn reason(self) -> MemoryFailureReason {
        self.reason
    }

    pub const fn message(self) -> &'static str {
        self.message
    }

    pub fn into_engine_core_error(self) -> EngineCoreError {
        let message = match self.reason {
            MemoryFailureReason::AlreadyReleased => "memory release references unknown allocation",
            _ => self.message,
        };
        EngineCoreError::InvalidDescriptor(message)
    }
}

impl From<MemoryFailure> for EngineCoreError {
    fn from(failure: MemoryFailure) -> Self {
        failure.into_engine_core_error()
    }
}

impl PartialEq<MemoryFailureReason> for MemoryFailure {
    fn eq(&self, other: &MemoryFailureReason) -> bool {
        self.reason == *other
    }
}

impl PartialEq<MemoryFailure> for MemoryFailureReason {
    fn eq(&self, other: &MemoryFailure) -> bool {
        *self == other.reason
    }
}

impl fmt::Display for MemoryFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.reason.as_str(), self.message)
    }
}

impl std::error::Error for MemoryFailure {}
