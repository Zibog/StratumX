/// Validation for world property substrate operations
use crate::types::*;
use thiserror::Error;

/// Validation errors for world property substrate
#[derive(Debug, Error)]
pub enum SubstrateValidationError {
    #[error("Invalid field dimensions: {0}")]
    InvalidDimensions(String),

    #[error("Property not found: {0:?}")]
    PropertyNotFound(PropertyType),

    #[error("Coordinates out of bounds: ({0}, {1}, {2})")]
    OutOfBounds(usize, usize, usize),

    #[error("Cycle detected in update order graph")]
    CycleDetected,

    #[error("Invalid field value: {0}")]
    InvalidValue(String),

    #[error("Conflict resolution failed: {0}")]
    ConflictResolutionFailed(String),

    #[error("Persistence error: {0}")]
    PersistenceError(String),
}

/// Validate cell field dimensions
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

    // Check for reasonable maximum size (prevent memory exhaustion)
    const MAX_DIMENSION: usize = 1024;
    if dx > MAX_DIMENSION || dy > MAX_DIMENSION || dz > MAX_DIMENSION {
        return Err(SubstrateValidationError::InvalidDimensions(format!(
            "Dimensions exceed maximum {}: ({}, {}, {})",
            MAX_DIMENSION, dx, dy, dz
        )));
    }

    Ok(())
}

/// Validate coordinates against field dimensions
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

/// Validate field value is within reasonable range
pub fn validate_field_value(
    property: PropertyType,
    value: f32,
) -> Result<(), SubstrateValidationError> {
    // Check for NaN or infinity
    if !value.is_finite() {
        return Err(SubstrateValidationError::InvalidValue(format!(
            "Value must be finite for {:?}: {}",
            property, value
        )));
    }

    // Property-specific range validation
    match property {
        PropertyType::Wetness => {
            if !(0.0..=1.0).contains(&value) {
                return Err(SubstrateValidationError::InvalidValue(format!(
                    "Wetness must be in range [0.0, 1.0]: {}",
                    value
                )));
            }
        }
        PropertyType::Heat => {
            if value < 0.0 {
                return Err(SubstrateValidationError::InvalidValue(format!(
                    "Heat (Kelvin) must be non-negative: {}",
                    value
                )));
            }
        }
        PropertyType::SmokeDensity => {
            if !(0.0..=1.0).contains(&value) {
                return Err(SubstrateValidationError::InvalidValue(format!(
                    "Smoke density must be in range [0.0, 1.0]: {}",
                    value
                )));
            }
        }
        PropertyType::ToxicContamination => {
            if !(0.0..=1.0).contains(&value) {
                return Err(SubstrateValidationError::InvalidValue(format!(
                    "Toxic contamination must be in range [0.0, 1.0]: {}",
                    value
                )));
            }
        }
        PropertyType::VisibilityObscuration => {
            if !(0.0..=1.0).contains(&value) {
                return Err(SubstrateValidationError::InvalidValue(format!(
                    "Visibility obscuration must be in range [0.0, 1.0]: {}",
                    value
                )));
            }
        }
        PropertyType::AnomalyIntensity => {
            if !(0.0..=1.0).contains(&value) {
                return Err(SubstrateValidationError::InvalidValue(format!(
                    "Anomaly intensity must be in range [0.0, 1.0]: {}",
                    value
                )));
            }
        }
        // WindHint and SoundPressureHint can have arbitrary values
        PropertyType::WindHint | PropertyType::SoundPressureHint => {}
    }

    Ok(())
}

/// Validate update order graph has no cycles
pub fn validate_update_order(graph: &UpdateOrderGraph) -> Result<(), SubstrateValidationError> {
    graph
        .topological_sort()
        .map(|_| ())
        .map_err(|_| SubstrateValidationError::CycleDetected)
}

/// Validate conflict rule
pub fn validate_conflict_rule(rule: &ConflictRule) -> Result<(), SubstrateValidationError> {
    if rule.properties.is_empty() {
        return Err(SubstrateValidationError::ConflictResolutionFailed(
            "Conflict rule must have at least one property".to_string(),
        ));
    }

    // If using blend strategy, verify weights are provided
    if rule.resolution == ConflictResolution::Blend {
        for prop in &rule.properties {
            if !rule.weights.contains_key(prop) {
                return Err(SubstrateValidationError::ConflictResolutionFailed(format!(
                    "Missing weight for property {:?} in blend rule",
                    prop
                )));
            }
        }

        // Verify weights sum to approximately 1.0
        let sum: f32 = rule.weights.values().sum();
        if (sum - 1.0).abs() > 0.01 {
            return Err(SubstrateValidationError::ConflictResolutionFailed(format!(
                "Blend weights must sum to 1.0, got {}",
                sum
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dimensions() {
        assert!(validate_dimensions((10, 10, 10)).is_ok());
        assert!(validate_dimensions((0, 10, 10)).is_err());
        assert!(validate_dimensions((10, 0, 10)).is_err());
        assert!(validate_dimensions((10, 10, 0)).is_err());
        assert!(validate_dimensions((2000, 10, 10)).is_err()); // Too large
    }

    #[test]
    fn test_validate_coordinates() {
        let dims = (10, 10, 10);
        assert!(validate_coordinates(5, 5, 5, dims).is_ok());
        assert!(validate_coordinates(0, 0, 0, dims).is_ok());
        assert!(validate_coordinates(9, 9, 9, dims).is_ok());
        assert!(validate_coordinates(10, 5, 5, dims).is_err());
        assert!(validate_coordinates(5, 10, 5, dims).is_err());
        assert!(validate_coordinates(5, 5, 10, dims).is_err());
    }

    #[test]
    fn test_validate_field_value() {
        // Wetness
        assert!(validate_field_value(PropertyType::Wetness, 0.5).is_ok());
        assert!(validate_field_value(PropertyType::Wetness, 0.0).is_ok());
        assert!(validate_field_value(PropertyType::Wetness, 1.0).is_ok());
        assert!(validate_field_value(PropertyType::Wetness, -0.1).is_err());
        assert!(validate_field_value(PropertyType::Wetness, 1.1).is_err());

        // Heat
        assert!(validate_field_value(PropertyType::Heat, 300.0).is_ok());
        assert!(validate_field_value(PropertyType::Heat, 0.0).is_ok());
        assert!(validate_field_value(PropertyType::Heat, -1.0).is_err());

        // NaN/Infinity
        assert!(validate_field_value(PropertyType::Wetness, f32::NAN).is_err());
        assert!(validate_field_value(PropertyType::Wetness, f32::INFINITY).is_err());
    }

    #[test]
    fn test_validate_update_order() {
        let mut graph = UpdateOrderGraph::new();
        graph.add_edge(PropertyType::Heat, PropertyType::Wetness);
        assert!(validate_update_order(&graph).is_ok());

        // Add cycle
        graph.add_edge(PropertyType::Wetness, PropertyType::Heat);
        assert!(validate_update_order(&graph).is_err());
    }

    #[test]
    fn test_validate_conflict_rule() {
        use std::collections::HashMap;

        // Valid priority rule
        let rule = ConflictRule {
            properties: vec![PropertyType::Heat, PropertyType::Wetness],
            resolution: ConflictResolution::Priority,
            weights: HashMap::new(),
        };
        assert!(validate_conflict_rule(&rule).is_ok());

        // Invalid blend rule (missing weights)
        let rule = ConflictRule {
            properties: vec![PropertyType::Heat, PropertyType::Wetness],
            resolution: ConflictResolution::Blend,
            weights: HashMap::new(),
        };
        assert!(validate_conflict_rule(&rule).is_err());

        // Valid blend rule
        let mut weights = HashMap::new();
        weights.insert(PropertyType::Heat, 0.6);
        weights.insert(PropertyType::Wetness, 0.4);
        let rule = ConflictRule {
            properties: vec![PropertyType::Heat, PropertyType::Wetness],
            resolution: ConflictResolution::Blend,
            weights,
        };
        assert!(validate_conflict_rule(&rule).is_ok());
    }
}
