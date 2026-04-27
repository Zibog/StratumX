use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebrisParticle {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub mass_kg: f32,
    pub size_m: f32,
}

pub fn generate_debris(
    position: [f32; 3],
    energy_j: f32,
    terrain_density: f32,
    debris_count: u32,
    crater_radius: f32,
    ejecta_radius: f32,
    ejecta_volume: f32,
) -> Vec<DebrisParticle> {
    let mut debris = Vec::new();
    let count = debris_count.min(100);
    for i in 0..count {
        let angle = (i as f32 / count as f32) * 2.0 * std::f32::consts::PI;
        let distance = crater_radius + (i as f32 / count as f32) * ejecta_radius;
        let dx = angle.cos() * distance;
        let dz = angle.sin() * distance;
        let dy = (i as f32 / count as f32) * 2.0;
        let speed = (2.0 * energy_j / terrain_density).sqrt() * 0.1;
        let mass_kg = ejecta_volume * terrain_density / count as f32;
        let size_m = (mass_kg / terrain_density).powf(1.0 / 3.0);
        debris.push(DebrisParticle {
            position: [position[0] + dx, position[1] + dy, position[2] + dz],
            velocity: [angle.cos() * speed, speed * 0.5, angle.sin() * speed],
            mass_kg,
            size_m,
        });
    }
    debris
}
