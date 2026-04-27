use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WetnessState {
    pub wetness_percent: f32,
    pub last_wet_time: f32,
}

impl WetnessState {
    pub fn new() -> Self {
        Self {
            wetness_percent: 0.0,
            last_wet_time: 0.0,
        }
    }
    pub fn add_water(&mut self, amount: f32, current_time: f32) {
        self.wetness_percent = (self.wetness_percent + amount).min(100.0);
        self.last_wet_time = current_time;
    }
    pub fn dry(&mut self, rate: f32, delta_time: f32) {
        self.wetness_percent = (self.wetness_percent - rate * delta_time).max(0.0);
    }
    pub fn is_wet(&self) -> bool {
        self.wetness_percent > 10.0
    }
}

impl Default for WetnessState {
    fn default() -> Self {
        Self::new()
    }
}
