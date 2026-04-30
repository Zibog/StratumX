use super::domains::PromotedCommand;
use super::ids::*;

pub(crate) fn route_id(command: &PromotedCommand) -> Option<&'static str> {
    Some(match command {
        PromotedCommand::TerrainImport { .. } => ROUTE_TERRAIN_IMPORT,
        PromotedCommand::TerrainRebuild => ROUTE_TERRAIN_REBUILD,
        PromotedCommand::TerrainSculptRaise { .. } => ROUTE_TERRAIN_SCULPT_RAISE,
        PromotedCommand::TerrainSculptLower { .. } => ROUTE_TERRAIN_SCULPT_LOWER,
        PromotedCommand::TerrainSculptSmooth { .. } => ROUTE_TERRAIN_SMOOTH_PATCH,
        PromotedCommand::TerrainSculptFlatten { .. } => ROUTE_TERRAIN_SCULPT_FLATTEN,
        PromotedCommand::TerrainPaintMaterial { .. } => ROUTE_TERRAIN_PAINT_MATERIAL,
        PromotedCommand::TerrainSetLayerMaterial { .. } => ROUTE_TERRAIN_LAYER_MATERIAL_SET,
        PromotedCommand::TerrainAddHole { .. } => ROUTE_TERRAIN_HOLE_ADD,
        PromotedCommand::TerrainRemoveHole { .. } => ROUTE_TERRAIN_HOLE_REMOVE,
        PromotedCommand::EnvironmentSetTime { .. } => ROUTE_SKY_SET_TIME_OF_DAY,
        PromotedCommand::EnvironmentSetWeather { .. } => ROUTE_SKY_SET_WEATHER_REGIME,
        PromotedCommand::EnvironmentSetCloudCoverage { .. } => ROUTE_SKY_BIND_CLOUD_PROFILE,
        PromotedCommand::EnvironmentSetFogDensity { .. } => ROUTE_ENVIRONMENT_FOG_DENSITY,
        PromotedCommand::SkyBindProfile { .. } => ROUTE_SKY_BIND_PROFILE,
        PromotedCommand::SkySetTimeOfDay { .. } => ROUTE_SKY_SET_TIME_OF_DAY,
        PromotedCommand::SkySetWeatherRegime { .. } => ROUTE_SKY_SET_WEATHER_REGIME,
        PromotedCommand::SkyBindCloudProfile { .. } => ROUTE_SKY_BIND_CLOUD_PROFILE,
        PromotedCommand::MaterialCreate { .. } => ROUTE_MATERIAL_CREATE,
        PromotedCommand::MaterialDelete { .. } => ROUTE_MATERIAL_DELETE,
        PromotedCommand::MaterialDuplicateProfile { .. } => ROUTE_MATERIAL_DUPLICATE_PROFILE,
        PromotedCommand::MaterialBindVisualResponse { .. } => ROUTE_MATERIAL_BIND_VISUAL_RESPONSE,
        PromotedCommand::MaterialBindAcousticProfile { .. } => ROUTE_MATERIAL_BIND_ACOUSTIC_PROFILE,
        PromotedCommand::MaterialBindLightResponse { .. } => ROUTE_MATERIAL_BIND_LIGHT_RESPONSE,
        PromotedCommand::MaterialBindMicrodetailProfile { .. } => {
            ROUTE_MATERIAL_BIND_MICRODETAIL_PROFILE
        }
        PromotedCommand::MaterialBindWeatherModulation { .. } => {
            ROUTE_MATERIAL_BIND_WEATHER_MODULATION
        }
        PromotedCommand::MaterialPreviewBurn { .. } => ROUTE_MATERIAL_PREVIEW_BURN,
        PromotedCommand::MaterialSetCheapRuntimeRung { .. } => {
            ROUTE_MATERIAL_SET_CHEAP_RUNTIME_RUNG
        }
        PromotedCommand::MaterialInspectBranchCoverage { .. } => ROUTE_MATERIAL_INSPECT_COVERAGE,
        PromotedCommand::AudioCreateSource { .. } => ROUTE_AUDIO_SOURCE_CREATE,
        PromotedCommand::AudioBindWorldSource { .. } => ROUTE_AUDIO_SOURCE_BIND,
        PromotedCommand::AudioSetAcousticProfile { .. } => ROUTE_AUDIO_ACOUSTIC_SET,
        PromotedCommand::AudioAssignEmitterClassWorldSource { .. } => {
            ROUTE_AUDIO_ASSIGN_EMITTER_CLASS_WORLD_SOURCE
        }
        PromotedCommand::AudioBindZoneProfileWorldSurface { .. } => {
            ROUTE_AUDIO_BIND_ZONE_PROFILE_WORLD_SURFACE
        }
        PromotedCommand::AudioBindPriorityDuckingPolicy { .. } => {
            ROUTE_AUDIO_BIND_PRIORITY_DUCKING_POLICY
        }
        PromotedCommand::AudioPreviewAudibilityFreeCamera { .. } => {
            ROUTE_AUDIO_PREVIEW_AUDIBILITY_FREE_CAMERA
        }
        PromotedCommand::AudioPreviewObstructionVsOcclusion { .. } => {
            ROUTE_AUDIO_PREVIEW_OBSTRUCTION_VS_OCCLUSION
        }
        PromotedCommand::AudioPreviewIndoorOutdoorTransition { .. } => {
            ROUTE_AUDIO_PREVIEW_INDOOR_OUTDOOR_TRANSITION
        }
        PromotedCommand::AudioPreviewVoiceSubtitleLegality { .. } => {
            ROUTE_AUDIO_PREVIEW_VOICE_SUBTITLE_LEGALITY
        }
        _ => return None,
    })
}
