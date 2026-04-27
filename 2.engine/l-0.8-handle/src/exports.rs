// Public API for handle module

pub use crate::types::{
    DenseExecutionHandle, InvalidationState, StableComponentHandle, StableEntityHandle,
    ValidationContext, ValidationResult,
};
pub use crate::validation::{
    is_component_valid, is_entity_valid, is_validation_successful, validation_result_desc,
    HandleError,
};
