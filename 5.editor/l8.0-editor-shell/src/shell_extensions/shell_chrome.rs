//! Status bar and shell chrome surfaces.

use eframe::egui;
use stratumx_editor_l8_0_editor_shell::status_bar::{MessageType, RuntimeState};

use super::EditorApp;

impl EditorApp {
    pub fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let state_color = match self.state.shell.status_bar.runtime_state {
                    RuntimeState::Editing => egui::Color32::GREEN,
                    RuntimeState::Playing => egui::Color32::YELLOW,
                    RuntimeState::Simulating => egui::Color32::LIGHT_BLUE,
                    RuntimeState::Paused => egui::Color32::GRAY,
                };

                ui.colored_label(
                    state_color,
                    self.state.shell.status_bar.runtime_state_display(),
                );

                if let Some(world_name) = &self.state.shell.status_bar.world_name {
                    ui.separator();
                    ui.label(format!("World: {}", world_name));
                }

                ui.separator();
                ui.label(format!("{:.0} FPS", self.state.shell.status_bar.fps));
                ui.label(format!(
                    "{:.1}ms",
                    self.state.shell.status_bar.frame_time_ms
                ));
                if let Some(quality) = &self.state.shell.status_bar.quality_summary {
                    ui.separator();
                    if let Some(verify) = &quality.verify_status {
                        ui.label(format!("Verify: {verify}"));
                    }
                    if let Some(full) = &quality.full_status {
                        ui.label(format!("Full: {full}"));
                    }
                    ui.label(format!("Tests: {}", quality.declared_tests));
                    ui.label(format!("Routes: {}", quality.route_coverage));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(message) = &self.state.shell.status_bar.message {
                        let color = match self.state.shell.status_bar.message_type {
                            MessageType::Info => egui::Color32::WHITE,
                            MessageType::Success => egui::Color32::GREEN,
                            MessageType::Warning => egui::Color32::YELLOW,
                            MessageType::Error => egui::Color32::RED,
                        };
                        ui.colored_label(color, message);
                    }
                });
            });
        });
    }

    pub fn render_panels(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for panel_id in self.state.shell.get_open_panels() {
            match panel_id.as_str() {
                "viewport" => self.render_viewport(ctx, frame),
                "outliner" => self.render_outliner(ctx),
                "inspector" => self.render_inspector(ctx),
                "diagnostics" => self.render_diagnostics(ctx),
                "terrain" => self.render_terrain_panel(ctx),
                "environment" => self.render_sky_panel(ctx),
                _ => {}
            }
        }

        if self.state.shell.project_dialogs.new_project.is_open {
            self.render_project_wizard(ctx);
        }
    }
}
