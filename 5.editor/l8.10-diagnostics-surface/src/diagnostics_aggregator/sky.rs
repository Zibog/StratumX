use editor_dto_law::{DiagnosticLevel, Posture, WeatherDiagnostics};

use super::DiagnosticsAggregator;

/// Parameters for updating weather diagnostics from environment state.
pub struct WeatherUpdateParams {
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

impl DiagnosticsAggregator {
    /// Report sky bind truth
    pub fn report_sky_bind_truth(&mut self, posture: Posture, message: Option<String>) {
        self.environment_posture = posture.clone();
        if let Some(msg) = message {
            let level = match posture {
                Posture::Failed => DiagnosticLevel::Error,
                Posture::Degraded => DiagnosticLevel::Warning,
                _ => DiagnosticLevel::Info,
            };
            self.add_entry(level, msg, "Environment".to_string());
        }
    }

    /// Report weather diagnostics
    pub fn report_weather_diagnostics(&mut self, diagnostics: WeatherDiagnostics) {
        self.weather_diagnostics = Some(diagnostics);
    }

    /// Update weather diagnostics from environment state
    pub fn update_weather_diagnostics(&mut self, params: WeatherUpdateParams) {
        self.weather_diagnostics = Some(WeatherDiagnostics {
            sky_bundle_status: params.sky_bundle_status,
            asset_root_resolved: params.asset_root_resolved,
            stars_present: params.stars_present,
            moon_albedo_present: params.moon_albedo_present,
            moon_height_present: params.moon_height_present,
            moon_normal_present: params.moon_normal_present,
            sun_profile_active: params.sun_profile_active,
            weather_director_active: params.weather_director_active,
            weather_cells_count: params.weather_cells_count,
            cloud_shadow_posture: params.cloud_shadow_posture,
            weather_regime: params.weather_regime,
            render_pass_active: params.render_pass_active,
            viewport_mode: params.viewport_mode,
        });
    }

    /// Get environment posture
    pub fn environment_posture(&self) -> Posture {
        self.environment_posture.clone()
    }

    /// Get weather diagnostics
    pub fn weather_diagnostics(&self) -> Option<&WeatherDiagnostics> {
        self.weather_diagnostics.as_ref()
    }
}
