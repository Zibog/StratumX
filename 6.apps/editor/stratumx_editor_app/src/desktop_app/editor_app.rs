//! Desktop editor app container.

use crate::desktop_app::app_state::AppState;
use crate::editor_host::EditorHost;
use stratumx_editor_l9_2_terrain_landscape_authoring_suite::host::terrain_authoring_ops::TerrainAuthoringOps;

pub struct EditorApp {
    pub runtime_host: EditorHost,
    pub state: AppState,
    pub terrain_ops: TerrainAuthoringOps,
}

impl EditorApp {
    pub fn new(host: EditorHost, _cc: &eframe::CreationContext<'_>) -> Self {
        let terrain_ops = TerrainAuthoringOps::new();

        Self {
            runtime_host: host,
            state: AppState::new(),
            terrain_ops,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.run_frame(ctx, frame);
    }
}
