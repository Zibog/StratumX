//! Status Bar — bottom status bar with runtime info, messages, and quick actions.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBar {
    pub message: Option<String>,
    pub message_type: MessageType,
    pub runtime_state: RuntimeState,
    pub fps: f32,
    pub frame_time_ms: f32,
    pub selected_count: usize,
    pub world_name: Option<String>,
    pub cursor_position: Option<(f32, f32, f32)>,
    pub quality_summary: Option<GeneratedQualitySummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeState {
    Editing,
    Playing,
    Simulating,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeneratedQualitySummary {
    pub verify_status: Option<String>,
    pub smoke_status: Option<String>,
    pub full_status: Option<String>,
    pub declared_tests: u64,
    pub route_coverage: u64,
    pub workspace_packages: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct CommandSummaryArtifact {
    status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MetricsSummaryArtifact {
    declared_tests: u64,
    route_coverage: u64,
    workspace_packages: u64,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            message: None,
            message_type: MessageType::Info,
            runtime_state: RuntimeState::Editing,
            fps: 0.0,
            frame_time_ms: 0.0,
            selected_count: 0,
            world_name: None,
            cursor_position: None,
            quality_summary: None,
        }
    }

    /// Sets a status message
    pub fn set_message(&mut self, message: impl Into<String>, message_type: MessageType) {
        self.message = Some(message.into());
        self.message_type = message_type;
    }

    /// Clears the status message after a delay (caller should handle)
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Updates performance stats
    pub fn update_performance(&mut self, fps: f32, frame_time_ms: f32) {
        self.fps = fps;
        self.frame_time_ms = frame_time_ms;
    }

    /// Sets the runtime state
    pub fn set_runtime_state(&mut self, state: RuntimeState) {
        self.runtime_state = state;
    }

    /// Sets selection info
    pub fn set_selection(&mut self, count: usize) {
        self.selected_count = count;
    }

    /// Sets the world name
    pub fn set_world_name(&mut self, name: Option<String>) {
        self.world_name = name;
    }

    /// Sets cursor position
    pub fn set_cursor_position(&mut self, x: f32, y: f32, z: f32) {
        self.cursor_position = Some((x, y, z));
    }

    /// Clears cursor position
    pub fn clear_cursor_position(&mut self) {
        self.cursor_position = None;
    }

    pub fn set_quality_summary(&mut self, summary: Option<GeneratedQualitySummary>) {
        self.quality_summary = summary;
    }

    pub fn load_quality_summary_from_generated(
        &mut self,
        generated_root: &Path,
    ) -> Result<(), String> {
        self.quality_summary = Some(GeneratedQualitySummary::load(generated_root)?);
        Ok(())
    }

    pub fn runtime_state_display(&self) -> &str {
        match self.runtime_state {
            RuntimeState::Editing => "Editing",
            RuntimeState::Playing => "Playing",
            RuntimeState::Simulating => "Simulating",
            RuntimeState::Paused => "Paused",
        }
    }
}

impl GeneratedQualitySummary {
    pub fn load(generated_root: &Path) -> Result<Self, String> {
        let test_results_root = generated_root.join("test-results");
        let metrics_root = generated_root.join("metrics");

        let verify_status = load_status_file(&test_results_root.join("verify-summary.json"))?;
        let smoke_status = load_status_file(&test_results_root.join("smoke-summary.json"))?;
        let full_status = load_status_file(&test_results_root.join("full-summary.json"))?;
        let metrics: MetricsSummaryArtifact =
            load_json(&metrics_root.join("metrics-summary.json"))?;

        Ok(Self {
            verify_status: Some(verify_status.status),
            smoke_status: Some(smoke_status.status),
            full_status: Some(full_status.status),
            declared_tests: metrics.declared_tests,
            route_coverage: metrics.route_coverage,
            workspace_packages: metrics.workspace_packages,
        })
    }
}

fn load_status_file(path: &Path) -> Result<CommandSummaryArtifact, String> {
    load_json(path)
}

fn load_json<T>(path: &Path) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&contents)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}
