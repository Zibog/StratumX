// Authority Core Diagnostics
//
// Typed diagnostics for authority container operations.
// All authority operations should publish diagnostics through this module.

use std::fmt;

/// Diagnostic event severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// Diagnostic event for authority operations
#[derive(Debug, Clone)]
pub struct AuthorityDiagnostic {
    pub severity: DiagnosticSeverity,
    pub authority_type: AuthorityType,
    pub operation: String,
    pub message: String,
    pub recovery_hint: Option<String>,
}

/// Type of authority container
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityType {
    Audio,
    Material,
    Terrain,
}

impl fmt::Display for AuthorityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthorityType::Audio => write!(f, "Audio"),
            AuthorityType::Material => write!(f, "Material"),
            AuthorityType::Terrain => write!(f, "Terrain"),
        }
    }
}

impl AuthorityDiagnostic {
    /// Create an info diagnostic
    pub fn info(
        authority_type: AuthorityType,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: DiagnosticSeverity::Info,
            authority_type,
            operation: operation.into(),
            message: message.into(),
            recovery_hint: None,
        }
    }

    /// Create a warning diagnostic
    pub fn warning(
        authority_type: AuthorityType,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            authority_type,
            operation: operation.into(),
            message: message.into(),
            recovery_hint: None,
        }
    }

    /// Create an error diagnostic
    pub fn error(
        authority_type: AuthorityType,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            authority_type,
            operation: operation.into(),
            message: message.into(),
            recovery_hint: None,
        }
    }

    /// Add a recovery hint to the diagnostic
    pub fn with_recovery_hint(mut self, hint: impl Into<String>) -> Self {
        self.recovery_hint = Some(hint.into());
        self
    }
}

impl fmt::Display for AuthorityDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}] {} - {}: {}",
            self.severity, self.authority_type, self.operation, self.message
        )?;
        if let Some(hint) = &self.recovery_hint {
            write!(f, " (Recovery: {})", hint)?;
        }
        Ok(())
    }
}

/// Diagnostic publisher trait
///
/// Authority containers should implement this to publish diagnostics
pub trait DiagnosticPublisher {
    /// Publish a diagnostic event
    fn publish_diagnostic(&self, diagnostic: AuthorityDiagnostic);
}

/// Default diagnostic publisher that prints to stderr
pub struct StderrDiagnosticPublisher;

impl DiagnosticPublisher for StderrDiagnosticPublisher {
    fn publish_diagnostic(&self, diagnostic: AuthorityDiagnostic) {
        eprintln!("{}", diagnostic);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let diag = AuthorityDiagnostic::info(
            AuthorityType::Audio,
            "initialize",
            "Audio authority initialized successfully",
        );
        assert_eq!(diag.severity, DiagnosticSeverity::Info);
        assert_eq!(diag.authority_type, AuthorityType::Audio);
        assert_eq!(diag.operation, "initialize");
    }

    #[test]
    fn test_diagnostic_with_recovery_hint() {
        let diag = AuthorityDiagnostic::error(
            AuthorityType::Material,
            "initialize",
            "Failed to initialize material registry",
        )
        .with_recovery_hint("Ensure material database is accessible");

        assert!(diag.recovery_hint.is_some());
        assert_eq!(
            diag.recovery_hint.unwrap(),
            "Ensure material database is accessible"
        );
    }

    #[test]
    fn test_diagnostic_display() {
        let diag = AuthorityDiagnostic::warning(
            AuthorityType::Terrain,
            "add_layer",
            "Layer limit approaching",
        )
        .with_recovery_hint("Consider merging layers");

        let display = format!("{}", diag);
        assert!(display.contains("Warning"));
        assert!(display.contains("Terrain"));
        assert!(display.contains("add_layer"));
        assert!(display.contains("Recovery"));
    }
}
