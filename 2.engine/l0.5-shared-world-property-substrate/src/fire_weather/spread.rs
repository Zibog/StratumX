use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmokeParticle {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub density: f32,
    pub lifetime_sec: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmokeSystem {
    pub particles: Vec<SmokeParticle>,
    pub wind_velocity: [f32; 3],
}

impl SmokeSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            wind_velocity: [0.0, 0.0, 0.0],
        }
    }
    pub fn emit_smoke(&mut self, position: [f32; 3], intensity: f32) {
        let particle_count = (intensity * 5.0) as usize;
        for i in 0..particle_count {
            let angle = (i as f32 / particle_count as f32) * 2.0 * std::f32::consts::PI;
            let spread = 0.2;
            self.particles.push(SmokeParticle {
                position,
                velocity: [
                    angle.cos() * spread,
                    1.0 + (i as f32 * 0.1),
                    angle.sin() * spread,
                ],
                density: 1.0,
                lifetime_sec: 10.0,
            });
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.velocity[0] += self.wind_velocity[0] * delta_time;
            particle.velocity[1] += self.wind_velocity[1] * delta_time;
            particle.velocity[2] += self.wind_velocity[2] * delta_time;
            particle.position[0] += particle.velocity[0] * delta_time;
            particle.position[1] += particle.velocity[1] * delta_time;
            particle.position[2] += particle.velocity[2] * delta_time;
            particle.density *= 0.99;
            particle.lifetime_sec -= delta_time;
        }
        self.particles
            .retain(|p| p.lifetime_sec > 0.0 && p.density > 0.01);
    }
}

impl Default for SmokeSystem {
    fn default() -> Self {
        Self::new()
    }
}
