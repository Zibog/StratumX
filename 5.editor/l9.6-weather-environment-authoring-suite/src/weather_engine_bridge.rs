// Weather Engine Bridge
// Connects editor environment delivery to engine SkyWeatherState truth

use editor_dto_law::{CloudShadowPosture, WeatherRegime};
use engine_material::SkyWeatherState;
use engine_world::WorldState;

/// Bridge to engine SkyWeatherState
/// Holds local state for editor and syncs to engine world
pub struct WeatherEngineBridge {
    sky_state: SkyWeatherState,
}

impl WeatherEngineBridge {
    pub fn new() -> Self {
        Self {
            sky_state: SkyWeatherState::new_default(),
        }
    }

    /// Get reference to sky state for reading
    pub fn sky_state(&self) -> &SkyWeatherState {
        &self.sky_state
    }

    /// Get mutable reference to sky state for editing
    pub fn sky_state_mut(&mut self) -> &mut SkyWeatherState {
        &mut self.sky_state
    }

    /// Sync local state to engine world
    pub fn sync_to_world(&self, world: &mut WorldState) -> Result<(), String> {
        if let Some(scene) = world.vertical_slice_scene_mut() {
            scene.sky = self.sky_state.clone();
            Ok(())
        } else {
            Err("No vertical slice scene available".to_string())
        }
    }

    /// Sync from engine world to local state
    pub fn sync_from_world(&mut self, world: &WorldState) -> Result<(), String> {
        if let Some(scene) = world.vertical_slice_scene() {
            self.sky_state = scene.sky.clone();
            Ok(())
        } else {
            Err("No vertical slice scene available".to_string())
        }
    }

    /// Set sun temperature in local state
    pub fn set_sun_temperature(&mut self, kelvin: f32) -> Result<(), String> {
        self.sky_state.set_sun_temperature(kelvin);
        Ok(())
    }

    /// Set sun halo in local state
    pub fn set_sun_halo(&mut self, intensity: f32, falloff: f32) -> Result<(), String> {
        self.sky_state.set_sun_halo(intensity, falloff);
        Ok(())
    }

    /// Set sun disk radius in local state
    pub fn set_sun_disk_radius(&mut self, angular_radius_deg: f32) -> Result<(), String> {
        self.sky_state.set_sun_disk_radius(angular_radius_deg);
        Ok(())
    }

    /// Set weather regime in local state
    pub fn set_weather_regime(&mut self, regime: WeatherRegime) -> Result<(), String> {
        let engine_regime = match regime {
            WeatherRegime::Clear => engine_material::WeatherRegime::Clear,
            WeatherRegime::Scattered => engine_material::WeatherRegime::Scattered,
            WeatherRegime::Overcast => engine_material::WeatherRegime::Overcast,
            WeatherRegime::IncomingStorm => engine_material::WeatherRegime::IncomingStorm,
            WeatherRegime::HeavyStorm => engine_material::WeatherRegime::HeavyStorm,
            WeatherRegime::PostStormCalm => engine_material::WeatherRegime::PostStormCalm,
            WeatherRegime::FogMorning => engine_material::WeatherRegime::FogMorning,
            WeatherRegime::WindyOvercast => engine_material::WeatherRegime::WindyOvercast,
        };
        self.sky_state.set_weather_regime(engine_regime);
        Ok(())
    }

    /// Lock weather authoring in local state
    pub fn lock_weather_authoring(&mut self, locked: bool) -> Result<(), String> {
        self.sky_state.lock_weather_authoring(locked);
        Ok(())
    }

    /// Set weather bias in local state
    pub fn set_weather_bias(&mut self, storm: f32, fog: f32, rain: f32) -> Result<(), String> {
        self.sky_state.set_weather_bias(storm, fog, rain);
        Ok(())
    }

    /// Create weather cell in local state
    pub fn create_weather_cell(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        density: f32,
    ) -> Result<u32, String> {
        let cell_id = self
            .sky_state
            .create_weather_cell(position, velocity, radius_km, density);
        Ok(cell_id)
    }

    /// Update weather cell in local state
    pub fn update_weather_cell(
        &mut self,
        cell_id: u32,
        precipitation_rate: Option<f32>,
        lightning_probability: Option<f32>,
    ) -> Result<(), String> {
        if self
            .sky_state
            .update_weather_cell(cell_id, precipitation_rate, lightning_probability)
        {
            Ok(())
        } else {
            Err(format!("Weather cell {} not found", cell_id))
        }
    }

    /// Remove weather cell from local state
    pub fn remove_weather_cell(&mut self, cell_id: u32) -> Result<(), String> {
        if self.sky_state.remove_weather_cell(cell_id) {
            Ok(())
        } else {
            Err(format!("Weather cell {} not found", cell_id))
        }
    }

    /// Set cloud shadow active in local state
    pub fn set_cloud_shadow_active(&mut self, active: bool) -> Result<(), String> {
        self.sky_state.set_cloud_shadow_active(active);
        Ok(())
    }

    /// Set cloud shadow posture in local state
    pub fn set_cloud_shadow_posture(&mut self, posture: CloudShadowPosture) -> Result<(), String> {
        let engine_posture = match posture {
            CloudShadowPosture::Full => engine_material::ShadowMapPosture::Full,
            CloudShadowPosture::Simplified => engine_material::ShadowMapPosture::Simplified,
            CloudShadowPosture::Fallback => engine_material::ShadowMapPosture::Fallback,
            CloudShadowPosture::Disabled => engine_material::ShadowMapPosture::Disabled,
        };
        self.sky_state.set_cloud_shadow_posture(engine_posture);
        Ok(())
    }
}

impl Default for WeatherEngineBridge {
    fn default() -> Self {
        Self::new()
    }
}
