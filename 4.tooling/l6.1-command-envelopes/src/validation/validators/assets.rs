// Asset command validators (terrain, environment, sky)

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;
use crate::validation::rules::*;

pub fn validate_asset_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        // Terrain commands
        PromotedCommand::TerrainImport { heightmap_path } => {
            validate_path(heightmap_path, "heightmap_path")
        }
        PromotedCommand::TerrainRebuild => ValidationResult::Valid,
        PromotedCommand::TerrainSculptRaise {
            position,
            radius,
            strength,
        } => validate_terrain_sculpt(position, *radius, *strength),
        PromotedCommand::TerrainSculptLower {
            position,
            radius,
            strength,
        } => validate_terrain_sculpt(position, *radius, *strength),
        PromotedCommand::TerrainSculptSmooth {
            position,
            radius,
            strength,
        } => validate_terrain_sculpt(position, *radius, *strength),
        PromotedCommand::TerrainSculptFlatten {
            position,
            radius,
            strength,
            target_height: _,
        } => validate_terrain_sculpt(position, *radius, *strength),
        PromotedCommand::TerrainPaintMaterial {
            position,
            radius,
            strength,
            material_layer: _,
        } => validate_terrain_sculpt(position, *radius, *strength),
        PromotedCommand::TerrainSetLayerMaterial {
            layer_id: _,
            albedo_texture_path,
            uv_scale,
        } => validate_terrain_layer_material(albedo_texture_path, uv_scale),
        PromotedCommand::TerrainAddHole { position, radius } => {
            validate_terrain_hole(position, *radius)
        }
        PromotedCommand::TerrainRemoveHole { position, radius } => {
            validate_terrain_hole(position, *radius)
        }

        // Environment commands
        PromotedCommand::EnvironmentSetTime { time_of_day_hours } => {
            validate_time_of_day(*time_of_day_hours)
        }
        PromotedCommand::EnvironmentSetWeather { weather_regime } => {
            validate_non_empty(weather_regime, "weather_regime")
        }
        PromotedCommand::EnvironmentSetCloudCoverage { coverage } => {
            validate_range(*coverage, 0.0, 1.0, "coverage")
        }
        PromotedCommand::EnvironmentSetFogDensity { density } => {
            validate_positive(*density, "density")
        }

        // Sky commands
        PromotedCommand::SkyBindProfile { sky_profile } => {
            validate_non_empty(sky_profile, "sky_profile")
        }
        PromotedCommand::SkySetTimeOfDay { time_of_day_hours } => {
            validate_time_of_day(*time_of_day_hours)
        }
        PromotedCommand::SkySetWeatherRegime { weather_regime } => {
            validate_non_empty(weather_regime, "weather_regime")
        }
        PromotedCommand::SkyBindCloudProfile { cloud_profile } => {
            validate_non_empty(cloud_profile, "cloud_profile")
        }

        _ => ValidationResult::Valid,
    }
}
