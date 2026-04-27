use serde::{Deserialize, Serialize};

/// Current weather state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherState {
    /// Current temperature in Celsius
    pub temperature_celsius: f32,

    /// Current humidity (0.0 to 1.0)
    pub humidity: f32,

    /// Current wind speed in m/s
    pub wind_speed: f32,

    /// Current wind direction [x, y, z] (normalized)
    pub wind_direction: [f32; 3],

    /// Current precipitation rate in mm/hour
    pub precipitation_rate: f32,

    /// Current cloud coverage (0.0 to 1.0)
    pub cloud_coverage: f32,

    /// Current fog density (0.0 to 1.0)
    pub fog_density: f32,

    /// Current visibility in meters
    pub visibility_meters: f32,

    /// Active storm front count
    pub active_storm_count: usize,

    /// Time of day in hours (0.0 to 24.0)
    pub time_of_day_hours: f32,

    /// Sun elevation in degrees
    pub sun_elevation_deg: f32,

    /// Whether it's currently raining
    pub is_raining: bool,

    /// Whether lightning is active
    pub lightning_active: bool,
}

impl WeatherState {
    pub fn new() -> Self {
        Self {
            temperature_celsius: 20.0,
            humidity: 0.5,
            wind_speed: 5.0,
            wind_direction: [1.0, 0.0, 0.0],
            precipitation_rate: 0.0,
            cloud_coverage: 0.5,
            fog_density: 0.0,
            visibility_meters: 10000.0,
            active_storm_count: 0,
            time_of_day_hours: 12.0,
            sun_elevation_deg: 45.0,
            is_raining: false,
            lightning_active: false,
        }
    }
}

impl Default for WeatherState {
    fn default() -> Self {
        Self::new()
    }
}
