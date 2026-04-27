use editor_dto_law::{DiagnosticLevel, Posture};

use super::DiagnosticsAggregator;

impl DiagnosticsAggregator {
    /// Report host truth
    pub fn report_host_truth(&mut self, posture: Posture, message: Option<String>) {
        self.host_posture = posture.clone();
        if let Some(msg) = message {
            self.add_entry(DiagnosticLevel::Info, msg, "Host".to_string());
        }
    }

    /// Report world open truth
    pub fn report_world_open_truth(&mut self, posture: Posture, message: Option<String>) {
        self.world_posture = posture.clone();
        if let Some(msg) = message {
            let level = match posture {
                Posture::Failed => DiagnosticLevel::Error,
                Posture::Degraded => DiagnosticLevel::Warning,
                _ => DiagnosticLevel::Info,
            };
            self.add_entry(level, msg, "World".to_string());
        }
    }

    /// Report runtime state
    pub fn report_runtime_state(&mut self, posture: Posture, message: Option<String>) {
        self.runtime_posture = posture.clone();
        if let Some(msg) = message {
            self.add_entry(DiagnosticLevel::Info, msg, "Runtime".to_string());
        }
    }

    /// Report bridge state
    pub fn report_bridge_state(&mut self, posture: Posture, message: Option<String>) {
        self.bridge_posture = posture.clone();
        if let Some(msg) = message {
            self.add_entry(DiagnosticLevel::Info, msg, "Bridge".to_string());
        }
    }

    /// Report degraded posture
    pub fn report_degraded_posture(&mut self, reason: String) {
        self.add_entry(DiagnosticLevel::Warning, reason, "Degradation".to_string());
    }

    /// Report failure classes
    pub fn report_failure(&mut self, source: String, message: String) {
        self.add_entry(DiagnosticLevel::Error, message, source);
    }

    /// Get host posture
    pub fn host_posture(&self) -> Posture {
        self.host_posture.clone()
    }

    /// Get world posture
    pub fn world_posture(&self) -> Posture {
        self.world_posture.clone()
    }

    /// Get runtime posture
    pub fn runtime_posture(&self) -> Posture {
        self.runtime_posture.clone()
    }

    /// Get bridge posture
    pub fn bridge_posture(&self) -> Posture {
        self.bridge_posture.clone()
    }
}
