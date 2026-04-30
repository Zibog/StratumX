use std::fmt;

/// Classification of errors for recovery purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// Invalid lifecycle state transition.
    InvalidLifecycleTransition,
    /// Resource not found.
    ResourceNotFound,
    /// Resource already exists.
    ResourceAlreadyExists,
    /// Invalid operation parameters.
    InvalidParameters,
    /// Preview connection failure.
    PreviewConnectionFailure,
    /// Registry corruption.
    RegistryCorruption,
    /// External dependency failure.
    ExternalDependencyFailure,
    /// Timeout.
    Timeout,
    /// Unknown error.
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

/// Tracks the error state for a single operation.
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

pub(crate) fn new_error_state(
    operation: String,
    last_error: String,
    error_class: ErrorClass,
    max_retries: u32,
    requires_rollback: bool,
) -> ErrorState {
    ErrorState {
        operation,
        last_error,
        error_class,
        retry_count: 0,
        max_retries,
        requires_rollback,
        cleared: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_class_display_stays_stable() {
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
