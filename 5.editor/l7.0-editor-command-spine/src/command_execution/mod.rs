//! Command Execution - Phase 06 Remediation
//!
//! This module provides command execution utilities that can be used with EditorHost.
//! The EditorHost-specific methods are implemented as functions that take &mut EditorHost,
//! since Rust does not allow impl blocks on foreign types.
//!
//! All world mutations flow through executors to maintain architectural boundaries.

use std::path::PathBuf;

// Re-export weather mapping utilities
pub use weather::{
    map_editor_weather_to_engine, map_engine_weather_to_editor, parse_weather_regime,
};

mod weather {
    /// Parse a weather regime string into the editor DTO type.
    pub fn parse_weather_regime(value: &str) -> Result<editor_dto_law::WeatherRegime, String> {
        match value {
            "Clear" => Ok(editor_dto_law::WeatherRegime::Clear),
            "Scattered" => Ok(editor_dto_law::WeatherRegime::Scattered),
            "Overcast" => Ok(editor_dto_law::WeatherRegime::Overcast),
            "IncomingStorm" => Ok(editor_dto_law::WeatherRegime::IncomingStorm),
            "HeavyStorm" => Ok(editor_dto_law::WeatherRegime::HeavyStorm),
            "PostStormCalm" => Ok(editor_dto_law::WeatherRegime::PostStormCalm),
            "FogMorning" => Ok(editor_dto_law::WeatherRegime::FogMorning),
            "WindyOvercast" => Ok(editor_dto_law::WeatherRegime::WindyOvercast),
            _ => Err(format!("Unsupported weather regime: {}", value)),
        }
    }

    pub fn map_editor_weather_to_engine(
        value: editor_dto_law::WeatherRegime,
    ) -> engine_material::WeatherRegime {
        match value {
            editor_dto_law::WeatherRegime::Clear => engine_material::WeatherRegime::Clear,
            editor_dto_law::WeatherRegime::Scattered => engine_material::WeatherRegime::Scattered,
            editor_dto_law::WeatherRegime::Overcast => engine_material::WeatherRegime::Overcast,
            editor_dto_law::WeatherRegime::IncomingStorm => {
                engine_material::WeatherRegime::IncomingStorm
            }
            editor_dto_law::WeatherRegime::HeavyStorm => engine_material::WeatherRegime::HeavyStorm,
            editor_dto_law::WeatherRegime::PostStormCalm => {
                engine_material::WeatherRegime::PostStormCalm
            }
            editor_dto_law::WeatherRegime::FogMorning => engine_material::WeatherRegime::FogMorning,
            editor_dto_law::WeatherRegime::WindyOvercast => {
                engine_material::WeatherRegime::WindyOvercast
            }
        }
    }

    /// Map engine weather regime to editor DTO
    pub fn map_engine_weather_to_editor(
        value: engine_material::WeatherRegime,
    ) -> editor_dto_law::WeatherRegime {
        match value {
            engine_material::WeatherRegime::Clear => editor_dto_law::WeatherRegime::Clear,
            engine_material::WeatherRegime::Scattered => editor_dto_law::WeatherRegime::Scattered,
            engine_material::WeatherRegime::Overcast => editor_dto_law::WeatherRegime::Overcast,
            engine_material::WeatherRegime::IncomingStorm => {
                editor_dto_law::WeatherRegime::IncomingStorm
            }
            engine_material::WeatherRegime::HeavyStorm => editor_dto_law::WeatherRegime::HeavyStorm,
            engine_material::WeatherRegime::PostStormCalm => {
                editor_dto_law::WeatherRegime::PostStormCalm
            }
            engine_material::WeatherRegime::FogMorning => editor_dto_law::WeatherRegime::FogMorning,
            engine_material::WeatherRegime::WindyOvercast => {
                editor_dto_law::WeatherRegime::WindyOvercast
            }
        }
    }
}

// Heightmap decoding functions
pub fn decode_raw_heightmap(data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
    let total_pixels = data.len();
    let size = (total_pixels as f64).sqrt() as u32;
    if (size * size) as usize != total_pixels {
        return Err("RAW heightmap must be square".to_string());
    }

    let samples = data
        .iter()
        .map(|value| *value as f32 / 255.0 * 1000.0)
        .collect();
    Ok((samples, size, size))
}

pub fn decode_r16_heightmap(data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
    if !data.len().is_multiple_of(2) {
        return Err("R16 heightmap byte count must be even".to_string());
    }

    let total_pixels = data.len() / 2;
    let size = (total_pixels as f64).sqrt() as u32;
    if (size * size) as usize != total_pixels {
        return Err("R16 heightmap must be square".to_string());
    }

    let mut samples = Vec::with_capacity(total_pixels);
    for index in 0..total_pixels {
        let offset = index * 2;
        let value = u16::from_le_bytes([data[offset], data[offset + 1]]);
        samples.push(value as f32 / 65535.0 * 1000.0);
    }
    Ok((samples, size, size))
}

pub fn decode_png_heightmap(data: &[u8]) -> Result<(Vec<f32>, u32, u32), String> {
    let image = image::load_from_memory(data)
        .map_err(|error| format!("Failed to decode PNG heightmap: {}", error))?;
    let grayscale = image.to_luma8();
    let width = grayscale.width();
    let height = grayscale.height();
    let samples = grayscale
        .pixels()
        .map(|pixel| pixel[0] as f32 / 255.0 * 1000.0)
        .collect();
    Ok((samples, width, height))
}

/// Create a project shell with an initial world (Phase 08)
pub fn create_project_with_world(
    project_name: &str,
    project_root: &str,
    world_name: &str,
) -> Result<PathBuf, String> {
    // Create directory structure
    let project_dir = PathBuf::from(project_root).join(project_name);
    let world_dir = project_dir.join("worlds").join(world_name);

    std::fs::create_dir_all(&world_dir)
        .map_err(|e| format!("Failed to create project structure: {}", e))?;

    // Create world.json
    let world_json = serde_json::json!({
        "world_name": world_name,
        "version": "0.1.0",
        "terrain": {
            "resolution": [256, 256],
            "world_size": [1000.0, 1000.0],
            "chunk_grid": [4, 4]
        },
        "environment": {
            "weather_regime": "Scattered",
            "time_of_day_hours": 14.0,
            "cloud_coverage": 0.35
        }
    });

    let world_json_path = world_dir.join("world.json");
    std::fs::write(
        &world_json_path,
        serde_json::to_string_pretty(&world_json).unwrap(),
    )
    .map_err(|e| format!("Failed to write world.json: {}", e))?;

    Ok(world_dir)
}
