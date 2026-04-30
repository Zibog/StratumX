use std::collections::HashMap;

use super::errors::{new_error_state, ErrorClass, ErrorState};
use super::rollback_policy::requires_rollback;

/// Manages retry, rollback, and error state clearing for authority operations.
pub struct RecoveryManager {
    /// Active error states tracked by operation key.
    pub(crate) error_states: HashMap<String, ErrorState>,
    /// Default max retries per operation.
    default_max_retries: u32,
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self {
            error_states: HashMap::new(),
            default_max_retries: 3,
        }
    }

    /// Register a new error state for an operation.
    pub fn register_error(
        &mut self,
        operation: impl Into<String>,
        error_message: impl Into<String>,
        error_class: ErrorClass,
    ) {
        let operation = operation.into();
        let state = new_error_state(
            operation.clone(),
            error_message.into(),
            error_class,
            self.default_max_retries,
            requires_rollback(error_class),
        );
        self.error_states.insert(operation, state);
    }
}

impl RecoveryManager {
    /// Get the current error state for an operation.
    pub fn get_error_state(&self, operation: &str) -> Option<&ErrorState> {
        self.error_states.get(operation)
    }
}
