// Recovery policies and context

use super::strategies::{ErrorClass, RecoveryStrategy};

/// Recovery target for specific error conditions
#[derive(Debug, Clone)]
pub struct RecoveryTarget {
    pub error_class: ErrorClass,
    pub strategy: RecoveryStrategy,
    pub description: String,
}

impl RecoveryTarget {
    /// Create a new recovery target
    pub fn new(
        error_class: ErrorClass,
        strategy: RecoveryStrategy,
        description: impl Into<String>,
    ) -> Self {
        Self {
            error_class,
            strategy,
            description: description.into(),
        }
    }

    /// Get the default recovery target for an error class
    pub fn default_for_error_class(error_class: ErrorClass) -> Self {
        match error_class {
            ErrorClass::InvalidLifecycleTransition => Self::new(
                error_class,
                RecoveryStrategy::Abort,
                "Invalid lifecycle transition - check state before operation",
            ),
            ErrorClass::ResourceNotFound => Self::new(
                error_class,
                RecoveryStrategy::Skip,
                "Resource not found - skip operation or create resource",
            ),
            ErrorClass::ResourceAlreadyExists => Self::new(
                error_class,
                RecoveryStrategy::Skip,
                "Resource already exists - skip creation or use existing",
            ),
            ErrorClass::InvalidParameters => Self::new(
                error_class,
                RecoveryStrategy::Abort,
                "Invalid parameters - validate input before retry",
            ),
            ErrorClass::PreviewConnectionFailure => Self::new(
                error_class,
                RecoveryStrategy::RetryAfterDelay(1000),
                "Preview connection failed - retry after delay",
            ),
            ErrorClass::RegistryCorruption => Self::new(
                error_class,
                RecoveryStrategy::Reinitialize,
                "Registry corruption detected - reinitialize authority",
            ),
            ErrorClass::ExternalDependencyFailure => Self::new(
                error_class,
                RecoveryStrategy::RetryAfterDelay(2000),
                "External dependency failure - retry after delay",
            ),
            ErrorClass::Timeout => Self::new(
                error_class,
                RecoveryStrategy::Retry,
                "Operation timeout - retry immediately",
            ),
            ErrorClass::Unknown => Self::new(
                error_class,
                RecoveryStrategy::ManualIntervention(
                    "Unknown error - manual investigation required".to_string(),
                ),
                "Unknown error - manual intervention required",
            ),
        }
    }
}

/// Recovery context for authority operations
pub struct RecoveryContext {
    pub operation: String,
    pub error_message: String,
    pub error_class: ErrorClass,
    pub retry_count: u32,
    pub max_retries: u32,
}

impl RecoveryContext {
    /// Create a new recovery context
    pub fn new(operation: impl Into<String>, error_message: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            error_message: error_message.into(),
            error_class: ErrorClass::Unknown,
            retry_count: 0,
            max_retries: 3,
        }
    }

    /// Set the error class
    pub fn with_error_class(mut self, error_class: ErrorClass) -> Self {
        self.error_class = error_class;
        self
    }

    /// Set the max retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }

    /// Check if max retries exceeded
    pub fn is_max_retries_exceeded(&self) -> bool {
        self.retry_count >= self.max_retries
    }

    /// Get the recovery target for this context
    pub fn get_recovery_target(&self) -> RecoveryTarget {
        let mut target = RecoveryTarget::default_for_error_class(self.error_class);

        // Override strategy if max retries exceeded
        if self.is_max_retries_exceeded() {
            match target.strategy {
                RecoveryStrategy::Retry | RecoveryStrategy::RetryAfterDelay(_) => {
                    target.strategy = RecoveryStrategy::Abort;
                    target.description = format!(
                        "{} (max retries {} exceeded)",
                        target.description, self.max_retries
                    );
                }
                _ => {}
            }
        }

        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_target_creation() {
        let target = RecoveryTarget::new(
            ErrorClass::ResourceNotFound,
            RecoveryStrategy::Skip,
            "Resource not found",
        );
        assert_eq!(target.error_class, ErrorClass::ResourceNotFound);
        assert_eq!(target.strategy, RecoveryStrategy::Skip);
    }

    #[test]
    fn test_default_recovery_targets() {
        let target =
            RecoveryTarget::default_for_error_class(ErrorClass::InvalidLifecycleTransition);
        assert_eq!(target.strategy, RecoveryStrategy::Abort);

        let target = RecoveryTarget::default_for_error_class(ErrorClass::PreviewConnectionFailure);
        assert!(matches!(
            target.strategy,
            RecoveryStrategy::RetryAfterDelay(_)
        ));
    }

    #[test]
    fn test_recovery_context() {
        let mut context = RecoveryContext::new("initialize", "Failed to initialize")
            .with_error_class(ErrorClass::ExternalDependencyFailure)
            .with_max_retries(2);

        assert_eq!(context.retry_count, 0);
        assert!(!context.is_max_retries_exceeded());

        context.increment_retry();
        context.increment_retry();
        assert!(context.is_max_retries_exceeded());

        let target = context.get_recovery_target();
        assert_eq!(target.strategy, RecoveryStrategy::Abort);
    }
}

/// Manages retry, rollback, and error state clearing for authority operations
pub struct RecoveryManager {
    /// Active error states tracked by operation key
    error_states: std::collections::HashMap<String, ErrorState>,
    /// Default max retries per operation
    default_max_retries: u32,
}

/// Tracks the error state for a single operation
#[derive(Debug, Clone)]
pub struct ErrorState {
    pub operation: String,
    pub last_error: String,
    pub error_class: ErrorClass,
    pub retry_count: u32,
    pub max_retries: u32,
    pub requires_rollback: bool,
    pub cleared: bool,
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self {
            error_states: std::collections::HashMap::new(),
            default_max_retries: 3,
        }
    }

    /// Register a new error state for an operation
    pub fn register_error(
        &mut self,
        operation: impl Into<String>,
        error_message: impl Into<String>,
        error_class: ErrorClass,
    ) {
        let operation = operation.into();
        self.error_states.insert(
            operation.clone(),
            ErrorState {
                operation,
                last_error: error_message.into(),
                error_class,
                retry_count: 0,
                max_retries: self.default_max_retries,
                requires_rollback: matches!(
                    error_class,
                    ErrorClass::RegistryCorruption | ErrorClass::InvalidLifecycleTransition
                ),
                cleared: false,
            },
        );
    }

    /// Attempt to retry an operation
    /// Returns Some(RecoveryStrategy) if retry is allowed, None if max retries exceeded
    pub fn try_retry(&mut self, operation: &str) -> Option<RecoveryStrategy> {
        let state = self.error_states.get_mut(operation)?;

        if state.retry_count >= state.max_retries {
            state.requires_rollback = true;
            return None;
        }

        state.retry_count += 1;

        // Determine retry strategy based on error class
        match state.error_class {
            ErrorClass::Timeout => Some(RecoveryStrategy::Retry),
            ErrorClass::PreviewConnectionFailure => Some(RecoveryStrategy::RetryAfterDelay(1000)),
            ErrorClass::ExternalDependencyFailure => Some(RecoveryStrategy::RetryAfterDelay(2000)),
            ErrorClass::ResourceNotFound | ErrorClass::ResourceAlreadyExists => {
                Some(RecoveryStrategy::Skip)
            }
            _ => None, // Don't retry unknown/invalid states
        }
    }

    /// Rollback an operation to its previous consistent state
    pub fn rollback(&mut self, operation: &str) -> Result<(), String> {
        let state = self
            .error_states
            .get_mut(operation)
            .ok_or_else(|| format!("No error state found for operation: {}", operation))?;

        state.requires_rollback = true;
        // Mark for rollback - caller should perform the actual rollback
        Ok(())
    }

    /// Clear the error state for an operation
    pub fn clear_error(&mut self, operation: &str) -> Result<(), String> {
        let state = self
            .error_states
            .get_mut(operation)
            .ok_or_else(|| format!("No error state found for operation: {}", operation))?;

        state.cleared = true;
        Ok(())
    }

    /// Check if an operation's error has been cleared
    pub fn is_error_cleared(&self, operation: &str) -> bool {
        self.error_states
            .get(operation)
            .map(|s| s.cleared)
            .unwrap_or(true)
    }

    /// Get the current error state for an operation
    pub fn get_error_state(&self, operation: &str) -> Option<&ErrorState> {
        self.error_states.get(operation)
    }

    /// Remove a cleared error state
    pub fn purge_cleared(&mut self, operation: &str) {
        self.error_states.remove(operation);
    }
}

#[cfg(test)]
mod recovery_manager_tests {
    use super::*;

    #[test]
    fn test_recovery_manager_retry() {
        let mut manager = RecoveryManager::new();
        manager.register_error("init", "timeout", ErrorClass::Timeout);

        // First 3 retries should succeed
        assert!(manager.try_retry("init").is_some());
        assert!(manager.try_retry("init").is_some());
        assert!(manager.try_retry("init").is_some());

        // 4th attempt should fail (max retries exceeded)
        assert!(manager.try_retry("init").is_none());
    }

    #[test]
    fn test_recovery_manager_rollback() {
        let mut manager = RecoveryManager::new();
        manager.register_error("init", "corruption", ErrorClass::RegistryCorruption);

        assert!(manager.rollback("init").is_ok());
        let state = manager.get_error_state("init").unwrap();
        assert!(state.requires_rollback);
    }

    #[test]
    fn test_recovery_manager_clear_error() {
        let mut manager = RecoveryManager::new();
        manager.register_error("init", "timeout", ErrorClass::Timeout);

        assert!(!manager.is_error_cleared("init"));
        manager.clear_error("init").unwrap();
        assert!(manager.is_error_cleared("init"));

        manager.purge_cleared("init");
        assert!(manager.get_error_state("init").is_none());
    }

    #[test]
    fn test_recovery_manager_rollback_required_for_corruption() {
        let mut manager = RecoveryManager::new();
        manager.register_error("init", "corruption", ErrorClass::RegistryCorruption);

        let state = manager.get_error_state("init").unwrap();
        assert!(state.requires_rollback);

        // ResourceNotFound does NOT require rollback
        manager.register_error("query", "not found", ErrorClass::ResourceNotFound);
        let state = manager.get_error_state("query").unwrap();
        assert!(!state.requires_rollback);
    }
}
