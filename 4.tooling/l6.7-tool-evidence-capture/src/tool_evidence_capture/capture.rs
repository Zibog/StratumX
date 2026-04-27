// TOOLING L6.7: EVIDENCE CAPTURE - Core DTOs
// Implements compare/capture/recover chain for certification

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub bundle_id: String,
    pub pack_id: String,
    pub scenario_id: String,
    pub build_profile: String,
    pub schema_revision: String,
    pub timestamp: u64,
    pub artifacts: Vec<ArtifactRef>,
    pub verdict: EvidenceVerdict,
    pub operator_signoff: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub artifact_id: String,
    pub artifact_type: ArtifactType,
    pub path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArtifactType {
    FrameCapture,
    AudioCapture,
    StateSnapshot,
    DiagnosticsLog,
    CompareDigest,
    RecoveryTrace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvidenceVerdict {
    Green,  // Pass
    Orange, // Warning/Degraded
    Red,    // Failure
}
