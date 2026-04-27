// Tool Session Diagnostics Integration
//
// Integrates diagnostics into tool session executors.

/// Diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// Publish executor diagnostic
pub fn publish_executor_diagnostic(
    domain: &str,
    operation: &str,
    severity: DiagnosticSeverity,
    message: impl Into<String>,
) {
    let msg = message.into();
    let level = match severity {
        DiagnosticSeverity::Info => "INFO",
        DiagnosticSeverity::Warning => "WARN",
        DiagnosticSeverity::Error => "ERROR",
    };
    eprintln!("[SESSION][{}][{}][{}] {}", level, domain, operation, msg);
}
