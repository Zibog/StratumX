use std::collections::HashMap;

use crate::types::{ConflictResolution, ConflictRule, PropertyType};

use super::SubstrateValidationError;

/// Validate conflict rule.
pub fn validate_conflict_rule(rule: &ConflictRule) -> Result<(), SubstrateValidationError> {
    if rule.properties.is_empty() {
        return Err(SubstrateValidationError::ConflictResolutionFailed(
            "Conflict rule must have at least one property".to_string(),
        ));
    }

    if rule.resolution == ConflictResolution::Blend {
        if let Some(property) = missing_blend_weight(&rule.properties, &rule.weights) {
            return Err(SubstrateValidationError::ConflictResolutionFailed(format!(
                "Missing weight for property {:?} in blend rule",
                property
            )));
        }

        let sum = blend_weight_sum(&rule.weights);
        if (sum - 1.0).abs() > 0.01 {
            return Err(SubstrateValidationError::ConflictResolutionFailed(format!(
                "Blend weights must sum to 1.0, got {}",
                sum
            )));
        }
    }

    Ok(())
}

pub(crate) fn missing_blend_weight(
    properties: &[PropertyType],
    weights: &HashMap<PropertyType, f32>,
) -> Option<PropertyType> {
    properties
        .iter()
        .copied()
        .find(|property| !weights.contains_key(property))
}

pub(crate) fn blend_weight_sum(weights: &HashMap<PropertyType, f32>) -> f32 {
    weights.values().sum()
}
