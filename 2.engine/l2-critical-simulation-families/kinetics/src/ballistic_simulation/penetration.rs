// Penetration mechanics

use super::projectile::BallisticSimulator;
use crate::{ImpactVerdict, LayerImpactEvent};
use engine_core::EngineCoreResult;
use engine_material::{MaterialRegistry, MaterialStack};

impl BallisticSimulator {
    pub(crate) fn resolve_layer_traversal(
        &self,
        entry_energy_j: f32,
        incidence_angle_deg: f32,
        projectile_diameter_mm: f32,
        stack: &MaterialStack,
        materials: &MaterialRegistry,
    ) -> EngineCoreResult<Vec<LayerImpactEvent>> {
        let mut events = Vec::new();
        let mut remaining_energy = entry_energy_j;

        for (idx, layer) in stack.layers.iter().enumerate() {
            if remaining_energy <= 0.0 {
                break;
            }

            let archetype = materials.archetype(layer.archetype_id).ok_or(
                engine_core::EngineCoreError::InvalidDescriptor("material archetype not found"),
            )?;

            let absorption_factor = self.compute_absorption_factor(
                archetype.fracture_energy_j_m2,
                archetype.thickness_mm,
                layer.thickness_mm,
                projectile_diameter_mm,
                incidence_angle_deg,
            );

            let absorbed = remaining_energy * absorption_factor;
            let exit_energy = remaining_energy - absorbed;

            let verdict = if exit_energy > 0.1 {
                ImpactVerdict::Penetrated
            } else if absorbed > archetype.fracture_energy_j_m2 * 0.5 {
                ImpactVerdict::Embedded
            } else {
                ImpactVerdict::Stopped
            };

            events.push(LayerImpactEvent {
                layer_index: idx as u8,
                entry_energy_j: remaining_energy,
                exit_energy_j: exit_energy,
                energy_absorbed_j: absorbed,
                verdict,
            });

            remaining_energy = exit_energy;

            if verdict != ImpactVerdict::Penetrated {
                break;
            }
        }

        Ok(events)
    }

    pub(crate) fn compute_absorption_factor(
        &self,
        fracture_energy_j_m2: f32,
        archetype_thickness_mm: f32,
        layer_thickness_mm: f32,
        _projectile_diameter_mm: f32,
        incidence_angle_deg: f32,
    ) -> f32 {
        let base_absorption = (fracture_energy_j_m2 / 100.0).min(0.9);
        let thickness_factor = (layer_thickness_mm / archetype_thickness_mm).min(1.0);
        let angle_factor = (incidence_angle_deg / 90.0).clamp(0.5, 1.0);
        (base_absorption * thickness_factor * angle_factor).clamp(0.1, 0.95)
    }
}
