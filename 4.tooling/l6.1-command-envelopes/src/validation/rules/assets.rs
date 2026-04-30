// Asset validation rules

use crate::validation::errors::ValidationResult;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_terrain_sculpt() {
        assert!(validate_terrain_sculpt(&[0.0, 0.0], 1.0, 0.5).is_valid());
        assert!(!validate_terrain_sculpt(&[0.0, 0.0], -1.0, 0.5).is_valid());
        assert!(!validate_terrain_sculpt(&[0.0, 0.0], 1.0, 1.5).is_valid());
    }
}
