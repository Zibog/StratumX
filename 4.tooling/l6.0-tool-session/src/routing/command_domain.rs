//! Command domain classification.
//!
//! Each promoted command belongs to exactly one domain.

use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub enum CommandDomain {
    Project,
    World,
    Runtime,
    Terrain,
    Environment,
    Material,
    Audio,
    Build,
    Shell,
    Scene,
}

impl CommandDomain {
    pub fn classify(command: &PromotedCommand) -> Self {
        match command {
            PromotedCommand::WorldOpen { .. }
            | PromotedCommand::WorldSave { .. }
            | PromotedCommand::WorldClose => Self::World,

            PromotedCommand::RuntimePlay
            | PromotedCommand::RuntimePause
            | PromotedCommand::RuntimeStop
            | PromotedCommand::RuntimeSimulate => Self::Runtime,

            PromotedCommand::TerrainImport { .. }
            | PromotedCommand::TerrainRebuild
            | PromotedCommand::TerrainSculptRaise { .. }
            | PromotedCommand::TerrainSculptLower { .. }
            | PromotedCommand::TerrainSculptSmooth { .. }
            | PromotedCommand::TerrainSculptFlatten { .. }
            | PromotedCommand::TerrainPaintMaterial { .. }
            | PromotedCommand::TerrainSetLayerMaterial { .. }
            | PromotedCommand::TerrainAddHole { .. }
            | PromotedCommand::TerrainRemoveHole { .. } => Self::Terrain,

            PromotedCommand::EnvironmentSetTime { .. }
            | PromotedCommand::EnvironmentSetWeather { .. }
            | PromotedCommand::EnvironmentSetCloudCoverage { .. }
            | PromotedCommand::EnvironmentSetFogDensity { .. }
            | PromotedCommand::SkyBindProfile { .. }
            | PromotedCommand::SkySetTimeOfDay { .. }
            | PromotedCommand::SkySetWeatherRegime { .. }
            | PromotedCommand::SkyBindCloudProfile { .. } => Self::Environment,

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
            | PromotedCommand::MaterialInspectBranchCoverage { .. } => Self::Material,

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
            | PromotedCommand::AudioPreviewVoiceSubtitleLegality { .. } => Self::Audio,

            PromotedCommand::BuildRun
            | PromotedCommand::BuildRelease
            | PromotedCommand::ValidationRunFull
            | PromotedCommand::ValidationRunSmoke
            | PromotedCommand::AutomationRebuildAll
            | PromotedCommand::AutomationValidateAll => Self::Build,

            PromotedCommand::ProjectBootstrap { .. }
            | PromotedCommand::ProjectCreate { .. }
            | PromotedCommand::ProjectSave { .. }
            | PromotedCommand::ProjectBuild { .. }
            | PromotedCommand::ProjectExport { .. }
            | PromotedCommand::ProjectLaunch { .. }
            | PromotedCommand::ProjectVerifyFirstResult => Self::Project,

            PromotedCommand::ShellActivateViewport
            | PromotedCommand::ShellActivateOutliner
            | PromotedCommand::ShellActivateInspector
            | PromotedCommand::ShellActivateContentBrowser
            | PromotedCommand::ShellActivateMaterialLab
            | PromotedCommand::ShellActivateTerrainLab
            | PromotedCommand::ShellActivateSkyLab => Self::Shell,

            PromotedCommand::SceneBootstrap
            | PromotedCommand::SceneFireTestShot { .. }
            | PromotedCommand::SceneReset => Self::Scene,
        }
    }
}
