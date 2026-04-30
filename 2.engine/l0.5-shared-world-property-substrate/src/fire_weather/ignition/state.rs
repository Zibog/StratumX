use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireState {
    pub burning: bool,
    pub temperature_celsius: f32,
    pub fuel_remaining_percent: f32,
    pub ignition_time: Option<f32>,
}

impl FireState {
    pub fn new() -> Self {
        Self {
            burning: false,
            temperature_celsius: 20.0,
            fuel_remaining_percent: 100.0,
            ignition_time: None,
        }
    }

    pub fn attempt_ignition(
        &mut self,
        heat_source_temp: f32,
        material_ignition_temp: f32,
        current_time: f32,
    ) -> bool {
        if self.burning {
            return false;
        }
        if heat_source_temp >= material_ignition_temp && self.fuel_remaining_percent > 0.0 {
            self.burning = true;
            self.temperature_celsius = material_ignition_temp;
            self.ignition_time = Some(current_time);
            true
        } else {
            false
        }
    }

    pub fn extinguish(&mut self) {
        self.burning = false;
        self.temperature_celsius = 20.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.burning {
            self.fuel_remaining_percent = (self.fuel_remaining_percent - delta_time).max(0.0);
            self.temperature_celsius = 800.0;
            if self.fuel_remaining_percent <= 0.0 {
                self.extinguish();
            }
        } else {
            self.temperature_celsius = (self.temperature_celsius - 50.0 * delta_time).max(20.0);
        }
    }
}

impl Default for FireState {
    fn default() -> Self {
        Self::new()
    }
}
