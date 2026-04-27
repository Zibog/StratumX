//! Minimal outliner backed by host read models.

use eframe::egui;

use super::EditorApp;

impl EditorApp {
    pub fn render_outliner(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("outliner")
            .min_width(240.0)
            .show(ctx, |ui| {
                ui.heading("Outliner");
                ui.separator();

                if let Some(summary) = self.world_summary() {
                    ui.group(|ui| {
                        ui.label("World");
                        ui.label(format!("Name: {}", summary.label));
                    });

                    ui.separator();

                    ui.group(|ui| {
                        ui.label("Terrain");
                        ui.label(format!(
                            "Resolution: {}x{}",
                            summary.terrain.resolution[0], summary.terrain.resolution[1]
                        ));
                        ui.label(format!(
                            "Chunks: {}x{}",
                            summary.terrain.chunk_grid[0], summary.terrain.chunk_grid[1]
                        ));
                    });

                    ui.separator();

                    ui.group(|ui| {
                        ui.label("Environment");
                        ui.label(format!(
                            "Time: {:.1}h",
                            summary.environment.time_of_day_hours
                        ));
                        ui.label(format!("Weather: {:?}", summary.environment.weather_regime));
                    });
                } else {
                    ui.label("No world loaded");
                }
            });
    }
}
