//! Environment Command Handlers

use super::super::EditorApp;

impl EditorApp {
    pub(super) fn handle_environment_set_time(&mut self, time_of_day_hours: f32) -> Result<String, String> {
        self.runtime_host.apply_environment_time(time_of_day_hours)
    }

    pub(super) fn handle_environment_set_weather(&mut self, weather_regime: String) -> Result<String, String> {
        self.runtime_host.apply_environment_weather(&weather_regime)
    }

    pub(super) fn handle_environment_set_cloud_coverage(&mut self, coverage: f32) -> Result<String, String> {
        self.runtime_host.apply_environment_cloud_coverage(coverage)
    }

    pub(super) fn handle_environment_set_fog_density(&mut self, density: f32) -> Result<String, String> {
        self.runtime_host.apply_environment_fog_density(density)
    }
}
