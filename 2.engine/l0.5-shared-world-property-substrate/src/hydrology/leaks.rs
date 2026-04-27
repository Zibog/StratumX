use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leak {
    pub position: [f32; 3],
    pub hole_height_m: f32,
    pub hole_diameter_mm: f32,
    pub active: bool,
}

impl Leak {
    pub fn new(position: [f32; 3], hole_diameter_mm: f32) -> Self {
        Self {
            position,
            hole_height_m: position[1],
            hole_diameter_mm,
            active: true,
        }
    }
    pub fn flow_rate_liters_per_sec(&self, water_level_m: f32) -> f32 {
        if !self.active || water_level_m <= self.hole_height_m {
            return 0.0;
        }
        let g = 9.81;
        let height_diff = water_level_m - self.hole_height_m;
        let velocity = (2.0 * g * height_diff).sqrt();
        let hole_radius_m = (self.hole_diameter_mm / 1000.0) / 2.0;
        let area_m2 = std::f32::consts::PI * hole_radius_m * hole_radius_m;
        area_m2 * velocity * 1000.0
    }
}
