// Impact detection

use super::projectile::BallisticSimulator;
use crate::ProjectileState;

impl BallisticSimulator {
    pub fn check_wall_hit(
        &self,
        state: &ProjectileState,
        wall_position: [f32; 3],
        wall_dimensions: [f32; 3],
    ) -> Option<([f32; 3], f32)> {
        let half_w = wall_dimensions[0] / 2.0;
        let half_h = wall_dimensions[1] / 2.0;
        let half_d = wall_dimensions[2] / 2.0;

        let rel_x = state.position[0] - wall_position[0];
        let rel_y = state.position[1] - wall_position[1];
        let rel_z = state.position[2] - wall_position[2];

        if rel_x.abs() <= half_w && rel_y.abs() <= half_h && rel_z >= -half_d && rel_z <= half_d {
            let v_mag = (state.velocity[0] * state.velocity[0]
                + state.velocity[1] * state.velocity[1]
                + state.velocity[2] * state.velocity[2])
                .sqrt();

            if v_mag < 0.001 {
                return None;
            }

            let normal = [0.0, 0.0, -1.0];
            let dot = state.velocity[0] * normal[0]
                + state.velocity[1] * normal[1]
                + state.velocity[2] * normal[2];
            let angle_rad = (dot / v_mag).abs().acos();
            let angle_deg = angle_rad.to_degrees();

            Some((state.position, angle_deg))
        } else {
            None
        }
    }
}
