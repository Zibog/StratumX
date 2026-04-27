// Weather scenario validation

use super::presets::WeatherScenario;

impl WeatherScenario {
    pub fn validate(&self) -> Result<(), String> {
        if self.storm_bias < 0.0 || self.storm_bias > 1.0 {
            return Err("storm_bias must be in range [0.0, 1.0]".to_string());
        }
        if self.fog_bias < 0.0 || self.fog_bias > 1.0 {
            return Err("fog_bias must be in range [0.0, 1.0]".to_string());
        }
        if self.rain_bias < 0.0 || self.rain_bias > 1.0 {
            return Err("rain_bias must be in range [0.0, 1.0]".to_string());
        }
        if self.cell_intensity < 0.0 || self.cell_intensity > 1.0 {
            return Err("cell_intensity must be in range [0.0, 1.0]".to_string());
        }
        Ok(())
    }
}
