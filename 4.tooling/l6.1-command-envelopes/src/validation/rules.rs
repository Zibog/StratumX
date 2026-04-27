// Validation rules and helper functions

use super::errors::ValidationResult;

/// Validate project name
pub fn validate_project_name(name: &str) -> ValidationResult {
    if name.is_empty() {
        return ValidationResult::Invalid {
            errors: vec!["project_name cannot be empty".to_string()],
        };
    }
    if name.len() > 64 {
        return ValidationResult::Invalid {
            errors: vec!["project_name cannot exceed 64 characters".to_string()],
        };
    }
    ValidationResult::Valid
}

/// Validate project creation fields
pub fn validate_project_create(
    project_name: &str,
    project_root: &str,
    world_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if project_name.is_empty() {
        result = result.add_error("project_name cannot be empty");
    }
    if project_root.is_empty() {
        result = result.add_error("project_root cannot be empty");
    }
    if world_name.is_empty() {
        result = result.add_error("world_name cannot be empty");
    }
    result
}

/// Validate a path field
pub fn validate_path(path: &str, field_name: &str) -> ValidationResult {
    if path.is_empty() {
        ValidationResult::Invalid {
            errors: vec![format!("{} cannot be empty", field_name)],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate a non-empty string field
pub fn validate_non_empty(value: &str, field_name: &str) -> ValidationResult {
    if value.is_empty() {
        ValidationResult::Invalid {
            errors: vec![format!("{} cannot be empty", field_name)],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate terrain sculpt parameters
pub fn validate_terrain_sculpt(
    position: &[f32; 2],
    radius: f32,
    strength: f32,
) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if radius <= 0.0 {
        result = result.add_error("radius must be positive");
    }
    if !(0.0..=1.0).contains(&strength) {
        result = result.add_error("strength must be between 0.0 and 1.0");
    }
    if !position[0].is_finite() || !position[1].is_finite() {
        result = result.add_error("position must be finite");
    }
    result
}

/// Validate terrain layer material parameters
pub fn validate_terrain_layer_material(
    albedo_texture_path: &str,
    uv_scale: &[f32; 2],
) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if albedo_texture_path.is_empty() {
        result = result.add_error("albedo_texture_path cannot be empty");
    }
    if uv_scale[0] <= 0.0 || uv_scale[1] <= 0.0 {
        result = result.add_error("uv_scale must be positive");
    }
    result
}

/// Validate terrain hole parameters
pub fn validate_terrain_hole(position: &[f32; 2], radius: f32) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if radius <= 0.0 {
        result = result.add_error("radius must be positive");
    }
    if !position[0].is_finite() || !position[1].is_finite() {
        result = result.add_error("position must be finite");
    }
    result
}

/// Validate time of day (0.0-24.0)
pub fn validate_time_of_day(hours: f32) -> ValidationResult {
    if !(0.0..24.0).contains(&hours) {
        ValidationResult::Invalid {
            errors: vec!["time_of_day_hours must be between 0.0 and 24.0".to_string()],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate a value is within a range
pub fn validate_range(value: f32, min: f32, max: f32, field_name: &str) -> ValidationResult {
    if value < min || value > max {
        ValidationResult::Invalid {
            errors: vec![format!(
                "{} must be between {} and {}",
                field_name, min, max
            )],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate a value is positive
pub fn validate_positive(value: f32, field_name: &str) -> ValidationResult {
    if value < 0.0 {
        ValidationResult::Invalid {
            errors: vec![format!("{} must be positive", field_name)],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate material duplicate parameters
pub fn validate_material_duplicate(
    source_material_id: &str,
    target_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if source_material_id.is_empty() {
        result = result.add_error("source_material_id cannot be empty");
    }
    if target_name.is_empty() {
        result = result.add_error("target_name cannot be empty");
    }
    result
}

/// Validate material binding parameters
pub fn validate_material_binding(material_id: &str, binding_value: &str) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if material_id.is_empty() {
        result = result.add_error("material_id cannot be empty");
    }
    if binding_value.is_empty() {
        result = result.add_error("binding value cannot be empty");
    }
    result
}

/// Validate material rung level
pub fn validate_material_rung(material_id: &str, rung_level: u32) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if material_id.is_empty() {
        result = result.add_error("material_id cannot be empty");
    }
    if rung_level > 10 {
        result = result.add_error("rung_level cannot exceed 10");
    }
    result
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("MyProject").is_valid());
        assert!(!validate_project_name("").is_valid());
        assert!(!validate_project_name(&"a".repeat(65)).is_valid());
    }

    #[test]
    fn test_validate_terrain_sculpt() {
        assert!(validate_terrain_sculpt(&[0.0, 0.0], 1.0, 0.5).is_valid());
        assert!(!validate_terrain_sculpt(&[0.0, 0.0], -1.0, 0.5).is_valid());
        assert!(!validate_terrain_sculpt(&[0.0, 0.0], 1.0, 1.5).is_valid());
    }

    #[test]
    fn test_validate_time_of_day() {
        assert!(validate_time_of_day(12.0).is_valid());
        assert!(!validate_time_of_day(-1.0).is_valid());
        assert!(!validate_time_of_day(25.0).is_valid());
    }
}
