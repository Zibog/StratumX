// Physics integration

use super::projectile::BallisticSimulator;
use crate::ProjectileState;
use engine_core::EngineCoreResult;

const GRAVITY_M_S2: f32 = 9.81;
const AIR_DENSITY_KG_M3: f32 = 1.225;

impl BallisticSimulator {
    pub fn integrate_step(&self, state: &mut ProjectileState, dt_s: f32) -> EngineCoreResult<()> {
        let profile = self.projectile_profile(state.profile_id).ok_or(
            engine_core::EngineCoreError::InvalidDescriptor("projectile profile not found"),
        )?;

        let v_mag = (state.velocity[0] * state.velocity[0]
            + state.velocity[1] * state.velocity[1]
            + state.velocity[2] * state.velocity[2])
            .sqrt();

        let area_m2 = std::f32::consts::PI * (profile.diameter_mm / 2000.0).powi(2);
        let drag_force =
            0.5 * AIR_DENSITY_KG_M3 * v_mag * v_mag * profile.drag_coefficient * area_m2;
        let drag_accel = drag_force / profile.mass_kg;

        let drag_dir = if v_mag > 0.001 {
            [
                -state.velocity[0] / v_mag,
                -state.velocity[1] / v_mag,
                -state.velocity[2] / v_mag,
            ]
        } else {
            [0.0, 0.0, 0.0]
        };

        state.velocity[0] += (drag_dir[0] * drag_accel) * dt_s;
        state.velocity[1] += (drag_dir[1] * drag_accel - GRAVITY_M_S2) * dt_s;
        state.velocity[2] += (drag_dir[2] * drag_accel) * dt_s;

        state.position[0] += state.velocity[0] * dt_s;
        state.position[1] += state.velocity[1] * dt_s;
        state.position[2] += state.velocity[2] * dt_s;

        state.time_alive_s += dt_s;

        Ok(())
    }
}
