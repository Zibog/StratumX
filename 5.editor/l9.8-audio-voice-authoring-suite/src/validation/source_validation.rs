//! Validation rules for audio sources.

use crate::model::AudioSource;

/// Validate an audio source for consistency.
pub fn validate_source(source: &AudioSource) -> Vec<String> {
    let mut errors = Vec::new();

    if source.name.is_empty() {
        errors.push("AudioSource name must not be empty".to_string());
    }

    if source.priority == 0 {
        errors.push("AudioSource priority must be greater than 0".to_string());
    }

    if let Some(ref profile) = source.acoustic_profile {
        if profile.is_empty() {
            errors.push("acoustic_profile must not be empty if present".to_string());
        }
    }

    if let Some(ref emitter) = source.emitter_class {
        if emitter.is_empty() {
            errors.push("emitter_class must not be empty if present".to_string());
        }
    }

    errors
}
