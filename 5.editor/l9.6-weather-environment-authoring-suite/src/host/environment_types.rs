//! Environment Types
//!
//! Type definitions for environment service.

#[derive(Debug, Clone)]
pub struct SkyConfiguration {
    pub sun_intensity: f32,
    pub sun_color: [f32; 3],
    pub ambient_color: [f32; 3],
    pub sky_color: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct WeatherConfiguration {
    pub precipitation: f32,
    pub wind_speed: f32,
    pub wind_direction: [f32; 2],
    pub cloud_coverage: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentChangeType {
    Sky,
    Weather,
    Time,
}

#[derive(Debug, Clone)]
pub struct EnvironmentChangedEvent {
    pub change_type: EnvironmentChangeType,
}
