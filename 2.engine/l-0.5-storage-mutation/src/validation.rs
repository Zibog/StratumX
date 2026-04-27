use serde::{Deserialize, Serialize};

/// Errors in mutation operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationError {
    /// Deferred writes require a staged mutation handoff window.
    InvalidHandoffWindow,
    /// Batch order must be non-zero.
    InvalidBatchOrder,
    /// Region tag mismatch.
    RegionMismatch,
    /// Family tag mismatch.
    FamilyMismatch,
    /// Changeset is empty when expected to have content.
    EmptyChangeset,
    /// Component write failed validation.
    InvalidWrite(String),
}

impl std::fmt::Display for MutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutationError::InvalidHandoffWindow => write!(
                f,
                "deferred writes require staged mutation handoff window"
            ),
            MutationError::InvalidBatchOrder => write!(f, "batch order must be non-zero"),
            MutationError::RegionMismatch => write!(f, "region tag mismatch"),
            MutationError::FamilyMismatch => write!(f, "family tag mismatch"),
            MutationError::EmptyChangeset => write!(f, "changeset is empty"),
            MutationError::InvalidWrite(msg) => write!(f, "invalid write: {}", msg),
        }
    }
}

impl std::error::Error for MutationError {}

/// Validate batch order is non-zero.
pub fn validate_batch_order(order: u64) -> Result<(), MutationError> {
    if order == 0 {
        return Err(MutationError::InvalidBatchOrder);
    }
    Ok(())
}
