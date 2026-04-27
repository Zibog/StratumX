use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Terrain state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainState {
    /// Terrain heightmap resolution.
    pub heightmap_resolution: (u32, u32),

    /// Terrain size in world units.
    pub terrain_size: (f32, f32),

    /// Current terrain layer.
    pub current_layer: String,

    /// Terrain modification count.
    pub modification_count: u64,
}

impl TerrainState {
    /// Creates a new terrain state.
    pub fn new(
        heightmap_resolution: (u32, u32),
        terrain_size: (f32, f32),
        current_layer: String,
    ) -> Self {
        Self {
            heightmap_resolution,
            terrain_size,
            current_layer,
            modification_count: 0,
        }
    }

    /// Increments the modification count.
    pub fn increment_modifications(&mut self) {
        self.modification_count += 1;
    }
}

/// Environment state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    /// Sky profile ID.
    pub sky_profile_id: Option<Uuid>,

    /// Time of day (0.0 - 24.0).
    pub time_of_day: f32,

    /// Weather condition.
    pub weather_condition: WeatherCondition,

    /// Ambient light color (RGB).
    pub ambient_light: [f32; 3],

    /// Sun direction (normalized vector).
    pub sun_direction: [f32; 3],
}

impl EnvironmentState {
    /// Creates a new environment state with default values.
    pub fn new() -> Self {
        Self {
            sky_profile_id: None,
            time_of_day: 12.0,
            weather_condition: WeatherCondition::Clear,
            ambient_light: [0.2, 0.2, 0.2],
            sun_direction: [0.0, 1.0, 0.0],
        }
    }

    /// Sets the time of day.
    pub fn set_time_of_day(&mut self, time: f32) {
        self.time_of_day = time.clamp(0.0, 24.0);
    }

    /// Sets the weather condition.
    pub fn set_weather(&mut self, weather: WeatherCondition) {
        self.weather_condition = weather;
    }
}

impl Default for EnvironmentState {
    fn default() -> Self {
        Self::new()
    }
}

/// Weather condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeatherCondition {
    /// Clear sky.
    Clear,

    /// Partly cloudy.
    PartlyCloudy,

    /// Overcast.
    Overcast,

    /// Rain.
    Rain,

    /// Storm.
    Storm,

    /// Snow.
    Snow,

    /// Fog.
    Fog,
}
