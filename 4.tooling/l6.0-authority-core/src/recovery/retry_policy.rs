use super::errors::ErrorClass;
use super::evidence_recovery::RecoveryManager;
use super::policy_ids::{RecoveryStrategy, RecoveryTarget};

/// Recovery context for authority operations.
pub struct RecoveryContext {
    pub operation: String,
    pub error_message: String,
    pub error_class: ErrorClass,
    pub retry_count: u32,
    pub max_retries: u32,
}

fn retry_exhausted_description(description: &str, max_retries: u32) -> String {
    format!("{} (max retries {} exceeded)", description, max_retries)
}

impl RecoveryContext {
    /// Create a new recovery context.
    pub fn new(operation: impl Into<String>, error_message: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            error_message: error_message.into(),
            error_class: ErrorClass::Unknown,
            retry_count: 0,
            max_retries: 3,
        }
    }

    /// Set the error class.
    pub fn with_error_class(mut self, error_class: ErrorClass) -> Self {
        self.error_class = error_class;
        self
    }

    /// Set the max retries.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Increment retry count.
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }

    /// Check if max retries exceeded.
    pub fn is_max_retries_exceeded(&self) -> bool {
        self.retry_count >= self.max_retries
    }

    /// Get the recovery target for this context.
    pub fn get_recovery_target(&self) -> RecoveryTarget {
        let mut target = RecoveryTarget::default_for_error_class(self.error_class);

        if self.is_max_retries_exceeded() {
            match target.strategy {
                RecoveryStrategy::Retry | RecoveryStrategy::RetryAfterDelay(_) => {
                    target.strategy = RecoveryStrategy::Abort;
                    target.description =
                        retry_exhausted_description(&target.description, self.max_retries);
                }
                _ => {}
            }
        }

        target
    }
}

impl RecoveryManager {
    /// Attempt to retry an operation.
    /// Returns `Some(RecoveryStrategy)` if retry is allowed, `None` if max retries exceeded.
    pub fn try_retry(&mut self, operation: &str) -> Option<RecoveryStrategy> {
        let state = self.error_states.get_mut(operation)?;

        if state.retry_count >= state.max_retries {
            state.requires_rollback = true;
            return None;
        }

        state.retry_count += 1;

        match state.error_class {
            ErrorClass::Timeout => Some(RecoveryStrategy::Retry),
            ErrorClass::PreviewConnectionFailure => Some(RecoveryStrategy::RetryAfterDelay(1000)),
            ErrorClass::ExternalDependencyFailure => Some(RecoveryStrategy::RetryAfterDelay(2000)),
            ErrorClass::ResourceNotFound | ErrorClass::ResourceAlreadyExists => {
                Some(RecoveryStrategy::Skip)
            }
            _ => None,
        }
    }
}
