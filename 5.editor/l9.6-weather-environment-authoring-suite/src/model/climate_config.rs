use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TheaterId(pub u64);

impl TheaterId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Configuration for a climate theater
/// Extends beyond bundle path and storm front ids per Requirement 5.2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClimateConfig {
    /// Name of the climate theater
    pub name: String,

    /// Sky bundle path
    pub sky_bundle_path: String,

    /// Base temperature in Celsius
    pub base_temperature_celsius: f32,

    /// Humidity level (0.0 to 1.0)
    pub humidity: f32,

    /// Wind base speed in m/s
    pub wind_base_speed: f32,

    /// Precipitation probability (0.0 to 1.0)
    pub precipitation_probability: f32,

    /// Cloud coverage (0.0 to 1.0)
    pub cloud_coverage: f32,

    /// Fog density (0.0 to 1.0)
    pub fog_density: f32,

    /// Time of day in hours (0.0 to 24.0)
    pub time_of_day_hours: f32,

    /// Day of year (1 to 365)
    pub day_of_year: u16,

    /// Latitude in degrees (-90.0 to 90.0)
    pub latitude_deg: f32,

    /// Whether the theater is enabled
    pub enabled: bool,
}

impl ClimateConfig {
    pub fn new(name: String, sky_bundle_path: String) -> Self {
        Self {
            name,
            sky_bundle_path,
            base_temperature_celsius: 20.0,
            humidity: 0.5,
            wind_base_speed: 5.0,
            precipitation_probability: 0.3,
            cloud_coverage: 0.5,
            fog_density: 0.0,
            time_of_day_hours: 12.0,
            day_of_year: 180,
            latitude_deg: 45.0,
            enabled: true,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Climate theater name cannot be empty".to_string());
        }

        if self.sky_bundle_path.is_empty() {
            return Err("Sky bundle path cannot be empty".to_string());
        }

        if !(-100.0..=100.0).contains(&self.base_temperature_celsius) {
            return Err("Base temperature must be between -100 and 100 Celsius".to_string());
        }

        if !(0.0..=1.0).contains(&self.humidity) {
            return Err("Humidity must be between 0.0 and 1.0".to_string());
        }

        if self.wind_base_speed < 0.0 || self.wind_base_speed > 100.0 {
            return Err("Wind base speed must be between 0.0 and 100.0 m/s".to_string());
        }

        if !(0.0..=1.0).contains(&self.precipitation_probability) {
            return Err("Precipitation probability must be between 0.0 and 1.0".to_string());
        }

        if !(0.0..=1.0).contains(&self.cloud_coverage) {
            return Err("Cloud coverage must be between 0.0 and 1.0".to_string());
        }

        if !(0.0..=1.0).contains(&self.fog_density) {
            return Err("Fog density must be between 0.0 and 1.0".to_string());
        }

        if !(0.0..=24.0).contains(&self.time_of_day_hours) {
            return Err("Time of day must be between 0.0 and 24.0 hours".to_string());
        }

        if !(1..=365).contains(&self.day_of_year) {
            return Err("Day of year must be between 1 and 365".to_string());
        }

        if !(-90.0..=90.0).contains(&self.latitude_deg) {
            return Err("Latitude must be between -90.0 and 90.0 degrees".to_string());
        }

        Ok(())
    }
}
