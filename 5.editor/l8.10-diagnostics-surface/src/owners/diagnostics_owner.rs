//! Diagnostics Owner — canonical container for diagnostic state.
//!
//! Phase 05: Types moved to model/diagnostics_types.rs.
//! This file contains ONLY the DiagnosticsOwner container struct.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export types from model for backward compatibility
pub use crate::model::diagnostics_types::{
    DiagnosticMessage, DiagnosticSource, FailureCode, Severity, TraceId, TraceLineage,
};

/// Diagnostics state container
///
/// Owns diagnostics-level state including messages, trace lineage,
/// failure codes, and recovery action mappings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsOwner {
    /// Diagnostic messages
    pub messages: Vec<DiagnosticMessage>,

    /// Trace lineage for error tracking
    pub trace_lineage: HashMap<TraceId, TraceLineage>,

    /// Failure codes
    pub failure_codes: Vec<FailureCode>,

    /// Recovery actions mapped by failure code
    pub recovery_actions: HashMap<FailureCode, String>,
}

impl DiagnosticsOwner {
    /// Creates a new empty diagnostics state
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            trace_lineage: HashMap::new(),
            failure_codes: Vec::new(),
            recovery_actions: HashMap::new(),
        }
    }
}

impl Default for DiagnosticsOwner {
    fn default() -> Self {
        Self::new()
    }
}
