use editor_dto_law::{DiagnosticLevel, Posture};

use super::DiagnosticsAggregator;

impl DiagnosticsAggregator {
    /// Report terrain bind truth
    pub fn report_terrain_bind_truth(&mut self, posture: Posture, message: Option<String>) {
        self.terrain_posture = posture.clone();
        if let Some(msg) = message {
            let level = match posture {
                Posture::Failed => DiagnosticLevel::Error,
                Posture::Degraded => DiagnosticLevel::Warning,
                _ => DiagnosticLevel::Info,
            };
            self.add_entry(level, msg, "Terrain".to_string());
        }
    }

    /// Report viewport state
    pub fn report_viewport_state(&mut self, posture: Posture, message: Option<String>) {
        self.viewport_posture = posture.clone();
        if let Some(msg) = message {
            self.add_entry(DiagnosticLevel::Info, msg, "Viewport".to_string());
        }
    }

    /// Get terrain posture
    pub fn terrain_posture(&self) -> Posture {
        self.terrain_posture.clone()
    }

    /// Get viewport posture
    pub fn viewport_posture(&self) -> Posture {
        self.viewport_posture.clone()
    }
}
