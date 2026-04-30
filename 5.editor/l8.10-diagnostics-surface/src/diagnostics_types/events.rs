// Diagnostic event types
//
// Extracted from diagnostics_types.rs during Phase 3 cleanup.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Severity, TraceId};
use crate::ArtifactRef;

/// Diagnostic message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticMessage {
    /// Message severity
    pub severity: Severity,

    /// Message text
    pub message: String,

    /// Diagnostic source
    pub source: DiagnosticSource,

    /// Trace ID for lineage tracking
    pub trace_id: Option<TraceId>,

    /// Artifact reference
    pub artifact_ref: Option<ArtifactRef>,

    /// Timestamp
    pub timestamp: u64,
}

impl DiagnosticMessage {
    /// Creates a new diagnostic message
    pub fn new(severity: Severity, message: String, source: DiagnosticSource) -> Self {
        Self {
            severity,
            message,
            source,
            trace_id: None,
            artifact_ref: None,
            timestamp: 0,
        }
    }

    /// Sets the trace ID
    pub fn with_trace_id(mut self, trace_id: TraceId) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    /// Sets the artifact reference
    pub fn with_artifact_ref(mut self, artifact_ref: ArtifactRef) -> Self {
        self.artifact_ref = Some(artifact_ref);
        self
    }

    /// Sets the timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// Diagnostic source
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticSource {
    /// Command spine
    CommandSpine,

    /// Tooling route
    ToolingRoute(String),

    /// SDK packet
    SdkPacket(String),

    /// Engine truth owner
    EngineTruthOwner(String),

    /// Editor component
    EditorComponent(String),
}

/// Trace lineage for error tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLineage {
    /// Trace ID
    pub trace_id: TraceId,

    /// Parent trace ID (if any)
    pub parent_trace_id: Option<TraceId>,

    /// Operation that created this trace
    pub operation: String,

    /// Timestamp
    pub timestamp: u64,

    /// Additional context
    pub context: HashMap<String, String>,
}

impl TraceLineage {
    /// Creates a new trace lineage
    pub fn new(trace_id: TraceId, operation: String, timestamp: u64) -> Self {
        Self {
            trace_id,
            parent_trace_id: None,
            operation,
            timestamp,
            context: HashMap::new(),
        }
    }

    /// Sets the parent trace ID
    pub fn with_parent(mut self, parent_trace_id: TraceId) -> Self {
        self.parent_trace_id = Some(parent_trace_id);
        self
    }

    /// Adds context information
    pub fn add_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }
}
