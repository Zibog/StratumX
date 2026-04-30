// Diagnostic identifier types
//
// Extracted from diagnostics_types.rs during Phase 3 cleanup.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl Default for TraceId {
    fn default() -> Self {
        Self::from_uuid(Uuid::nil())
    }
}

// NOTE: Default uses the nil UUID as an explicit "no trace" sentinel.
// Use TraceId::new() when a call site needs a real fresh trace ID.

impl From<Uuid> for TraceId {
    fn from(id: Uuid) -> Self {
        Self(id)
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
