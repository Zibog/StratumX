use crate::types::PropertyType;
use thiserror::Error;

/// Validation errors for world property substrate.
#[derive(Debug, Error)]
pub enum SubstrateValidationError {
    #[error("Invalid field dimensions: {0}")]
    InvalidDimensions(String),

    #[error("Property not found: {0:?}")]
    PropertyNotFound(PropertyType),

    #[error("Coordinates out of bounds: ({0}, {1}, {2})")]
    OutOfBounds(usize, usize, usize),

    #[error("Cycle detected in update order graph")]
    CycleDetected,

    #[error("Invalid field value: {0}")]
    InvalidValue(String),

    #[error("Conflict resolution failed: {0}")]
    ConflictResolutionFailed(String),

    #[error("Persistence error: {0}")]
    PersistenceError(String),
}
