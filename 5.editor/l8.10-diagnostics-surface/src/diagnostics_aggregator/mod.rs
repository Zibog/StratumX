pub mod frame;
pub mod sky;
pub mod terrain;

use editor_dto_law::{
    DiagnosticEntry, DiagnosticLevel, DiagnosticsSummaryDto, Posture,
    ViewportDiagnosticsProjection, WeatherDiagnostics,
};

pub struct DiagnosticsAggregator {
    host_posture: Posture,
    world_posture: Posture,
    terrain_posture: Posture,
    environment_posture: Posture,
    viewport_posture: Posture,
    runtime_posture: Posture,
    bridge_posture: Posture,
    entries: Vec<DiagnosticEntry>,
    weather_diagnostics: Option<WeatherDiagnostics>,
}

impl DiagnosticsAggregator {
    pub fn new() -> Self {
        Self {
            host_posture: Posture::Unknown,
            world_posture: Posture::Unknown,
            terrain_posture: Posture::Unknown,
            environment_posture: Posture::Unknown,
            viewport_posture: Posture::Unknown,
            runtime_posture: Posture::Unknown,
            bridge_posture: Posture::Unknown,
            entries: Vec::new(),
            weather_diagnostics: None,
        }
    }

    fn add_entry(&mut self, level: DiagnosticLevel, message: String, source: String) {
        self.entries.push(DiagnosticEntry {
            level,
            message,
            source,
            location: None,
        });
    }

    /// Get summary DTO
    pub fn get_summary(&self) -> DiagnosticsSummaryDto {
        let degradation_posture = self.calculate_degradation_posture();

        DiagnosticsSummaryDto {
            host_posture: self.host_posture.clone(),
            world_posture: self.world_posture.clone(),
            terrain_posture: self.terrain_posture.clone(),
            environment_posture: self.environment_posture.clone(),
            viewport_posture: self.viewport_posture.clone(),
            runtime_posture: self.runtime_posture.clone(),
            bridge_posture: self.bridge_posture.clone(),
            degradation_posture,
            weather_diagnostics: self.weather_diagnostics.clone(),
        }
    }

    /// Get viewport diagnostics projection
    pub fn get_viewport_projection(&self) -> ViewportDiagnosticsProjection {
        ViewportDiagnosticsProjection {
            host: self.host_posture.clone(),
            world: self.world_posture.clone(),
            terrain: self.terrain_posture.clone(),
            environment: self.environment_posture.clone(),
            runtime: self.runtime_posture.clone(),
            budgets: Posture::Healthy,
        }
    }

    /// Get all diagnostic entries
    pub fn get_entries(&self) -> &[DiagnosticEntry] {
        &self.entries
    }

    /// Clear all entries
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    fn calculate_degradation_posture(&self) -> Posture {
        let postures = [
            &self.host_posture,
            &self.world_posture,
            &self.terrain_posture,
            &self.environment_posture,
            &self.viewport_posture,
            &self.runtime_posture,
            &self.bridge_posture,
        ];

        if postures.iter().any(|p| matches!(p, Posture::Failed)) {
            return Posture::Failed;
        }
        if postures.iter().any(|p| matches!(p, Posture::Degraded)) {
            return Posture::Degraded;
        }
        if postures.iter().all(|p| matches!(p, Posture::Healthy)) {
            return Posture::Healthy;
        }
        Posture::Unknown
    }
}

impl Default for DiagnosticsAggregator {
    fn default() -> Self {
        Self::new()
    }
}
