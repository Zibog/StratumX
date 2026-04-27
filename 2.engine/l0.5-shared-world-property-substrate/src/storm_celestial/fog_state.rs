use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FogState {
    pub density: f32,
    pub color: [f32; 3],
    pub height_falloff: f32,
    pub max_visibility_km: f32,
}

impl Default for FogState {
    fn default() -> Self {
        Self {
            density: 0.02,
            color: [0.8, 0.85, 0.9],
            height_falloff: 0.1,
            max_visibility_km: 50.0,
        }
    }
}

impl FogState {
    pub fn new(density: f32) -> Self {
        Self {
            density: density.clamp(0.0, 1.0),
            color: [0.8, 0.85, 0.9],
            height_falloff: 0.1,
            max_visibility_km: (1.0 - density) * 100.0,
        }
    }
    pub fn set_density(&mut self, density: f32) {
        self.density = density.clamp(0.0, 1.0);
        self.max_visibility_km = (1.0 - self.density) * 100.0;
    }
    pub fn visibility_at_height(&self, height_m: f32) -> f32 {
        let height_factor = (-self.height_falloff * height_m).exp();
        self.max_visibility_km * height_factor
    }
}
