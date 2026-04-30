use eframe::egui;

use super::EditorApp;

impl EditorApp {
    pub(super) fn render_diagnostics_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("diagnostics")
            .min_width(250.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Diagnostics");
            });
    }
}
