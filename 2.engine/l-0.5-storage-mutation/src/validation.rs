use crate::MutationFailureReason;

// Validation types for storage mutation

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationError {
    InvalidBatchOrder,
    EmptyChangeset,
    DuplicateOperation,
    ConflictingOperation,
    ReplayGuardRequired,
    ReplayStateMismatch,
}

impl MutationError {
    pub fn as_failure_reason(self) -> MutationFailureReason {
        match self {
            Self::InvalidBatchOrder => MutationFailureReason::InvalidBatchOrder,
            Self::EmptyChangeset => MutationFailureReason::EmptyChangeset,
            Self::DuplicateOperation => MutationFailureReason::DuplicateOperation,
            Self::ConflictingOperation => MutationFailureReason::ConflictingOperation,
            Self::ReplayGuardRequired => MutationFailureReason::ReplayGuardRequired,
            Self::ReplayStateMismatch => MutationFailureReason::ReplayStateMismatch,
        }
    }
}
