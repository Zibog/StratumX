//! Public API - re-exports and public entry points.
//!
//! External consumers should import from this module.

pub use crate::model::{
    AudibilityPreview, AudioRegistry, AudioSource, AudioVoiceAuthoringSuite, AudioZone,
    DuckingPolicy, ObjectHandle, ObstructionOcclusionResult, TransitionResult,
    VoiceSubtitleLegality,
};

pub use crate::runtime::{
    preview_audibility, preview_obstruction_occlusion, preview_transition,
    preview_voice_subtitle_legality, AudioAuthoringService,
};
pub use crate::validation::{validate_source, validate_zone};
