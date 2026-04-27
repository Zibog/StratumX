// Command payload validators - main entry point

use super::errors::ValidationResult;
use super::rules::*;
use crate::promoted_commands::PromotedCommand;

/// Validate a promoted command
pub fn validate_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        // Project commands
        PromotedCommand::ProjectBootstrap { project_name } => validate_project_name(project_name),
        PromotedCommand::ProjectCreate {
            project_name,
            project_root,
            world_name,
        } => validate_project_create(project_name, project_root, world_name),
        PromotedCommand::ProjectSave { save_path } => validate_path(save_path, "save_path"),
        PromotedCommand::ProjectBuild { target_platform } => {
            validate_non_empty(target_platform, "target_platform")
        }
        PromotedCommand::ProjectExport { export_path } => validate_path(export_path, "export_path"),
        PromotedCommand::ProjectLaunch { launch_mode } => {
            validate_non_empty(launch_mode, "launch_mode")
        }
        PromotedCommand::ProjectVerifyFirstResult => ValidationResult::Valid,

        // World commands
        PromotedCommand::WorldOpen { world_path } => validate_path(world_path, "world_path"),
        PromotedCommand::WorldSave { world_path } => validate_path(world_path, "world_path"),
        PromotedCommand::WorldClose => ValidationResult::Valid,

        // Runtime commands
        PromotedCommand::RuntimePlay
        | PromotedCommand::RuntimePause
        | PromotedCommand::RuntimeStop
        | PromotedCommand::RuntimeSimulate => ValidationResult::Valid,

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

        // Shell commands
        PromotedCommand::ShellActivateViewport
        | PromotedCommand::ShellActivateOutliner
        | PromotedCommand::ShellActivateInspector
        | PromotedCommand::ShellActivateContentBrowser
        | PromotedCommand::ShellActivateMaterialLab
        | PromotedCommand::ShellActivateTerrainLab
        | PromotedCommand::ShellActivateSkyLab => ValidationResult::Valid,

        // Material authority commands
        PromotedCommand::MaterialAuthorityInitialize
        | PromotedCommand::MaterialAuthorityDispose => ValidationResult::Valid,

        // Audio authority commands
        PromotedCommand::AudioAuthorityInitialize | PromotedCommand::AudioAuthorityDispose => {
            ValidationResult::Valid
        }

        // Material commands
        PromotedCommand::MaterialCreate { material_name } => {
            validate_non_empty(material_name, "material_name")
        }
        PromotedCommand::MaterialDelete { material_id } => {
            validate_non_empty(material_id, "material_id")
        }
        PromotedCommand::MaterialDuplicateProfile {
            source_material_id,
            target_name,
        } => validate_material_duplicate(source_material_id, target_name),
        PromotedCommand::MaterialBindVisualResponse {
            material_id,
            visual_family,
        } => validate_material_binding(material_id, visual_family),
        PromotedCommand::MaterialBindAcousticProfile {
            material_id,
            acoustic_profile,
        } => validate_material_binding(material_id, acoustic_profile),
        PromotedCommand::MaterialBindLightResponse {
            material_id,
            light_response,
        } => validate_material_binding(material_id, light_response),
        PromotedCommand::MaterialBindMicrodetailProfile {
            material_id,
            microdetail_profile,
        } => validate_material_binding(material_id, microdetail_profile),
        PromotedCommand::MaterialBindWeatherModulation {
            material_id,
            weather_modulation,
        } => validate_material_binding(material_id, weather_modulation),
        PromotedCommand::MaterialPreviewBurn {
            material_id,
            preview_target,
        } => validate_material_binding(material_id, preview_target),
        PromotedCommand::MaterialSetCheapRuntimeRung {
            material_id,
            rung_level,
        } => validate_material_rung(material_id, *rung_level),
        PromotedCommand::MaterialInspectBranchCoverage { material_id } => {
            validate_non_empty(material_id, "material_id")
        }

        // Audio commands
        PromotedCommand::AudioCreateSource { source_name } => {
            validate_non_empty(source_name, "source_name")
        }
        PromotedCommand::AudioBindWorldSource {
            source_id,
            position,
        } => validate_audio_source(source_id, position),
        PromotedCommand::AudioSetAcousticProfile { source_id, profile } => {
            validate_audio_binding(source_id, profile)
        }
        PromotedCommand::AudioAssignEmitterClassWorldSource {
            source_id,
            emitter_class,
        } => validate_audio_binding(source_id, emitter_class),
        PromotedCommand::AudioBindZoneProfileWorldSurface {
            zone_id,
            reverb_profile,
        } => validate_audio_binding(zone_id, reverb_profile),
        PromotedCommand::AudioBindPriorityDuckingPolicy { policy_id } => {
            validate_non_empty(policy_id, "policy_id")
        }
        PromotedCommand::AudioPreviewAudibilityFreeCamera { listener_profile } => {
            validate_non_empty(listener_profile, "listener_profile")
        }
        PromotedCommand::AudioPreviewObstructionVsOcclusion { path_id } => {
            validate_non_empty(path_id, "path_id")
        }
        PromotedCommand::AudioPreviewIndoorOutdoorTransition { transition_path } => {
            validate_non_empty(transition_path, "transition_path")
        }
        PromotedCommand::AudioPreviewVoiceSubtitleLegality { dialogue_id } => {
            validate_non_empty(dialogue_id, "dialogue_id")
        }

        // Build and validation commands
        PromotedCommand::BuildRun
        | PromotedCommand::BuildRelease
        | PromotedCommand::ValidationRunFull
        | PromotedCommand::ValidationRunSmoke => ValidationResult::Valid,

        // Automation commands
        PromotedCommand::AutomationRebuildAll | PromotedCommand::AutomationValidateAll => {
            ValidationResult::Valid
        }

        // Scene commands
        PromotedCommand::SceneBootstrap | PromotedCommand::SceneReset => ValidationResult::Valid,
        PromotedCommand::SceneFireTestShot {
            weapon_entity_id: _,
        } => ValidationResult::Valid,
    }
}
