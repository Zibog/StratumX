use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StormFrontDto {
    pub front_id: u32,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub radius_km: f32,
    pub intensity: f32,
    pub rain_intensity_mm_per_hour: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssetStatusDto {
    Found,
    Missing,
    NotRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkyBundleStatusDto {
    pub manifest_loaded: bool,
    pub bundle_id: Option<String>,
    pub stars_status: AssetStatusDto,
    pub moon_albedo_status: AssetStatusDto,
    pub moon_normal_status: AssetStatusDto,
    pub sun_disk_status: AssetStatusDto,
    pub blue_noise_status: AssetStatusDto,
    pub noise_source_status: AssetStatusDto,
    pub noise_source_count: usize,
}
