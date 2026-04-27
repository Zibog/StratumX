//! StratumX Editor - L9.8 Audio Voice Authoring Suite
//!
//! Provides audio authoring capabilities including source management,
//! zone configuration, ducking policies, and audibility preview.

pub mod api;
pub mod audio_registry_state;
mod audio_authoring_service;
mod model;
mod runtime;
mod validation;

// Re-export public API at crate root for convenience.
// Using explicit re-exports to avoid ambiguous glob re-exports between api and audio_registry_state.
pub use api::{
    AudibilityPreview, AudioAuthoringService, AudioRegistry, AudioVoiceAuthoringSuite,
    DuckingPolicy, ObjectHandle, ObstructionOcclusionResult, TransitionResult,
    VoiceSubtitleLegality, preview_audibility, preview_obstruction_occlusion, preview_transition,
    preview_voice_subtitle_legality, validate_source, validate_zone,
};
pub use audio_registry_state::{
    AcousticProfile, AcousticProfileId, AudioSourceId, AudioSourceType, AudioZoneId,
};
// AudioSource and AudioZone come from api:: (model) which is the authoritative public API
pub use api::{AudioSource, AudioZone};
