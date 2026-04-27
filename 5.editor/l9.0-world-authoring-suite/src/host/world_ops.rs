//! World package open/save helpers for the launch contour.
//!
//! Note: The EditorHost-specific methods (open_world_from_path, save_world_to_path, etc.)
//! are implemented in the apps crate (6.apps/editor/stratumx_editor_app) where EditorHost
//! is defined, because Rust does not allow impl blocks on foreign types.
//!
//! This module provides the weather regime parsing utility used by both crates.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedWorldPackage {
    pub world_name: String,
    pub version: String,
    pub terrain: SavedTerrainPackage,
    #[serde(default)]
    pub environment: SavedEnvironmentPackage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedTerrainPackage {
    pub resolution: [u32; 2],
    pub world_size: [f32; 2],
    pub chunk_grid: [u32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedEnvironmentPackage {
    #[serde(default = "default_weather")]
    pub weather_regime: String,
    #[serde(default = "default_time_of_day")]
    pub time_of_day_hours: f32,
    #[serde(default = "default_cloud_coverage")]
    pub cloud_coverage: f32,
}

fn default_weather() -> String {
    "Scattered".to_string()
}

fn default_time_of_day() -> f32 {
    14.0
}

fn default_cloud_coverage() -> f32 {
    0.35
}

impl Default for SavedEnvironmentPackage {
    fn default() -> Self {
        Self {
            weather_regime: default_weather(),
            time_of_day_hours: default_time_of_day(),
            cloud_coverage: default_cloud_coverage(),
        }
    }
}

/// Parse a weather regime string into the engine type.
pub fn parse_weather_regime(value: &str) -> Result<engine_material::WeatherRegime, String> {
    match value {
        "Clear" => Ok(engine_material::WeatherRegime::Clear),
        "Scattered" => Ok(engine_material::WeatherRegime::Scattered),
        "Overcast" => Ok(engine_material::WeatherRegime::Overcast),
        "IncomingStorm" => Ok(engine_material::WeatherRegime::IncomingStorm),
        "HeavyStorm" => Ok(engine_material::WeatherRegime::HeavyStorm),
        "PostStormCalm" => Ok(engine_material::WeatherRegime::PostStormCalm),
        "FogMorning" => Ok(engine_material::WeatherRegime::FogMorning),
        "WindyOvercast" => Ok(engine_material::WeatherRegime::WindyOvercast),
        _ => Err(format!("Unsupported weather regime: {}", value)),
    }
}
