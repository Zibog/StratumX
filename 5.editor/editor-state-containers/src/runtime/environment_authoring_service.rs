//! Environment authoring service

use crate::*;

pub struct EnvironmentAuthoringService {
    world_state: std::sync::Arc<std::sync::Mutex<WorldState>>,
    event_bus: std::sync::Arc<dyn EventBus>,
    sky_profile_binding: Option<uuid::Uuid>,
}

impl EnvironmentAuthoringService {
    pub fn new(
        world_state: std::sync::Arc<std::sync::Mutex<WorldState>>,
        event_bus: std::sync::Arc<dyn EventBus>,
    ) -> Self {
        Self {
            world_state,
            event_bus,
            sky_profile_binding: None,
        }
    }

    pub fn configure_sky(&mut self, _sky_config: &[f32; 3]) -> Result<(), String> {
        self.event_bus.publish("environment.sky_configured");
        Ok(())
    }

    pub fn bind_sky_profile(&mut self, profile_id: uuid::Uuid) -> Result<(), String> {
        self.sky_profile_binding = Some(profile_id);
        self.event_bus.publish("environment.sky_profile_bound");
        Ok(())
    }

    pub fn get_sky_profile_binding(&self) -> Option<uuid::Uuid> {
        self.sky_profile_binding
    }

    pub fn set_weather(&mut self, weather: WeatherCondition) -> Result<(), String> {
        let mut world = self
            .world_state
            .lock()
            .map_err(|_| "World state lock poisoned".to_string())?;
        let environment = world.environment_state.get_or_insert_with(EnvironmentState::new);
        environment.set_weather(weather);
        self.event_bus.publish("environment.weather_set");
        Ok(())
    }

    pub fn update_lighting(&mut self, _ambient: f32, _intensity: i32) -> Result<(), String> {
        self.event_bus.publish("environment.lighting_updated");
        Ok(())
    }

    pub fn bind_weather_regime(&mut self, _regime_id: uuid::Uuid) -> Result<(), String> {
        self.event_bus.publish("environment.weather_regime_bound");
        Ok(())
    }

    pub fn bind_cloud_profile(&mut self, _profile_id: uuid::Uuid) -> Result<(), String> {
        self.event_bus.publish("environment.cloud_profile_bound");
        Ok(())
    }
}
