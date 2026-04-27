// Diagnostics DTOs
// Frozen semantic types for diagnostics and status reporting

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Posture {
    Healthy,
    Degraded,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSummaryDto {
    pub host_posture: Posture,
    pub world_posture: Posture,
    pub terrain_posture: Posture,
    pub environment_posture: Posture,
    pub viewport_posture: Posture,
    pub runtime_posture: Posture,
    pub bridge_posture: Posture,
    pub degradation_posture: Posture,
    pub weather_diagnostics: Option<WeatherDiagnostics>,
}

/// Weather-specific diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherDiagnostics {
    pub sky_bundle_status: String,
    pub asset_root_resolved: Option<String>,
    pub stars_present: bool,
    pub moon_albedo_present: bool,
    pub moon_height_present: bool,
    pub moon_normal_present: bool,
    pub sun_profile_active: bool,
    pub weather_director_active: bool,
    pub weather_cells_count: usize,
    pub cloud_shadow_posture: String,
    pub weather_regime: String,
    pub render_pass_active: bool,
    pub viewport_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticEntry {
    pub level: DiagnosticLevel,
    pub message: String,
    pub source: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportDiagnosticsProjection {
    pub host: Posture,
    pub world: Posture,
    pub terrain: Posture,
    pub environment: Posture,
    pub runtime: Posture,
    pub budgets: Posture,
}
