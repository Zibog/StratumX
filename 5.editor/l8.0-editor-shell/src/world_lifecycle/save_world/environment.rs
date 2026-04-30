// Environment binding saving - canonical sky/weather persistence

use editor_dto_law::SkyBinding;
use engine_material::WeatherRegime;
use engine_world::ProofRegionScene;
use std::fs;
use std::path::Path;

pub fn save_environment_binding(path: &Path, scene: &ProofRegionScene) -> Result<(), String> {
    let env_dir = path.join("environment");
    fs::create_dir_all(&env_dir).map_err(|e| format!("Failed to create environment dir: {}", e))?;

    // CANONICAL: Save actual weather regime from scene, not hardcoded "Clear"
    let weather_regime_str = match scene.sky.weather_director.target_regime {
        WeatherRegime::Clear => "Clear",
        WeatherRegime::Scattered => "Scattered",
        WeatherRegime::Overcast => "Overcast",
        WeatherRegime::IncomingStorm => "IncomingStorm",
        WeatherRegime::HeavyStorm => "HeavyStorm",
        WeatherRegime::PostStormCalm => "PostStormCalm",
        WeatherRegime::FogMorning => "FogMorning",
        WeatherRegime::WindyOvercast => "WindyOvercast",
    };

    let sky_binding = SkyBinding {
        sky_bundle_ref: scene
            .sky_bundle_path
            .clone()
            .unwrap_or_else(|| "shared/sky/sky_bundle.json".to_string()),
        time_of_day: scene.sky.celestial.time_of_day_hours,
        day_of_year: scene.sky.celestial.day_of_year,
        latitude_deg: scene.sky.celestial.latitude_deg,
        weather_regime: weather_regime_str.to_string(),
        cloud_coverage: scene.sky.cloud_profile.coverage,
        fog_density: scene.sky.atmosphere.fog_density,
    };

    let sky_json = serde_json::to_string_pretty(&sky_binding)
        .map_err(|e| format!("Failed to serialize sky binding: {}", e))?;
    fs::write(env_dir.join("sky_binding.json"), sky_json)
        .map_err(|e| format!("Failed to write sky_binding.json: {}", e))?;

    Ok(())
}
