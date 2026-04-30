use super::errors::ErrorClass;

/// Recovery strategy for authority operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Retry the operation immediately.
    Retry,
    /// Retry after a delay in milliseconds.
    RetryAfterDelay(u64),
    /// Reset the authority to initial state.
    Reset,
    /// Reinitialize the authority.
    Reinitialize,
    /// Skip the operation and continue.
    Skip,
    /// Abort the operation and propagate the error.
    Abort,
    /// Rollback to the previous consistent state.
    Rollback,
    /// Manual intervention is required.
    ManualIntervention(String),
}

/// Recovery target for a specific error condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryTarget {
    pub error_class: ErrorClass,
    pub strategy: RecoveryStrategy,
    pub description: String,
}

impl RecoveryTarget {
    /// Create a new recovery target.
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

    /// Get the default recovery target for an error class.
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
