// Command validators organized by domain

pub mod assets;
pub mod audio;
pub mod build;
pub mod editor;
pub mod graphics;
pub mod materials;
pub mod project;
pub mod runtime;

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;

/// Main entry point for command validation
pub fn validate_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        // Project commands
        PromotedCommand::ProjectBootstrap { .. }
        | PromotedCommand::ProjectCreate { .. }
        | PromotedCommand::ProjectSave { .. }
        | PromotedCommand::ProjectBuild { .. }
        | PromotedCommand::ProjectExport { .. }
        | PromotedCommand::ProjectLaunch { .. }
        | PromotedCommand::ProjectVerifyFirstResult => project::validate_project_command(cmd),

        // World and Runtime commands
        PromotedCommand::WorldOpen { .. }
        | PromotedCommand::WorldSave { .. }
        | PromotedCommand::WorldClose
        | PromotedCommand::RuntimePlay
        | PromotedCommand::RuntimePause
        | PromotedCommand::RuntimeStop
        | PromotedCommand::RuntimeSimulate => runtime::validate_runtime_command(cmd),

        // Terrain, Environment, Sky commands
        PromotedCommand::TerrainImport { .. }
        | PromotedCommand::TerrainRebuild
        | PromotedCommand::TerrainSculptRaise { .. }
        | PromotedCommand::TerrainSculptLower { .. }
        | PromotedCommand::TerrainSculptSmooth { .. }
        | PromotedCommand::TerrainSculptFlatten { .. }
        | PromotedCommand::TerrainPaintMaterial { .. }
        | PromotedCommand::TerrainSetLayerMaterial { .. }
        | PromotedCommand::TerrainAddHole { .. }
        | PromotedCommand::TerrainRemoveHole { .. }
        | PromotedCommand::EnvironmentSetTime { .. }
        | PromotedCommand::EnvironmentSetWeather { .. }
        | PromotedCommand::EnvironmentSetCloudCoverage { .. }
        | PromotedCommand::EnvironmentSetFogDensity { .. }
        | PromotedCommand::SkyBindProfile { .. }
        | PromotedCommand::SkySetTimeOfDay { .. }
        | PromotedCommand::SkySetWeatherRegime { .. }
        | PromotedCommand::SkyBindCloudProfile { .. } => assets::validate_asset_command(cmd),

        // Graphics/Shell commands
        PromotedCommand::ShellActivateViewport
        | PromotedCommand::ShellActivateOutliner
        | PromotedCommand::ShellActivateInspector
        | PromotedCommand::ShellActivateContentBrowser
        | PromotedCommand::ShellActivateMaterialLab
        | PromotedCommand::ShellActivateTerrainLab
        | PromotedCommand::ShellActivateSkyLab => graphics::validate_graphics_command(cmd),

        // Material commands
        PromotedCommand::MaterialAuthorityInitialize
        | PromotedCommand::MaterialAuthorityDispose
        | PromotedCommand::MaterialCreate { .. }
        | PromotedCommand::MaterialDelete { .. }
        | PromotedCommand::MaterialDuplicateProfile { .. }
        | PromotedCommand::MaterialBindVisualResponse { .. }
        | PromotedCommand::MaterialBindAcousticProfile { .. }
        | PromotedCommand::MaterialBindLightResponse { .. }
        | PromotedCommand::MaterialBindMicrodetailProfile { .. }
        | PromotedCommand::MaterialBindWeatherModulation { .. }
        | PromotedCommand::MaterialPreviewBurn { .. }
        | PromotedCommand::MaterialSetCheapRuntimeRung { .. }
        | PromotedCommand::MaterialInspectBranchCoverage { .. } => {
            materials::validate_material_command(cmd)
        }

        // Audio commands
        PromotedCommand::AudioAuthorityInitialize
        | PromotedCommand::AudioAuthorityDispose
        | PromotedCommand::AudioCreateSource { .. }
        | PromotedCommand::AudioBindWorldSource { .. }
        | PromotedCommand::AudioSetAcousticProfile { .. }
        | PromotedCommand::AudioAssignEmitterClassWorldSource { .. }
        | PromotedCommand::AudioBindZoneProfileWorldSurface { .. }
        | PromotedCommand::AudioBindPriorityDuckingPolicy { .. }
        | PromotedCommand::AudioPreviewAudibilityFreeCamera { .. }
        | PromotedCommand::AudioPreviewObstructionVsOcclusion { .. }
        | PromotedCommand::AudioPreviewIndoorOutdoorTransition { .. }
        | PromotedCommand::AudioPreviewVoiceSubtitleLegality { .. } => {
            audio::validate_audio_command(cmd)
        }

        // Build, validation, automation, scene commands
        PromotedCommand::BuildRun
        | PromotedCommand::BuildRelease
        | PromotedCommand::ValidationRunFull
        | PromotedCommand::ValidationRunSmoke
        | PromotedCommand::AutomationRebuildAll
        | PromotedCommand::AutomationValidateAll
        | PromotedCommand::SceneBootstrap
        | PromotedCommand::SceneReset
        | PromotedCommand::SceneFireTestShot { .. } => build::validate_build_command(cmd),
    }
}
