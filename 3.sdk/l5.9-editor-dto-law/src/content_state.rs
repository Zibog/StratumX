// Content State DTOs
// Frozen semantic types for terrain and environment state

use crate::identity::{ProfileRef, SkyEnvironmentBindingRef, StableWorldId, TerrainBindingRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LodPosture {
    Full,
    Reduced,
    Minimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainStateDto {
    pub world_ref: StableWorldId,
    pub terrain_binding_ref: Option<TerrainBindingRef>,
    pub present: bool,
    pub walkable: bool,
    pub material_profile_ref: Option<ProfileRef>,
    pub lod_posture: LodPosture,
    pub degraded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudPosture {
    Clear,
    Scattered,
    Overcast,
    Storm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FogPosture {
    None,
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrecipitationPosture {
    None,
    LightRain,
    HeavyRain,
    Snow,
    Hail,
}

/// Weather regime - artistic control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherRegime {
    Clear,
    Scattered,
    Overcast,
    IncomingStorm,
    HeavyStorm,
    PostStormCalm,
    FogMorning,
    WindyOvercast,
}

/// Cloud shadow posture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudShadowPosture {
    Full,
    Simplified,
    Fallback,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyStateDto {
    pub world_ref: StableWorldId,
    pub environment_binding_ref: Option<SkyEnvironmentBindingRef>,
    pub present: bool,
    pub time_of_day: f32, // 0.0 - 24.0
    pub date_or_cycle_ref: Option<String>,
    pub cloud_posture: CloudPosture,
    pub fog_posture: FogPosture,
    pub precipitation_posture: PrecipitationPosture,
    pub degraded: bool,
}
