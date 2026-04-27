// Recovery strategies and error classification

use std::fmt;

/// Recovery strategy for authority operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Retry the operation immediately
    Retry,
    /// Retry after a delay (milliseconds)
    RetryAfterDelay(u64),
    /// Reset the authority to initial state
    Reset,
    /// Reinitialize the authority
    Reinitialize,
    /// Skip the operation and continue
    Skip,
    /// Abort the operation and propagate error
    Abort,
    /// Rollback to previous consistent state
    Rollback,
    /// Manual intervention required
    ManualIntervention(String),
}

/// Classification of errors for recovery purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// Invalid lifecycle state transition
    InvalidLifecycleTransition,
    /// Resource not found
    ResourceNotFound,
    /// Resource already exists
    ResourceAlreadyExists,
    /// Invalid operation parameters
    InvalidParameters,
    /// Preview connection failure
    PreviewConnectionFailure,
    /// Registry corruption
    RegistryCorruption,
    /// External dependency failure
    ExternalDependencyFailure,
    /// Timeout
    Timeout,
    /// Unknown error
    Unknown,
}

impl fmt::Display for ErrorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorClass::InvalidLifecycleTransition => write!(f, "InvalidLifecycleTransition"),
            ErrorClass::ResourceNotFound => write!(f, "ResourceNotFound"),
            ErrorClass::ResourceAlreadyExists => write!(f, "ResourceAlreadyExists"),
            ErrorClass::InvalidParameters => write!(f, "InvalidParameters"),
            ErrorClass::PreviewConnectionFailure => write!(f, "PreviewConnectionFailure"),
            ErrorClass::RegistryCorruption => write!(f, "RegistryCorruption"),
            ErrorClass::ExternalDependencyFailure => write!(f, "ExternalDependencyFailure"),
            ErrorClass::Timeout => write!(f, "Timeout"),
            ErrorClass::Unknown => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_class_display() {
        assert_eq!(
            format!("{}", ErrorClass::InvalidLifecycleTransition),
            "InvalidLifecycleTransition"
        );
        assert_eq!(
            format!("{}", ErrorClass::ResourceNotFound),
            "ResourceNotFound"
        );
    }
}
