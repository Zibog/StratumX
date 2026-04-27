#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]

#[cfg(feature = "desktop")]
pub mod desktop;
pub mod host;

pub mod environment_delivery;
pub mod model;
pub mod runtime;
pub mod weather_engine_bridge;
pub mod weather_scenarios;

pub use environment_delivery::*;
pub use weather_engine_bridge::*;
pub use weather_scenarios::*;

// Re-export public API
pub use model::{
    ClimateConfig, StormFront, StormFrontId, StormParams, TheaterId, WeatherError, WeatherState,
};
pub use runtime::WeatherEnvironmentAuthoringSuite;

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ObjectHandle(pub u64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkySummaryCache {
    pub time_of_day_hours: f32,
    pub day_of_year: u16,
    pub latitude_deg: f32,
    pub sun_elevation_deg: f32,
    pub cloud_coverage: f32,
    pub fog_density: f32,
    pub rain_enabled: bool,
    pub rain_intensity_mm_per_hour: f32,
    pub wind_vector: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleValidationStatus {
    pub manifest_loaded: bool,
    pub bundle_id: Option<String>,
    pub stars_found: bool,
    pub moon_found: bool,
    pub sun_found: bool,
    pub noise_found: bool,
    pub last_validation_timestamp: Option<u64>,
}
