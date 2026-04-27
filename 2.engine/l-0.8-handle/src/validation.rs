use crate::types::{
    InvalidationState, StableComponentHandle, StableEntityHandle, ValidationResult,
};
use serde::{Deserialize, Serialize};

/// Errors that can occur during handle operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandleError {
    /// Handle validation failed.
    ValidationFailed(String),
    /// Handle context mismatch.
    ContextMismatch,
    /// Invalid handle state transition.
    InvalidStateTransition,
}

impl std::fmt::Display for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandleError::ValidationFailed(msg) => write!(f, "validation failed: {}", msg),
            HandleError::ContextMismatch => write!(f, "context mismatch"),
            HandleError::InvalidStateTransition => write!(f, "invalid state transition"),
        }
    }
}

impl std::error::Error for HandleError {}

/// Check if entity handle is in valid state.
pub fn is_entity_valid(handle: &StableEntityHandle) -> bool {
    handle.state == InvalidationState::Active
}

/// Check if component handle is in valid state.
pub fn is_component_valid(handle: &StableComponentHandle) -> bool {
    handle.state == InvalidationState::Active
}

/// Get validation result as boolean.
pub fn is_validation_successful(result: ValidationResult) -> bool {
    matches!(result, ValidationResult::Valid)
}

/// Describe validation result as string.
pub fn validation_result_desc(result: ValidationResult) -> &'static str {
    match result {
        ValidationResult::Valid => "valid",
        ValidationResult::Invalidated => "invalidated",
        ValidationResult::Stale => "stale",
        ValidationResult::IllegalContext => "illegal context",
    }
}
