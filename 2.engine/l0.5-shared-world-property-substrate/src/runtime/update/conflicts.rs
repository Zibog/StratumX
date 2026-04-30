use crate::types::{ConflictResolution, ConflictRule};
use crate::validation::validate_field_value;

use super::super::SubstrateRuntime;

impl SubstrateRuntime {
    pub(super) fn apply_conflict_rule(&mut self, rule: &ConflictRule) -> Result<(), String> {
        let target_property = *rule
            .properties
            .first()
            .ok_or_else(|| "conflict rule requires at least one property".to_string())?;

        let mut contributors = Vec::with_capacity(rule.properties.len());
        for property in &rule.properties {
            let field = self
                .substrate
                .cell_fields
                .get(property)
                .cloned()
                .ok_or_else(|| format!("conflict rule references missing field {:?}", property))?;
            contributors.push((*property, field));
        }

        let dimensions = contributors[0].1.dimensions;
        if let Some((property, field)) = contributors
            .iter()
            .find(|(_, field)| field.dimensions != dimensions)
        {
            return Err(format!(
                "conflict rule dimensions mismatch for {:?}: expected {:?}, got {:?}",
                property, dimensions, field.dimensions
            ));
        }

        let target = self
            .substrate
            .cell_fields
            .get_mut(&target_property)
            .ok_or_else(|| format!("target field {:?} is missing", target_property))?;

        for index in 0..target.data.len() {
            let resolved = resolve_index(rule, &contributors, index);
            validate_field_value(target_property, resolved).map_err(|err| err.to_string())?;
            target.data[index] = resolved;
        }

        self.mark_dirty(target_property);
        Ok(())
    }
}

fn resolve_index(
    rule: &ConflictRule,
    contributors: &[(crate::types::PropertyType, crate::types::CellField)],
    index: usize,
) -> f32 {
    match rule.resolution {
        ConflictResolution::Priority => contributors[0].1.data[index],
        ConflictResolution::Blend => contributors.iter().fold(0.0, |acc, (property, field)| {
            let weight = rule.weights.get(property).copied().unwrap_or(0.0);
            acc + field.data[index] * weight
        }),
        ConflictResolution::Maximum => contributors
            .iter()
            .map(|(_, field)| field.data[index])
            .fold(f32::NEG_INFINITY, f32::max),
        ConflictResolution::Minimum => contributors
            .iter()
            .map(|(_, field)| field.data[index])
            .fold(f32::INFINITY, f32::min),
    }
}
