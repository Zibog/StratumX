//! Preview operations for audio authoring.
//!
//! Contains preview-oriented logic delegating to the AudioRegistry.

use crate::model::{
    AudibilityPreview, AudioRegistry, ObjectHandle, ObstructionOcclusionResult, TransitionResult,
    VoiceSubtitleLegality,
};

/// Preview audibility from a listener position.
pub fn preview_audibility(
    registry: &AudioRegistry,
    listener_position: [f32; 3],
    max_distance: f32,
) -> AudibilityPreview {
    registry.preview_audibility(listener_position, max_distance)
}

/// Preview obstruction and occlusion for a source.
pub fn preview_obstruction_occlusion(
    registry: &AudioRegistry,
    source: ObjectHandle,
    listener_position: [f32; 3],
) -> Result<ObstructionOcclusionResult, String> {
    registry.preview_obstruction_occlusion(source, listener_position)
}

/// Preview a zone transition.
pub fn preview_transition(
    registry: &AudioRegistry,
    from_zone: ObjectHandle,
    to_zone: ObjectHandle,
) -> Result<TransitionResult, String> {
    registry.preview_transition(from_zone, to_zone)
}

/// Preview voice/subtitle legality.
pub fn preview_voice_subtitle_legality(
    registry: &AudioRegistry,
    dialogue_id: String,
) -> VoiceSubtitleLegality {
    registry.preview_voice_subtitle_legality(dialogue_id)
}
