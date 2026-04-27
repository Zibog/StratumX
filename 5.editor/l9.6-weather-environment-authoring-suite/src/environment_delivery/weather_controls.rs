// Weather Control Operations

use super::state::EnvironmentAuthoringState;
use editor_dto_law::{CloudShadowPosture, WeatherRegime};

impl EnvironmentAuthoringState {
    pub fn set_weather_regime(&mut self, regime: WeatherRegime) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_weather_regime(regime)
    }

    pub fn lock_weather_authoring(&mut self, locked: bool) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.lock_weather_authoring(locked)
    }

    pub fn set_weather_bias(&mut self, storm: f32, fog: f32, rain: f32) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_weather_bias(storm, fog, rain)
    }

    pub fn set_cloud_shadow_active(&mut self, active: bool) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_cloud_shadow_active(active)
    }

    pub fn set_cloud_shadow_posture(&mut self, posture: CloudShadowPosture) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_cloud_shadow_posture(posture)
    }

    pub fn apply_weather_scenario(
        &mut self,
        scenario: &crate::WeatherScenario,
    ) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_weather_regime(scenario.regime)
    }
}
