//! Environment Command Executor
//!
//! Executes environment-related commands (time, weather, clouds, fog) by mutating world state.
//! This is the ONLY place where environment mutations should occur from the editor layer.
//!
//! **Phase 06 Remediation**: Logic relocated from desktop_app/environment_world_ops.rs
//! to establish correct architectural boundary.

use editor_dto_law::WeatherRegime;

/// Environment command executor
///
/// Executes environment commands by mutating world state through the runtime host.
/// All environment mutations flow through this executor to maintain architectural boundaries.
pub struct EnvironmentCommandExecutor {
    // Will be wired to runtime host's world state
}

impl EnvironmentCommandExecutor {
    /// Create a new environment command executor
    pub fn new() -> Self {
        Self {}
    }

    /// Set the time of day in the active world
    ///
    /// Sets the time of day in the active world's sky system.
    ///
    /// # Arguments
    ///
    /// * `runtime_host` - Mutable reference to runtime host for world state access
    /// * `time_of_day_hours` - Time of day in hours (0.0 - 24.0)
    ///
    /// # Returns
    ///
    /// Success message or error
    pub fn set_time(
        &self,
        runtime_host: &mut dyn RuntimeHostAccess,
        time_of_day_hours: f32,
    ) -> Result<String, String> {
        let scene = runtime_host
            .get_world_state_mut()
            .and_then(|world| world.vertical_slice_scene_mut())
            .ok_or_else(|| "No world loaded".to_string())?;

        scene.sky.set_time_of_day(time_of_day_hours);
        Ok(format!("Environment time set to {:.2}h", time_of_day_hours))
    }

    /// Set the weather regime in the active world
    ///
    /// Sets the weather regime in the active world's sky system.
    ///
    /// # Arguments
    ///
    /// * `runtime_host` - Mutable reference to runtime host for world state access
    /// * `weather_regime` - Weather regime string (e.g., "Clear", "Overcast")
    ///
    /// # Returns
    ///
    /// Success message or error
    pub fn set_weather(
        &self,
        runtime_host: &mut dyn RuntimeHostAccess,
        weather_regime: &str,
    ) -> Result<String, String> {
        let scene = runtime_host
            .get_world_state_mut()
            .and_then(|world| world.vertical_slice_scene_mut())
            .ok_or_else(|| "No world loaded".to_string())?;

        let regime = parse_weather_regime(weather_regime)?;
        scene
            .sky
            .set_weather_regime(map_editor_weather_to_engine(regime));
        Ok(format!("Weather set to {}", weather_regime))
    }

    /// Set cloud coverage in the active world
    ///
    /// Sets the cloud coverage in the active world's sky system.
    ///
    /// # Arguments
    ///
    /// * `runtime_host` - Mutable reference to runtime host for world state access
    /// * `coverage` - Cloud coverage (0.0 - 1.0)
    ///
    /// # Returns
    ///
    /// Success message or error
    pub fn set_cloud_coverage(
        &self,
        runtime_host: &mut dyn RuntimeHostAccess,
        coverage: f32,
    ) -> Result<String, String> {
        let scene = runtime_host
            .get_world_state_mut()
            .and_then(|world| world.vertical_slice_scene_mut())
            .ok_or_else(|| "No world loaded".to_string())?;

        scene.sky.set_cloud_coverage(coverage);
        Ok(format!("Cloud coverage set to {:.2}", coverage))
    }

    /// Set fog density in the active world
    ///
    /// Sets the fog density in the active world's sky system.
    ///
    /// # Arguments
    ///
    /// * `runtime_host` - Mutable reference to runtime host for world state access
    /// * `density` - Fog density (0.0 - 1.0)
    ///
    /// # Returns
    ///
    /// Success message or error
    pub fn set_fog_density(
        &self,
        runtime_host: &mut dyn RuntimeHostAccess,
        density: f32,
    ) -> Result<String, String> {
        let scene = runtime_host
            .get_world_state_mut()
            .and_then(|world| world.vertical_slice_scene_mut())
            .ok_or_else(|| "No world loaded".to_string())?;

        scene.sky.set_fog_density(density);
        Ok(format!("Fog density set to {:.2}", density))
    }
}

/// Trait for runtime host access
///
/// Abstracts world state access to allow testing and decoupling from concrete runtime host.
pub trait RuntimeHostAccess {
    fn get_world_state_mut(&mut self) -> Option<&mut engine_world::WorldState>;
}

/// Parse weather regime string to enum
fn parse_weather_regime(value: &str) -> Result<WeatherRegime, String> {
    match value {
        "Clear" => Ok(WeatherRegime::Clear),
        "Scattered" => Ok(WeatherRegime::Scattered),
        "Overcast" => Ok(WeatherRegime::Overcast),
        "IncomingStorm" => Ok(WeatherRegime::IncomingStorm),
        "HeavyStorm" => Ok(WeatherRegime::HeavyStorm),
        "PostStormCalm" => Ok(WeatherRegime::PostStormCalm),
        "FogMorning" => Ok(WeatherRegime::FogMorning),
        "WindyOvercast" => Ok(WeatherRegime::WindyOvercast),
        _ => Err(format!("Unsupported weather regime: {}", value)),
    }
}

/// Map editor weather regime to engine weather regime
fn map_editor_weather_to_engine(value: WeatherRegime) -> engine_material::WeatherRegime {
    match value {
        WeatherRegime::Clear => engine_material::WeatherRegime::Clear,
        WeatherRegime::Scattered => engine_material::WeatherRegime::Scattered,
        WeatherRegime::Overcast => engine_material::WeatherRegime::Overcast,
        WeatherRegime::IncomingStorm => engine_material::WeatherRegime::IncomingStorm,
        WeatherRegime::HeavyStorm => engine_material::WeatherRegime::HeavyStorm,
        WeatherRegime::PostStormCalm => engine_material::WeatherRegime::PostStormCalm,
        WeatherRegime::FogMorning => engine_material::WeatherRegime::FogMorning,
        WeatherRegime::WindyOvercast => engine_material::WeatherRegime::WindyOvercast,
    }
}

