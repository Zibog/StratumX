use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkySummary {
    pub time_of_day_hours: f32,
    pub day_of_year: u16,
    pub latitude_deg: f32,
    pub sun_elevation_deg: f32,
    pub sun_intensity: f32,
    pub cloud_coverage: f32,
    pub fog_density: f32,
    pub rain_enabled: bool,
    pub rain_intensity_mm_per_hour: f32,
    pub wind_vector: [f32; 3],
    pub storm_front_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StormFrontSummary {
    pub front_id: u32,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub radius_km: f32,
    pub intensity: f32,
    pub rain_intensity_mm_per_hour: f32,
}
