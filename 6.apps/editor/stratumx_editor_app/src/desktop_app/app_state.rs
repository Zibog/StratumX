//! Desktop app state for the launch-critical thin shell.

use std::path::Path;

use editor_dto_law::WeatherRegime;
use eframe::egui;
use stratumx_editor_l8_0_editor_shell::ShellRuntime;
use stratumx_editor_l8_10_diagnostics_surface::desktop::diagnostics_state::{
    DiagnosticsState, DiagnosticsTab,
};
use stratumx_editor_l8_1_viewport_system::viewport_camera_controller::ViewportCameraController;

/// Selection context for inspector panel — stub type.
#[derive(Clone, Debug, Default)]
pub struct SelectionContext {
    pub selected_entity: Option<String>,
}

/// Sky editor state.
#[derive(Clone, Debug)]
pub struct SkyEditorState {
    pub time_of_day: f32,
    pub weather_regime: WeatherRegime,
    pub cloud_coverage: f32,
}

impl Default for SkyEditorState {
    fn default() -> Self {
        Self {
            time_of_day: 14.0,
            weather_regime: WeatherRegime::Scattered,
            cloud_coverage: 0.35,
        }
    }
}

/// Viewport preview state.
#[derive(Default)]
pub struct ViewportPreviewState {
    pub view_mode: ViewMode,
    pub show_shadow_map: bool,
    pub show_light_cache: bool,
}

/// View mode for viewport preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Final,
    Albedo,
    Normal,
}

/// Terrain summary for world display.
#[derive(Clone, Debug, Default)]
pub struct TerrainSummary {
    pub resolution: [u32; 2],
    pub world_size: [f32; 2],
    pub chunk_grid: [u32; 2],
    pub layer_count: usize,
}

/// Environment summary for world display.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentSummary {
    pub time_of_day_hours: f32,
    pub cloud_coverage: f32,
}

/// Cached world summary — only recomputed when the world revision changes.
/// **PHASE 8 REMEDIATED**: Prevents per-frame recomputation of terrain/env stats.
#[derive(Clone, Debug, Default)]
pub struct WorldSummary {
    pub label: String,
    pub terrain: TerrainSummary,
    pub environment: EnvironmentSummary,
}

/// Cached world summary — only recomputed when the world revision changes.
/// **PHASE 8 REMEDIATED**: Prevents per-frame recomputation of terrain/env stats.
#[derive(Clone, Debug, Default)]
pub struct CachedWorldSummary {
    pub mesh_revision: u64,
    pub summary: Option<WorldSummary>,
}

pub struct AppState {
    pub shell: ShellRuntime,
    pub last_frame_time: std::time::Instant,
    pub last_quality_status_refresh: std::time::Instant,
    pub camera_controller: ViewportCameraController,
    pub gpu_texture_id: Option<egui::TextureId>,
    pub diagnostics_state: DiagnosticsState,
    pub diagnostics_tab: DiagnosticsTab,
    pub inspector_context: SelectionContext,
    pub sky_editor_state: SkyEditorState,
    pub viewport_preview_state: ViewportPreviewState,
    pub show_diagnostics: bool,
    pub terrain_heightmap_input: String,
    /// **PHASE F**: Terrain layer material texture paths
    pub terrain_layer0_albedo_input: String,
    pub terrain_layer1_albedo_input: String,
    pub ui_status_message: Option<String>,
    /// **PHASE 8 REMEDIATED**: Cached world summary to avoid per-frame recomputation.
    pub cached_world_summary: CachedWorldSummary,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            shell: ShellRuntime::new(),
            last_frame_time: std::time::Instant::now(),
            last_quality_status_refresh: std::time::Instant::now(),
            camera_controller: ViewportCameraController::new(),
            gpu_texture_id: None,
            diagnostics_state: DiagnosticsState::default(),
            diagnostics_tab: DiagnosticsTab::default(),
            inspector_context: SelectionContext::default(),
            sky_editor_state: SkyEditorState::default(),
            viewport_preview_state: ViewportPreviewState::default(),
            show_diagnostics: true,
            terrain_heightmap_input: String::new(),
            terrain_layer0_albedo_input: String::new(),
            terrain_layer1_albedo_input: String::new(),
            ui_status_message: None,
            cached_world_summary: CachedWorldSummary::default(),
        }
    }

    pub fn query(&self) -> StateQueries<'_> {
        StateQueries { state: self }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StateQueries<'a> {
    state: &'a AppState,
}

impl<'a> StateQueries<'a> {
    pub fn workspace_path(&self) -> Option<&Path> {
        self.state.shell.workspace_path.as_deref()
    }

    pub fn status_message(&self) -> Option<&str> {
        self.state.ui_status_message.as_deref()
    }
}
