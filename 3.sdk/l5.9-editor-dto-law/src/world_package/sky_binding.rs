use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkyBinding {
    pub sky_bundle_ref: String,
    pub time_of_day: f32,
    pub day_of_year: u16,
    pub latitude_deg: f32,
    pub weather_regime: String,
    pub cloud_coverage: f32,
    pub fog_density: f32,
}
