use serde::{Deserialize, Serialize};

/// Environment authoring commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentCommand {
    SetTime { time_of_day_hours: f32 },
    SetWeather { weather_regime: String },
    SetCloudCoverage { coverage: f32 },
    SetFogDensity { density: f32 },
}

/// Sky authoring commands (canonical routes use sky.* prefix)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkyCommand {
    BindProfile { sky_profile: String },
    SetTimeOfDay { time_of_day_hours: f32 },
    SetWeatherRegime { weather_regime: String },
    BindCloudProfile { cloud_profile: String },
}

/// Shell surface activation commands
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShellCommand {
    ActivateViewport,
    ActivateOutliner,
    ActivateInspector,
    ActivateContentBrowser,
    ActivateMaterialLab,
    ActivateTerrainLab,
    ActivateSkyLab,
}
