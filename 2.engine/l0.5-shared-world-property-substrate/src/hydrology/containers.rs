use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluidContainer {
    pub position: [f32; 3],
    pub capacity_liters: f32,
    pub current_volume_liters: f32,
    pub base_height_m: f32,
    pub top_height_m: f32,
    pub cross_section_m2: f32,
}

impl FluidContainer {
    pub fn new(
        position: [f32; 3],
        capacity_liters: f32,
        height_m: f32,
        cross_section_m2: f32,
    ) -> Self {
        Self {
            position,
            capacity_liters,
            current_volume_liters: 0.0,
            base_height_m: position[1],
            top_height_m: position[1] + height_m,
            cross_section_m2,
        }
    }
    pub fn current_water_level_m(&self) -> f32 {
        if self.current_volume_liters <= 0.0 {
            return self.base_height_m;
        }
        let volume_m3 = self.current_volume_liters / 1000.0;
        self.base_height_m + volume_m3 / self.cross_section_m2
    }
    pub fn add_water(&mut self, liters: f32) -> f32 {
        let capacity_m3 = self.capacity_liters / 1000.0;
        let current_m3 = self.current_volume_liters / 1000.0;
        let added_m3 = liters / 1000.0;
        let new_volume_m3 = (current_m3 + added_m3).min(capacity_m3);
        let overflow_m3 = (current_m3 + added_m3 - capacity_m3).max(0.0);
        self.current_volume_liters = new_volume_m3 * 1000.0;
        overflow_m3 * 1000.0
    }
    pub fn remove_water(&mut self, liters: f32) -> f32 {
        let removed = liters.min(self.current_volume_liters);
        self.current_volume_liters -= removed;
        removed
    }
}
