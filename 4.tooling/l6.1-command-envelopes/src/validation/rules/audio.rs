// Audio validation rules

use crate::validation::errors::ValidationResult;

/// Validate audio source parameters
pub fn validate_audio_source(source_id: &str, position: &[f32; 3]) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if source_id.is_empty() {
        result = result.add_error("source_id cannot be empty");
    }
    if !position[0].is_finite() || !position[1].is_finite() || !position[2].is_finite() {
        result = result.add_error("position must be finite");
    }
    result
}

/// Validate audio binding parameters
pub fn validate_audio_binding(id: &str, binding_value: &str) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if id.is_empty() {
        result = result.add_error("id cannot be empty");
    }
    if binding_value.is_empty() {
        result = result.add_error("binding value cannot be empty");
    }
    result
}
