//! Environment Authoring Service
//!
//! Domain service for environment configuration including sky, weather, and lighting.
//! Owns environment truth: sky profile binding, weather regime binding, cloud profile binding.
//! UI panels read from this service, not own authoritative state.
//!
//! **Requirements: 6.2, 6.5**
//! Abstraction Level: L4 (Authoring Tools)

mod binding_truth;
mod environment_mutations;
mod publication_seam;

use crate::{EditorEvent, EnvironmentState, EventBus, WeatherCondition, WorldState};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub use binding_truth::EnvironmentBindings;

/// Environment authoring service for environment management operations
///
/// **Requirements: 6.2, 6.5**
/// - Owns sky profile binding (not UI)
/// - Owns weather regime binding (not UI)
/// - Owns cloud profile binding (not UI)
pub struct EnvironmentAuthoringService {
    world_state: Arc<Mutex<WorldState>>,
    event_bus: Arc<dyn EventBus>,
    bindings: EnvironmentBindings,
}

impl EnvironmentAuthoringService {
    /// Create a new environment authoring service
    pub fn new(world_state: Arc<Mutex<WorldState>>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            world_state,
            event_bus,
            bindings: EnvironmentBindings::default(),
        }
    }

    /// Bind a sky profile
    ///
    /// **Requirement 6.5**: Service owns sky profile binding, not UI
    pub fn bind_sky_profile(&mut self, profile_id: Uuid) -> Result<(), String> {
        self.bindings.bind_sky_profile(profile_id);
        self.emit(EditorEvent::SkyConfigured);
        Ok(())
    }

    /// Bind a weather regime
    ///
    /// **Requirement 6.5**: Service owns weather regime binding, not UI
    pub fn bind_weather_regime(&mut self, regime_id: Uuid) -> Result<(), String> {
        self.bindings.bind_weather_regime(regime_id);
        self.emit(EditorEvent::WeatherChanged);
        Ok(())
    }

    /// Bind a cloud profile
    ///
    /// **Requirement 6.5**: Service owns cloud profile binding, not UI
    pub fn bind_cloud_profile(&mut self, profile_id: Uuid) -> Result<(), String> {
        self.bindings.bind_cloud_profile(profile_id);
        self.emit(EditorEvent::SkyConfigured);
        Ok(())
    }

    /// Get the current sky profile binding
    ///
    /// **Requirement 6.6**: UI reads from service
    pub fn get_sky_profile_binding(&self) -> Option<Uuid> {
        self.bindings.sky_profile_binding()
    }

    /// Get the current weather regime binding
    ///
    /// **Requirement 6.6**: UI reads from service
    pub fn get_weather_regime_binding(&self) -> Option<Uuid> {
        self.bindings.weather_regime_binding()
    }

    /// Get the current cloud profile binding
    ///
    /// **Requirement 6.6**: UI reads from service
    pub fn get_cloud_profile_binding(&self) -> Option<Uuid> {
        self.bindings.cloud_profile_binding()
    }

    /// Clear all bindings
    pub fn clear_bindings(&mut self) {
        self.bindings.clear();
    }

    /// Configure sky settings
    pub fn configure_sky(&mut self, sky_color: [f32; 3], sun_intensity: f32) -> Result<(), String> {
        self.update_environment_state(|env| {
            environment_mutations::configure_sky(env, sky_color, sun_intensity)
        });
        self.emit(EditorEvent::SkyConfigured);
        Ok(())
    }

    /// Set weather conditions
    pub fn set_weather(&mut self, weather: WeatherCondition) -> Result<(), String> {
        self.update_environment_state(|env| environment_mutations::set_weather(env, weather));
        self.emit(EditorEvent::WeatherChanged);
        Ok(())
    }

    /// Update lighting settings
    pub fn update_lighting(
        &mut self,
        ambient_intensity: f32,
        shadow_quality: u32,
    ) -> Result<(), String> {
        self.update_environment_state(|env| {
            environment_mutations::update_lighting(env, ambient_intensity, shadow_quality)
        });
        self.emit(EditorEvent::LightingUpdated);
        Ok(())
    }

    fn update_environment_state<F>(&self, mutate: F)
    where
        F: FnOnce(&mut EnvironmentState),
    {
        let mut world = self.world_state.lock().unwrap();
        let environment = world
            .environment_state
            .get_or_insert_with(EnvironmentState::new);
        mutate(environment);
    }

    fn emit(&self, event: EditorEvent) {
        publication_seam::emit(&*self.event_bus, event);
    }
}
