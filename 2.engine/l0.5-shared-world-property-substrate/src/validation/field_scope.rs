use crate::types::PropertyType;

use super::SubstrateValidationError;

/// Validate cell field dimensions.
pub fn validate_dimensions(
    dimensions: (usize, usize, usize),
) -> Result<(), SubstrateValidationError> {
    let (dx, dy, dz) = dimensions;

    if dx == 0 || dy == 0 || dz == 0 {
        return Err(SubstrateValidationError::InvalidDimensions(format!(
            "Dimensions must be non-zero: ({}, {}, {})",
            dx, dy, dz
        )));
    }

    const MAX_DIMENSION: usize = 1024;
    if dx > MAX_DIMENSION || dy > MAX_DIMENSION || dz > MAX_DIMENSION {
        return Err(SubstrateValidationError::InvalidDimensions(format!(
            "Dimensions exceed maximum {}: ({}, {}, {})",
            MAX_DIMENSION, dx, dy, dz
        )));
    }

    Ok(())
}

/// Validate coordinates against field dimensions.
pub fn validate_coordinates(
    x: usize,
    y: usize,
    z: usize,
    dimensions: (usize, usize, usize),
) -> Result<(), SubstrateValidationError> {
    let (dx, dy, dz) = dimensions;

    if x >= dx || y >= dy || z >= dz {
        return Err(SubstrateValidationError::OutOfBounds(x, y, z));
    }

    Ok(())
}

/// Validate field value is within reasonable range.
pub fn validate_field_value(
    property: PropertyType,
    value: f32,
) -> Result<(), SubstrateValidationError> {
    if !value.is_finite() {
        return Err(SubstrateValidationError::InvalidValue(format!(
            "Value must be finite for {:?}: {}",
            property, value
        )));
    }

    match property {
        PropertyType::Wetness => validate_unit_interval("Wetness", value),
        PropertyType::Heat => {
            if value < 0.0 {
                Err(SubstrateValidationError::InvalidValue(format!(
                    "Heat (Kelvin) must be non-negative: {}",
                    value
                )))
            } else {
                Ok(())
            }
        }
        PropertyType::SmokeDensity => validate_unit_interval("Smoke density", value),
        PropertyType::ToxicContamination => validate_unit_interval("Toxic contamination", value),
        PropertyType::VisibilityObscuration => {
            validate_unit_interval("Visibility obscuration", value)
        }
        PropertyType::AnomalyIntensity => validate_unit_interval("Anomaly intensity", value),
        PropertyType::WindHint | PropertyType::SoundPressureHint => Ok(()),
    }
}

fn validate_unit_interval(label: &str, value: f32) -> Result<(), SubstrateValidationError> {
    if !(0.0..=1.0).contains(&value) {
        return Err(SubstrateValidationError::InvalidValue(format!(
            "{} must be in range [0.0, 1.0]: {}",
            label, value
        )));
    }

    Ok(())
}
