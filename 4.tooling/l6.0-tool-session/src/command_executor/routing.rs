// Command Executor Routing - promoted command routing logic
//
// All SDK bridge calls are delegated to canonical ingress packet handlers
// in link_ingress_packets crate. This module only routes to domain executors.

use super::core::CommandExecutor;
use crate::{ToolingError, ToolingRuntime};
use link_ingress_packets::VerticalSliceIngressPacket;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn route_promoted_command(
    executor: &mut CommandExecutor,
    command_id: u64,
    command: PromotedCommand,
    runtime: &mut ToolingRuntime,
) -> Result<Vec<u8>, ToolingError> {
    // Route command to appropriate domain executor
    match &command {
        // World lifecycle commands
        PromotedCommand::WorldOpen { .. }
        | PromotedCommand::WorldSave { .. }
        | PromotedCommand::WorldClose => executor.world_executor.execute(command, runtime),

        // Runtime control commands
        PromotedCommand::RuntimePlay
        | PromotedCommand::RuntimePause
        | PromotedCommand::RuntimeStop
        | PromotedCommand::RuntimeSimulate => executor.runtime_executor.execute(command, runtime),

        // Terrain manipulation commands
        PromotedCommand::TerrainImport { .. }
        | PromotedCommand::TerrainRebuild
        | PromotedCommand::TerrainSculptRaise { .. }
        | PromotedCommand::TerrainSculptLower { .. }
        | PromotedCommand::TerrainSculptSmooth { .. }
        | PromotedCommand::TerrainSculptFlatten { .. }
        | PromotedCommand::TerrainPaintMaterial { .. }
        | PromotedCommand::TerrainSetLayerMaterial { .. }
        | PromotedCommand::TerrainAddHole { .. }
        | PromotedCommand::TerrainRemoveHole { .. } => {
            executor.terrain_executor.execute(command, runtime)
        }

        // Environment (sky/weather/time) commands
        PromotedCommand::EnvironmentSetTime { .. }
        | PromotedCommand::EnvironmentSetWeather { .. }
        | PromotedCommand::EnvironmentSetCloudCoverage { .. }
        | PromotedCommand::EnvironmentSetFogDensity { .. }
        | PromotedCommand::SkyBindProfile { .. }
        | PromotedCommand::SkySetTimeOfDay { .. }
        | PromotedCommand::SkySetWeatherRegime { .. }
        | PromotedCommand::SkyBindCloudProfile { .. } => {
            executor.environment_executor.execute(command, runtime)
        }

        // Material authoring commands
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
            executor.material_executor.execute(command, runtime)
        }

        // Audio authoring commands
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
            executor.audio_executor.execute(command, runtime)
        }

        // Build and validation commands
        PromotedCommand::BuildRun
        | PromotedCommand::BuildRelease
        | PromotedCommand::ValidationRunFull
        | PromotedCommand::ValidationRunSmoke
        | PromotedCommand::AutomationRebuildAll
        | PromotedCommand::AutomationValidateAll => {
            executor.build_executor.execute(command, runtime)
        }

        // Shell and project commands
        PromotedCommand::ProjectBootstrap { .. }
        | PromotedCommand::ProjectCreate { .. }
        | PromotedCommand::ProjectSave { .. }
        | PromotedCommand::ProjectBuild { .. }
        | PromotedCommand::ProjectExport { .. }
        | PromotedCommand::ProjectLaunch { .. }
        | PromotedCommand::ProjectVerifyFirstResult
        | PromotedCommand::ShellActivateViewport
        | PromotedCommand::ShellActivateOutliner
        | PromotedCommand::ShellActivateInspector
        | PromotedCommand::ShellActivateContentBrowser
        | PromotedCommand::ShellActivateMaterialLab
        | PromotedCommand::ShellActivateTerrainLab
        | PromotedCommand::ShellActivateSkyLab => executor.shell_executor.execute(command, runtime),

        // Scene commands (legacy vertical slice) - delegated to packet executor
        // through the canonical SDK ingress boundary and real preview runtime session.
        PromotedCommand::SceneBootstrap => {
            let observation = executor
                .packet_executor
                .execute_packet(VerticalSliceIngressPacket::bootstrap_scene(command_id))
                .map_err(ToolingError::Message)?;
            serde_json::to_vec(&observation).map_err(|error| {
                ToolingError::Message(format!(
                    "scene observation serialization failed: {}",
                    error
                ))
            })
        }
        PromotedCommand::SceneFireTestShot { weapon_entity_id } => {
            let observation = executor
                .packet_executor
                .execute_packet(VerticalSliceIngressPacket::fire_test_shot(
                    command_id,
                    *weapon_entity_id,
                ))
                .map_err(ToolingError::Message)?;
            serde_json::to_vec(&observation).map_err(|error| {
                ToolingError::Message(format!(
                    "scene observation serialization failed: {}",
                    error
                ))
            })
        }
        PromotedCommand::SceneReset => {
            let observation = executor
                .packet_executor
                .execute_packet(VerticalSliceIngressPacket::reset_scene(command_id))
                .map_err(ToolingError::Message)?;
            serde_json::to_vec(&observation).map_err(|error| {
                ToolingError::Message(format!(
                    "scene observation serialization failed: {}",
                    error
                ))
            })
        }
    }
}
