use crate::types::PropertyType;
use crate::validation::validate_field_value;

use super::super::SubstrateRuntime;

impl SubstrateRuntime {
    pub(super) fn apply_property_update(&mut self, property: PropertyType) -> Result<(), String> {
        match property {
            PropertyType::Wetness => self.apply_dependency(
                PropertyType::Heat,
                PropertyType::Wetness,
                |heat, wetness| {
                    let evaporation = ((heat - 273.15) / 500.0).clamp(0.0, 1.0) * 0.25;
                    (wetness - evaporation).clamp(0.0, 1.0)
                },
            ),
            PropertyType::SmokeDensity => self.apply_dependency(
                PropertyType::WindHint,
                PropertyType::SmokeDensity,
                |wind, smoke| (smoke * (1.0 - wind.clamp(0.0, 1.0) * 0.2)).clamp(0.0, 1.0),
            ),
            PropertyType::VisibilityObscuration => self.apply_dependency(
                PropertyType::SmokeDensity,
                PropertyType::VisibilityObscuration,
                |smoke, visibility| visibility.max(smoke).clamp(0.0, 1.0),
            ),
            PropertyType::AnomalyIntensity => self.apply_dependency(
                PropertyType::ToxicContamination,
                PropertyType::AnomalyIntensity,
                |toxicity, anomaly| anomaly.max(toxicity).clamp(0.0, 1.0),
            ),
            _ => Ok(()),
        }
    }

    fn apply_dependency<F>(
        &mut self,
        source_property: PropertyType,
        target_property: PropertyType,
        mut transform: F,
    ) -> Result<(), String>
    where
        F: FnMut(f32, f32) -> f32,
    {
        let Some(source) = self.substrate.cell_fields.get(&source_property).cloned() else {
            return Ok(());
        };
        let Some(target) = self.substrate.cell_fields.get_mut(&target_property) else {
            return Ok(());
        };

        if source.dimensions != target.dimensions {
            return Err(format!(
                "property dependency dimensions mismatch: {:?} -> {:?} expected {:?}, got {:?}",
                source_property, target_property, source.dimensions, target.dimensions
            ));
        }

        let mut changed = false;
        for (source_value, target_value) in source.data.iter().copied().zip(target.data.iter_mut())
        {
            let updated = transform(source_value, *target_value);
            validate_field_value(target_property, updated).map_err(|err| err.to_string())?;
            if (*target_value - updated).abs() > f32::EPSILON {
                *target_value = updated;
                changed = true;
            }
        }

        if changed {
            self.mark_dirty(target_property);
        }
        Ok(())
    }
}
