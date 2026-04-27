use serde::{Deserialize, Serialize};

/// Unified promoted command enum that wraps all domain-specific command families
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PromotedCommand {
    // Project lifecycle
    ProjectBootstrap {
        project_name: String,
    },
    ProjectCreate {
        project_name: String,
        project_root: String,
        world_name: String,
    },
    ProjectSave {
        save_path: String,
    },
    ProjectBuild {
        target_platform: String,
    },
    ProjectExport {
        export_path: String,
    },
    ProjectLaunch {
        launch_mode: String,
    },
    ProjectVerifyFirstResult,
    // World lifecycle
    WorldOpen {
        world_path: String,
    },
    WorldSave {
        world_path: String,
    },
    WorldClose,
    // Runtime control
    RuntimePlay,
    RuntimePause,
    RuntimeStop,
    RuntimeSimulate,
    // Terrain authoring
    TerrainImport {
        heightmap_path: String,
    },
    TerrainRebuild,
    TerrainSculptRaise {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
    TerrainSculptLower {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
    TerrainSculptSmooth {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
    TerrainSculptFlatten {
        position: [f32; 2],
        radius: f32,
        strength: f32,
        target_height: f32,
    },
    TerrainPaintMaterial {
        position: [f32; 2],
        radius: f32,
        strength: f32,
        material_layer: u32,
    },
    TerrainSetLayerMaterial {
        layer_id: u16,
        albedo_texture_path: String,
        uv_scale: [f32; 2],
    },
    TerrainAddHole {
        position: [f32; 2],
        radius: f32,
    },
    TerrainRemoveHole {
        position: [f32; 2],
        radius: f32,
    },
    // Environment authoring
    EnvironmentSetTime {
        time_of_day_hours: f32,
    },
    EnvironmentSetWeather {
        weather_regime: String,
    },
    EnvironmentSetCloudCoverage {
        coverage: f32,
    },
    EnvironmentSetFogDensity {
        density: f32,
    },
    // Sky authoring
    SkyBindProfile {
        sky_profile: String,
    },
    SkySetTimeOfDay {
        time_of_day_hours: f32,
    },
    SkySetWeatherRegime {
        weather_regime: String,
    },
    SkyBindCloudProfile {
        cloud_profile: String,
    },
    // Shell surface activation
    ShellActivateViewport,
    ShellActivateOutliner,
    ShellActivateInspector,
    ShellActivateContentBrowser,
    ShellActivateMaterialLab,
    ShellActivateTerrainLab,
    ShellActivateSkyLab,
    // Material authority lifecycle
    MaterialAuthorityInitialize,
    MaterialAuthorityDispose,
    // Audio authority lifecycle
    AudioAuthorityInitialize,
    AudioAuthorityDispose,
    // Material authoring
    MaterialCreate {
        material_name: String,
    },
    MaterialDelete {
        material_id: String,
    },
    MaterialDuplicateProfile {
        source_material_id: String,
        target_name: String,
    },
    MaterialBindVisualResponse {
        material_id: String,
        visual_family: String,
    },
    MaterialBindAcousticProfile {
        material_id: String,
        acoustic_profile: String,
    },
    MaterialBindLightResponse {
        material_id: String,
        light_response: String,
    },
    MaterialBindMicrodetailProfile {
        material_id: String,
        microdetail_profile: String,
    },
    MaterialBindWeatherModulation {
        material_id: String,
        weather_modulation: String,
    },
    MaterialPreviewBurn {
        material_id: String,
        preview_target: String,
    },
    MaterialSetCheapRuntimeRung {
        material_id: String,
        rung_level: u32,
    },
    MaterialInspectBranchCoverage {
        material_id: String,
    },
    // Audio authoring
    AudioCreateSource {
        source_name: String,
    },
    AudioBindWorldSource {
        source_id: String,
        position: [f32; 3],
    },
    AudioSetAcousticProfile {
        source_id: String,
        profile: String,
    },
    AudioAssignEmitterClassWorldSource {
        source_id: String,
        emitter_class: String,
    },
    AudioBindZoneProfileWorldSurface {
        zone_id: String,
        reverb_profile: String,
    },
    AudioBindPriorityDuckingPolicy {
        policy_id: String,
    },
    AudioPreviewAudibilityFreeCamera {
        listener_profile: String,
    },
    AudioPreviewObstructionVsOcclusion {
        path_id: String,
    },
    AudioPreviewIndoorOutdoorTransition {
        transition_path: String,
    },
    AudioPreviewVoiceSubtitleLegality {
        dialogue_id: String,
    },
    // Build and validation
    BuildRun,
    BuildRelease,
    ValidationRunFull,
    ValidationRunSmoke,
    // Automation
    AutomationRebuildAll,
    AutomationValidateAll,
    // Scene (legacy vertical slice)
    SceneBootstrap,
    SceneFireTestShot {
        weapon_entity_id: u32,
    },
    SceneReset,
}

impl PromotedCommand {
    pub fn route_id(&self) -> &'static str {
        match self {
            // Project
            Self::ProjectBootstrap { .. } => "route.project.bootstrap.v1",
            Self::ProjectCreate { .. } => "route.project.create.v1",
            Self::ProjectSave { .. } => "route.project.save.v1",
            Self::ProjectBuild { .. } => "route.build.package.v1",
            Self::ProjectExport { .. } => "route.export.target.v1",
            Self::ProjectLaunch { .. } => "route.launch.verify_first_result.v1",
            Self::ProjectVerifyFirstResult => "route.launch.verify_first_result.v1",
            // World
            Self::WorldOpen { .. } => "route.world.open.v1",
            Self::WorldSave { .. } => "route.world.save.v1",
            Self::WorldClose => "route.world.close.v1",
            // Runtime
            Self::RuntimePlay => "route.runtime.play.v1",
            Self::RuntimePause => "route.runtime.pause.v1",
            Self::RuntimeStop => "route.runtime.stop.v1",
            Self::RuntimeSimulate => "route.runtime.simulate.v1",
            // Terrain
            Self::TerrainImport { .. } => "route.terrain.import.v1",
            Self::TerrainRebuild => "route.terrain.rebuild.v1",
            Self::TerrainSculptRaise { .. } => "route.terrain.sculpt.raise.v1",
            Self::TerrainSculptLower { .. } => "route.terrain.sculpt.lower.v1",
            Self::TerrainSculptSmooth { .. } => "route.terrain.smooth_patch.v1",
            Self::TerrainSculptFlatten { .. } => "route.terrain.sculpt.flatten.v1",
            Self::TerrainPaintMaterial { .. } => "route.terrain.paint.material.v1",
            Self::TerrainSetLayerMaterial { .. } => "route.terrain.layer.material.set.v1",
            Self::TerrainAddHole { .. } => "route.terrain.hole.add.v1",
            Self::TerrainRemoveHole { .. } => "route.terrain.hole.remove.v1",
            // Environment
            Self::EnvironmentSetTime { .. } => "route.sky.set_time_of_day.v1",
            Self::EnvironmentSetWeather { .. } => "route.sky.set_weather_regime.v1",
            Self::EnvironmentSetCloudCoverage { .. } => "route.sky.bind_cloud_profile.v1",
            Self::EnvironmentSetFogDensity { .. } => "route.environment.fog.density.v1",
            // Material authority
            Self::MaterialAuthorityInitialize => "route.material.authority.initialize.v1",
            Self::MaterialAuthorityDispose => "route.material.authority.dispose.v1",
            // Audio authority
            Self::AudioAuthorityInitialize => "route.audio.authority.initialize.v1",
            Self::AudioAuthorityDispose => "route.audio.authority.dispose.v1",
            // Material
            Self::MaterialCreate { .. } => "route.material.create.v1",
            Self::MaterialDelete { .. } => "route.material.delete.v1",
            Self::MaterialDuplicateProfile { .. } => "route.material.duplicate_profile.v1",
            Self::MaterialBindVisualResponse { .. } => "route.material.bind_visual_response.v1",
            Self::MaterialBindAcousticProfile { .. } => "route.material.bind_acoustic_profile.v1",
            Self::MaterialBindLightResponse { .. } => "route.material.bind_light_response.v1",
            Self::MaterialBindMicrodetailProfile { .. } => {
                "route.material.bind_microdetail_profile.v1"
            }
            Self::MaterialBindWeatherModulation { .. } => {
                "route.material.bind_weather_modulation.v1"
            }
            Self::MaterialPreviewBurn { .. } => "route.material.preview_burn.v1",
            Self::MaterialSetCheapRuntimeRung { .. } => "route.material.set_cheap_runtime_rung.v1",
            Self::MaterialInspectBranchCoverage { .. } => "route.material.inspect_coverage.v1",
            // Sky
            Self::SkyBindProfile { .. } => "route.sky.bind_profile.v1",
            Self::SkySetTimeOfDay { .. } => "route.sky.set_time_of_day.v1",
            Self::SkySetWeatherRegime { .. } => "route.sky.set_weather_regime.v1",
            Self::SkyBindCloudProfile { .. } => "route.sky.bind_cloud_profile.v1",
            // Shell
            Self::ShellActivateViewport => "route.shell.activate_viewport.v1",
            Self::ShellActivateOutliner => "route.shell.activate_outliner.v1",
            Self::ShellActivateInspector => "route.shell.activate_inspector.v1",
            Self::ShellActivateContentBrowser => "route.shell.activate_content_browser.v1",
            Self::ShellActivateMaterialLab => "route.shell.activate_material_surface.v1",
            Self::ShellActivateTerrainLab => "route.shell.activate_terrain_surface.v1",
            Self::ShellActivateSkyLab => "route.shell.activate_sky_surface.v1",
            // Audio
            Self::AudioCreateSource { .. } => "route.audio.source.create.v1",
            Self::AudioBindWorldSource { .. } => "route.audio.source.bind.v1",
            Self::AudioSetAcousticProfile { .. } => "route.audio.acoustic.set.v1",
            Self::AudioAssignEmitterClassWorldSource { .. } => {
                "route.audio.assign_emitter_class_world_source.v1"
            }
            Self::AudioBindZoneProfileWorldSurface { .. } => {
                "route.audio.bind_zone_profile_world_surface.v1"
            }
            Self::AudioBindPriorityDuckingPolicy { .. } => {
                "route.audio.bind_priority_ducking_policy.v1"
            }
            Self::AudioPreviewAudibilityFreeCamera { .. } => {
                "route.audio.preview_audibility_free_camera.v1"
            }
            Self::AudioPreviewObstructionVsOcclusion { .. } => {
                "route.audio.preview_obstruction_vs_occlusion.v1"
            }
            Self::AudioPreviewIndoorOutdoorTransition { .. } => {
                "route.audio.preview_indoor_outdoor_transition.v1"
            }
            Self::AudioPreviewVoiceSubtitleLegality { .. } => {
                "route.audio.preview_voice_subtitle_legality.v1"
            }
            // Build
            Self::BuildRun => "route.build.run.v1",
            Self::BuildRelease => "route.build.release.v1",
            Self::ValidationRunFull => "route.world.validate.v1",
            Self::ValidationRunSmoke => "route.validation.smoke.v1",
            // Automation
            Self::AutomationRebuildAll => "route.automation.rebuild_all.v1",
            Self::AutomationValidateAll => "route.automation.validate_all.v1",
            // Scene
            Self::SceneBootstrap => "route.scene.bootstrap.v1",
            Self::SceneFireTestShot { .. } => "route.scene.fire_test_shot.v1",
            Self::SceneReset => "route.scene.reset.v1",
        }
    }
}
