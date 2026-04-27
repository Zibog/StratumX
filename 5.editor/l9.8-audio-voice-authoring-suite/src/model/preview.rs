//! Preview result types for audio audibility, obstruction/occlusion, transitions, and subtitle legality.

use serde::{Deserialize, Serialize};

use crate::model::ObjectHandle;

/// Audibility Preview Result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudibilityPreview {
    pub listener_position: [i32; 3],
    pub audible_sources: Vec<ObjectHandle>,
    pub occluded_sources: Vec<ObjectHandle>,
    pub distance_attenuated: Vec<(ObjectHandle, u32)>,
}

/// Obstruction vs Occlusion Result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObstructionOcclusionResult {
    pub source: ObjectHandle,
    pub listener_position: [i32; 3],
    pub obstructed: bool,
    pub occluded: bool,
    pub obstruction_factor: u32,
    pub occlusion_factor: u32,
}

/// Indoor/Outdoor Transition Result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionResult {
    pub from_zone: ObjectHandle,
    pub to_zone: ObjectHandle,
    pub transition_valid: bool,
    pub reverb_blend_curve: Vec<u32>,
}

/// Voice/Subtitle Legality Result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSubtitleLegality {
    pub dialogue_id: String,
    pub voice_present: bool,
    pub subtitle_present: bool,
    pub locale_legal: bool,
    pub missing_locales: Vec<String>,
}
