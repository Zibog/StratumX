// Diagnostic severity levels
//
// Extracted from diagnostics_types.rs during Phase 3 cleanup.

use serde::{Deserialize, Serialize};

/// Diagnostic severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Informational message
    Info,

    /// Warning message
    Warning,

    /// Error message
    Error,

    /// Critical error message
    Critical,
}
