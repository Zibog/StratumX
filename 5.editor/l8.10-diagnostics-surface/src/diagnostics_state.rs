//! Diagnostics State - Re-exports from owners module
//!
//! This module provides backward compatibility by re-exporting types from the owners module.

pub use crate::owners::diagnostics_owner::{
    DiagnosticMessage, DiagnosticSource, DiagnosticsOwner as DiagnosticsState, FailureCode,
    Severity, TraceId, TraceLineage,
};
