// Diagnostics domain types
//
// Phase 05: Moved from owners/diagnostics_owner.rs to separate types from container.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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

/// Trace ID for lineage tracking
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId(pub Uuid);

impl TraceId {
    /// Creates a new trace ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a trace ID from a UUID
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

// NOTE: TraceId intentionally does NOT implement Default.
// A random UUID as default would create fake trace IDs that match nothing.
// Use TraceId::new() for a real trace, or TraceId::from_uuid(Uuid::nil())
// for an explicit "no trace" sentinel.

impl From<Uuid> for TraceId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
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

/// Failure code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FailureCode {
    /// Material profile invalid
    MaterialProfileInvalid,

    /// Material binding failed
    MaterialBindingFailed,

    /// Terrain simulation failed
    TerrainSimulationFailed,

    /// No workspace available
    NoWorkspaceAvailable,

    /// World open failed
    WorldOpenFailed,

    /// World save failed
    WorldSaveFailed,

    /// World close failed
    WorldCloseFailed,

    /// Hardware floor insufficient
    HardwareFloorInsufficient,

    /// Quality degradation rung exhausted
    QualityDegradationExhausted,

    /// Runtime initialization failed
    RuntimeInitFailed,

    /// Runtime execution failed
    RuntimeExecFailed,

    /// Diagnostics capture failed
    DiagnosticsCaptureFailed,

    /// Build failed
    BuildFailed,

    /// Release certification failed
    ReleaseCertFailed,
}

impl FailureCode {
    /// Returns a user-facing message for this failure code
    pub fn to_user_message(&self) -> &'static str {
        match self {
            Self::MaterialProfileInvalid => "Material profile is invalid",
            Self::MaterialBindingFailed => "Material binding failed",
            Self::TerrainSimulationFailed => "Terrain simulation failed",
            Self::NoWorkspaceAvailable => "No workspace available",
            Self::WorldOpenFailed => "Failed to open world",
            Self::WorldSaveFailed => "Failed to save world",
            Self::WorldCloseFailed => "Failed to close world",
            Self::HardwareFloorInsufficient => "Hardware requirements not met",
            Self::QualityDegradationExhausted => "Quality degradation limit reached",
            Self::RuntimeInitFailed => "Runtime initialization failed",
            Self::RuntimeExecFailed => "Runtime execution failed",
            Self::DiagnosticsCaptureFailed => "Diagnostics capture failed",
            Self::BuildFailed => "Build failed",
            Self::ReleaseCertFailed => "Release certification failed",
        }
    }
}
