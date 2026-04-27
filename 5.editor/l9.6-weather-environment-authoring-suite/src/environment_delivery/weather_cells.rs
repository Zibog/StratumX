// Weather Cell Operations

use super::state::EnvironmentAuthoringState;

impl EnvironmentAuthoringState {
    pub fn create_weather_cell(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        density: f32,
    ) -> Result<u32, String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge
            .create_weather_cell(position, velocity, radius_km, density)
    }

    pub fn update_weather_cell(
        &mut self,
        cell_id: u32,
        precipitation_rate: Option<f32>,
        lightning_probability: Option<f32>,
    ) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge
            .update_weather_cell(cell_id, precipitation_rate, lightning_probability)
    }

    pub fn create_storm_cell(&mut self, position: [f32; 3], intensity: f32) -> Result<u32, String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        let velocity = [0.0, 0.0, 0.0];
        let radius_km = 5.0 * intensity;
        let density = intensity;
        self.engine_bridge
            .create_weather_cell(position, velocity, radius_km, density)
    }
}
