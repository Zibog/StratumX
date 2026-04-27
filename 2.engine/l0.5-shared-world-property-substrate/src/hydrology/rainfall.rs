use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rainfall {
    pub intensity_mm_per_hour: f32,
    pub area_m2: f32,
    pub active: bool,
}

impl Rainfall {
    pub fn new(intensity_mm_per_hour: f32, area_m2: f32) -> Self {
        Self {
            intensity_mm_per_hour,
            area_m2,
            active: true,
        }
    }
    pub fn water_added_per_sec(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        let rate_m_per_sec = (self.intensity_mm_per_hour / 1000.0) / 3600.0;
        self.area_m2 * rate_m_per_sec * 1000.0
    }
}
