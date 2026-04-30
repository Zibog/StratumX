use serde::{Deserialize, Serialize};

use super::payloads::{PlanarPosition, UvScale, WorldPosition};

/// Unified promoted command enum that wraps all domain-specific command families.
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PromotedCommand {
    // Project lifecycle
    ProjectBootstrap { project_name: String },
    ProjectCreate { project_name: String, project_root: String, world_name: String },
    ProjectSave { save_path: String },
    ProjectBuild { target_platform: String },
    ProjectExport { export_path: String },
    ProjectLaunch { launch_mode: String },
    ProjectVerifyFirstResult,
    // World lifecycle
    WorldOpen { world_path: String },
    WorldSave { world_path: String },
    WorldClose,
    // Runtime control
    RuntimePlay,
    RuntimePause,
    RuntimeStop,
    RuntimeSimulate,
    // Terrain authoring
    TerrainImport { heightmap_path: String },
    TerrainRebuild,
    TerrainSculptRaise { position: PlanarPosition, radius: f32, strength: f32 },
    TerrainSculptLower { position: PlanarPosition, radius: f32, strength: f32 },
    TerrainSculptSmooth { position: PlanarPosition, radius: f32, strength: f32 },
    TerrainSculptFlatten { position: PlanarPosition, radius: f32, strength: f32, target_height: f32 },
    TerrainPaintMaterial { position: PlanarPosition, radius: f32, strength: f32, material_layer: u32 },
    TerrainSetLayerMaterial { layer_id: u16, albedo_texture_path: String, uv_scale: UvScale },
    TerrainAddHole { position: PlanarPosition, radius: f32 },
    TerrainRemoveHole { position: PlanarPosition, radius: f32 },
    // Environment authoring
    EnvironmentSetTime { time_of_day_hours: f32 },
    EnvironmentSetWeather { weather_regime: String },
    EnvironmentSetCloudCoverage { coverage: f32 },
    EnvironmentSetFogDensity { density: f32 },
    // Sky authoring
    SkyBindProfile { sky_profile: String },
    SkySetTimeOfDay { time_of_day_hours: f32 },
    SkySetWeatherRegime { weather_regime: String },
    SkyBindCloudProfile { cloud_profile: String },
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
    MaterialCreate { material_name: String },
    MaterialDelete { material_id: String },
    MaterialDuplicateProfile { source_material_id: String, target_name: String },
    MaterialBindVisualResponse { material_id: String, visual_family: String },
    MaterialBindAcousticProfile { material_id: String, acoustic_profile: String },
    MaterialBindLightResponse { material_id: String, light_response: String },
    MaterialBindMicrodetailProfile { material_id: String, microdetail_profile: String },
    MaterialBindWeatherModulation { material_id: String, weather_modulation: String },
    MaterialPreviewBurn { material_id: String, preview_target: String },
    MaterialSetCheapRuntimeRung { material_id: String, rung_level: u32 },
    MaterialInspectBranchCoverage { material_id: String },
    // Audio authoring
    AudioCreateSource { source_name: String },
    AudioBindWorldSource { source_id: String, position: WorldPosition },
    AudioSetAcousticProfile { source_id: String, profile: String },
    AudioAssignEmitterClassWorldSource { source_id: String, emitter_class: String },
    AudioBindZoneProfileWorldSurface { zone_id: String, reverb_profile: String },
    AudioBindPriorityDuckingPolicy { policy_id: String },
    AudioPreviewAudibilityFreeCamera { listener_profile: String },
    AudioPreviewObstructionVsOcclusion { path_id: String },
    AudioPreviewIndoorOutdoorTransition { transition_path: String },
    AudioPreviewVoiceSubtitleLegality { dialogue_id: String },
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
    SceneFireTestShot { weapon_entity_id: u32 },
    SceneReset,
}
