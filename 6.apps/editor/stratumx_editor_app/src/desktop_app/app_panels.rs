use eframe::egui;

use super::EditorApp;

impl EditorApp {
    pub fn render_main_menu(&mut self, ctx: &egui::Context) {
        use stratumx_editor_l8_0_editor_shell::MENU_SECTIONS;

        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                for section in MENU_SECTIONS {
                    ui.menu_button(section.label, |ui| {
                        for entry in section.entries {
                            let mut label = entry.label.to_string();
                            if let Some(shortcut) = entry.shortcut {
                                label.push_str(&format!(" ({shortcut})"));
                            }
                            if ui.button(label).clicked() {
                                self.handle_world_action(entry.id);
                                ui.close_menu();
                            }
                        }
                    });
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Command Palette").clicked() {
                        self.state.shell.open_command_palette();
                    }
                });
            });
        });
    }

    pub fn render_stage_strip(&mut self, ctx: &egui::Context) {
        use stratumx_editor_l8_0_editor_shell::{WorkspaceStage, STAGE_DEFINITIONS};

        egui::TopBottomPanel::top("stage_strip").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for definition in STAGE_DEFINITIONS {
                    let active_stage = self.state.shell.stage_strip.active_stage();
                    let selected = match (active_stage, definition.stage) {
                        (WorkspaceStage::World, WorkspaceStage::World)
                        | (WorkspaceStage::Terrain, WorkspaceStage::Terrain)
                        | (WorkspaceStage::Environment, WorkspaceStage::Environment)
                        | (WorkspaceStage::Simulation, WorkspaceStage::Simulation)
                        | (WorkspaceStage::Capture, WorkspaceStage::Capture) => true,
                        _ => false,
                    };
                    if ui.selectable_label(selected, definition.label).clicked() {
                        self.state.shell.activate_stage(definition.stage);
                    }
                }
            });
        });
    }

    pub fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let state_label = match self.state.shell.status_bar.runtime_state {
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Editing => {
                        "Editing"
                    }
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Playing => {
                        "Playing"
                    }
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Simulating => {
                        "Simulating"
                    }
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Paused => "Paused",
                };
                ui.label(state_label);
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(message) = &self.state.shell.status_bar.message {
                        ui.label(message);
                    }
                });
            });
        });
    }

    pub fn render_command_palette(&mut self, ctx: &egui::Context) {
        use stratumx_editor_l8_0_editor_shell::filter_command_palette;

        if !self.state.shell.command_palette.is_open {
            return;
        }

        let filtered_commands = filter_command_palette(&self.state.shell.command_palette.query);
        if filtered_commands.is_empty() {
            self.state.shell.command_palette.selected_index = 0;
        } else if self.state.shell.command_palette.selected_index >= filtered_commands.len() {
            self.state.shell.command_palette.selected_index = filtered_commands.len() - 1;
        }

        egui::Area::new("command_palette".into())
            .anchor(egui::Align2::CENTER_TOP, [0.0, 48.0])
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    let mut query = self.state.shell.command_palette.query.clone();
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut query)
                                .desired_width(420.0)
                                .hint_text("Type a command"),
                        )
                        .changed()
                    {
                        self.state.shell.command_palette_input(&query);
                    }
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .max_height(240.0)
                        .show(ui, |ui| {
                            for (index, command) in filtered_commands.iter().enumerate() {
                                let selected =
                                    index == self.state.shell.command_palette.selected_index;
                                let response = ui.selectable_label(selected, command.label);
                                if response.clicked() {
                                    self.handle_world_action(command.id);
                                    self.state.shell.close_command_palette();
                                }
                            }
                            if filtered_commands.is_empty() {
                                ui.label("No matching commands");
                            }
                        });
                });
            });

        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.state.shell.close_command_palette();
        }
    }

    pub fn render_panels(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for panel_id in self.state.shell.get_open_panels() {
            match panel_id.as_str() {
                "viewport" => self.render_viewport(ctx, frame),
                "outliner" => self.render_outliner(ctx),
                "inspector" => self.render_inspector_panel(ctx),
                "diagnostics" => self.render_diagnostics_panel(ctx),
                "terrain" => self.render_terrain_panel(ctx),
                "environment" => self.render_sky_panel(ctx),
                _ => {}
            }
        }
    }

    pub fn render_open_world_dialog(&mut self, _ctx: &egui::Context) {
        // No-op unless project dialogs are open.
    }
}
