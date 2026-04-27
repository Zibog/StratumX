// Sky environment loading from package

use editor_dto_law::{SkyBinding, WorldPackageManifest};
use engine_material::{SkyWeatherState, WeatherRegime};
use std::fs;
use std::path::Path;

pub(super) fn load_sky_from_package(
    world_path: &Path,
    manifest: &WorldPackageManifest,
) -> Result<SkyWeatherState, String> {
    let mut sky_state = SkyWeatherState::new_default();

    if let Some(env_ref) = &manifest.environment_root_ref {
        let env_path = world_path.join(env_ref);
        if env_path.exists() {
            let env_json = fs::read_to_string(&env_path)
                .map_err(|e| format!("Failed to read sky binding: {}", e))?;
            let sky_binding: SkyBinding = serde_json::from_str(&env_json)
                .map_err(|e| format!("Failed to parse sky binding: {}", e))?;

            sky_state.celestial.time_of_day_hours = sky_binding.time_of_day;
            sky_state.celestial.day_of_year = sky_binding.day_of_year;
            sky_state.celestial.latitude_deg = sky_binding.latitude_deg;
            sky_state.cloud_profile.coverage = sky_binding.cloud_coverage;
            sky_state.atmosphere.fog_density = sky_binding.fog_density;

            // Load weather regime from saved state
            sky_state.weather_director.target_regime = match sky_binding.weather_regime.as_str() {
                "Clear" => WeatherRegime::Clear,
                "Scattered" => WeatherRegime::Scattered,
                "Overcast" => WeatherRegime::Overcast,
                "IncomingStorm" => WeatherRegime::IncomingStorm,
                "HeavyStorm" => WeatherRegime::HeavyStorm,
                "PostStormCalm" => WeatherRegime::PostStormCalm,
                "FogMorning" => WeatherRegime::FogMorning,
                "WindyOvercast" => WeatherRegime::WindyOvercast,
                _ => WeatherRegime::Scattered, // Default fallback
            };
            // Complete the transition immediately on load
            sky_state.weather_director.transition_progress = 1.0;
        }
    }

    Ok(sky_state)
}
