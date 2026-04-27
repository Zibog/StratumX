use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub mass_kg: f32,
    pub size: [f32; 3],
    pub can_burn: bool,
}

pub fn generate_fragments(
    impact_position: [f32; 3],
    impact_direction: [f32; 3],
    impact_energy_j: f32,
    fragment_count: u32,
    failure_mode: super::support_failure::FailureMode,
    can_burn: bool,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let total_mass = 100.0;
    let fragment_mass = total_mass / fragment_count as f32;
    for i in 0..fragment_count {
        let angle = (i as f32 / fragment_count as f32) * 2.0 * std::f32::consts::PI;
        let spread = 0.5;
        let dx = angle.cos() * spread;
        let dy = (i as f32 / fragment_count as f32) * spread;
        let dz = angle.sin() * spread;
        let speed = (impact_energy_j / total_mass).sqrt() * 0.3;
        let size = match failure_mode {
            super::support_failure::FailureMode::Shatter => [0.05, 0.05, 0.05],
            super::support_failure::FailureMode::Fracture => [0.3, 0.3, 0.3],
            super::support_failure::FailureMode::Collapse => [0.5, 0.5, 0.5],
            super::support_failure::FailureMode::Topple => [1.0, 1.0, 1.0],
            _ => [0.2, 0.2, 0.2],
        };
        fragments.push(Fragment {
            position: [
                impact_position[0] + dx,
                impact_position[1] + dy,
                impact_position[2] + dz,
            ],
            velocity: [
                impact_direction[0] * speed + dx,
                impact_direction[1] * speed + dy + 1.0,
                impact_direction[2] * speed + dz,
            ],
            angular_velocity: [angle.sin() * 2.0, angle.cos() * 2.0, i as f32 * 0.5],
            mass_kg: fragment_mass,
            size,
            can_burn,
        });
    }
    fragments
}
