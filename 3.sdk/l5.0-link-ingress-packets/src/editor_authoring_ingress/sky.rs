use editor_dto_law::WeatherRegime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkyCommand {
    GetSkySummary,
    GetSkyBundleStatus,
    GetSkyDiagnostics,
    CaptureSkyBaseline,
    ResetSkyToBaseline,
    RecoverDefaultSky,
    SetTimeOfDay {
        hours: f32,
    },
    SetDayOfYear {
        day: u16,
    },
    SetLatitude {
        latitude_deg: f32,
    },
    SetCloudCoverage {
        value: f32,
    },
    SetFogDensity {
        value: f32,
    },
    SetRain {
        enabled: bool,
        intensity_mm_per_hour: f32,
    },
    SetWindVector {
        value: [f32; 3],
    },
    SetWeatherRegime {
        regime: WeatherRegime,
    },
    StepSkySimulation {
        dt_seconds: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StormCommand {
    CreateStormFront {
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        intensity: f32,
        rain_intensity_mm_per_hour: f32,
    },
    UpdateStormFront {
        front_id: u32,
        position: Option<[f32; 3]>,
        velocity: Option<[f32; 3]>,
        radius_km: Option<f32>,
        intensity: Option<f32>,
        rain_intensity_mm_per_hour: Option<f32>,
    },
    ListStormFronts,
}
