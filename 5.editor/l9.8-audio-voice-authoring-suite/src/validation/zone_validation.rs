//! Validation rules for audio zones.

use crate::model::AudioZone;

/// Validate an audio zone for consistency.
pub fn validate_zone(zone: &AudioZone) -> Vec<String> {
    let mut errors = Vec::new();

    if zone.name.is_empty() {
        errors.push("AudioZone name must not be empty".to_string());
    }

    if let Some(ref reverb) = zone.reverb_profile {
        if reverb.is_empty() {
            errors.push("reverb_profile must not be empty if present".to_string());
        }
    }

    errors
}
