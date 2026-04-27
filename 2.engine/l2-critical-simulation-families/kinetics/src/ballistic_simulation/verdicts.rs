// Impact resolution and verdicts

use super::projectile::BallisticSimulator;
use crate::{ImpactResult, ImpactVerdict, ProjectileState};
use engine_core::EngineCoreResult;
use engine_material::{MaterialRegistry, MaterialStackId};

impl BallisticSimulator {
    pub fn resolve_impact(
        &self,
        state: &ProjectileState,
        target_entity: u32,
        impact_position: [f32; 3],
        incidence_angle_deg: f32,
        stack_id: MaterialStackId,
        materials: &MaterialRegistry,
    ) -> EngineCoreResult<ImpactResult> {
        let profile = self.projectile_profile(state.profile_id).ok_or(
            engine_core::EngineCoreError::InvalidDescriptor("projectile profile not found"),
        )?;

        let v_mag = (state.velocity[0] * state.velocity[0]
            + state.velocity[1] * state.velocity[1]
            + state.velocity[2] * state.velocity[2])
            .sqrt();
        let entry_energy_j = 0.5 * profile.mass_kg * v_mag * v_mag;

        let stack =
            materials
                .stack(stack_id)
                .ok_or(engine_core::EngineCoreError::InvalidDescriptor(
                    "material stack not found",
                ))?;

        let layer_events = self.resolve_layer_traversal(
            entry_energy_j,
            incidence_angle_deg,
            profile.diameter_mm,
            stack,
            materials,
        )?;

        let final_energy = layer_events.last().map(|e| e.exit_energy_j).unwrap_or(0.0);
        let final_verdict = if final_energy > 0.1 {
            ImpactVerdict::Penetrated
        } else {
            ImpactVerdict::Stopped
        };

        Ok(ImpactResult {
            target_entity,
            impact_position,
            impact_velocity: state.velocity,
            entry_energy_j,
            incidence_angle_deg,
            layer_events,
            final_verdict,
        })
    }
}
