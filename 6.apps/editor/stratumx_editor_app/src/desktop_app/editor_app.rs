//! Desktop editor app container.

use crate::desktop_app::app_state::AppState;
use crate::editor_host::EditorHost;
use stratumx_editor_l7_0_editor_command_spine::app_actions::ActionPayload;
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

    pub fn dispatch_action(
        &mut self,
        action_id: &str,
        payload: Option<ActionPayload>,
    ) {
        self.dispatch_phase4_action(action_id, payload);
    }

    /// Dispatch action through the Phase 4 command spine.
    pub fn dispatch_phase4_action(
        &mut self,
        action_id: &str,
        payload: Option<ActionPayload>,
    ) {
        use stratumx_editor_l7_0_editor_command_spine::app_actions::ActionDispatcher;
        if let Some(command) = ActionDispatcher::dispatch(action_id, payload) {
            let _ = self.state.shell.submit_command(command);
        }
    }

    /// Sync UI state from world data.
    pub fn sync_ui_from_world(&mut self) {
        if let Some(summary) = self.cached_world_summary() {
            self.state
                .shell
                .status_bar
                .set_world_name(Some(summary.label.clone()));
            self.state.sky_editor_state.time_of_day = summary.environment.time_of_day_hours;
            self.state.sky_editor_state.cloud_coverage = summary.environment.cloud_coverage;
        } else {
            self.state.shell.status_bar.set_world_name(None);
        }
    }

    /// Get cached world summary.
    pub fn cached_world_summary(&mut self) -> Option<crate::desktop_app::app_state::WorldSummary> {
        // Stub: return cached summary or None
        self.state.cached_world_summary.summary.clone()
    }

    /// Invalidate the cached world summary.
    pub fn invalidate_cached_world_summary(&mut self) {
        self.state.cached_world_summary = Default::default();
    }

    /// Handle world action (open, save, close, panel toggle, etc.)
    pub fn handle_world_action(&mut self, action_id: &str) {
        self.dispatch_phase4_action(action_id, None);
    }

    // -----------------------------------------------------------------------
    // Render methods
    // -----------------------------------------------------------------------

    pub fn render_main_menu(&mut self, ctx: &eframe::egui::Context) {
        use stratumx_editor_l8_0_editor_shell::MENU_SECTIONS;
        eframe::egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            eframe::egui::menu::bar(ui, |ui| {
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
                ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                    if ui.button("Command Palette").clicked() {
                        self.state.shell.open_command_palette();
                    }
                });
            });
        });
    }

    pub fn render_stage_strip(&mut self, ctx: &eframe::egui::Context) {
        use stratumx_editor_l8_0_editor_shell::{WorkspaceStage, STAGE_DEFINITIONS};
        eframe::egui::TopBottomPanel::top("stage_strip").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for definition in STAGE_DEFINITIONS {
                    let active_stage = self.state.shell.stage_strip.active_stage();
                    let selected = match (active_stage, definition.stage) {
                        (WorkspaceStage::World, WorkspaceStage::World) => true,
                        (WorkspaceStage::Terrain, WorkspaceStage::Terrain) => true,
                        (WorkspaceStage::Environment, WorkspaceStage::Environment) => true,
                        (WorkspaceStage::Simulation, WorkspaceStage::Simulation) => true,
                        (WorkspaceStage::Capture, WorkspaceStage::Capture) => true,
                        _ => false,
                    };
                    if ui.selectable_label(selected, definition.label).clicked() {
                        self.state.shell.activate_stage(definition.stage);
                    }
                }
            });
        });
    }

    pub fn render_status_bar(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let state_label = match self.state.shell.status_bar.runtime_state {
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Editing => "Editing",
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Playing => "Playing",
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Simulating => "Simulating",
                    stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState::Paused => "Paused",
                };
                ui.label(state_label);
                if let Some(world_name) = &self.state.shell.status_bar.world_name {
                    ui.separator();
                    ui.label(format!("World: {}", world_name));
                }
                ui.separator();
                ui.label(format!("{:.0} FPS", self.state.shell.status_bar.fps));
                ui.label(format!("{:.1}ms", self.state.shell.status_bar.frame_time_ms));
                ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                    if let Some(message) = &self.state.shell.status_bar.message {
                        ui.label(message);
                    }
                });
            });
        });
    }

    pub fn render_command_palette(&mut self, ctx: &eframe::egui::Context) {
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
        eframe::egui::Area::new("command_palette".into())
            .anchor(eframe::egui::Align2::CENTER_TOP, [0.0, 48.0])
            .show(ctx, |ui| {
                eframe::egui::Frame::popup(ui.style()).show(ui, |ui| {
                    let mut query = self.state.shell.command_palette.query.clone();
                    if ui
                        .add(
                            eframe::egui::TextEdit::singleline(&mut query)
                                .desired_width(420.0)
                                .hint_text("Type a command"),
                        )
                        .changed()
                    {
                        self.state.shell.command_palette_input(&query);
                    }
                    ui.separator();
                    eframe::egui::ScrollArea::vertical()
                        .max_height(240.0)
                        .show(ui, |ui| {
                            for (index, command) in filtered_commands.iter().enumerate() {
                                let selected = index == self.state.shell.command_palette.selected_index;
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
        if ctx.input(|input| input.key_pressed(eframe::egui::Key::Escape)) {
            self.state.shell.close_command_palette();
        }
    }

    pub fn render_panels(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
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

    pub fn render_open_world_dialog(&mut self, _ctx: &eframe::egui::Context) {
        // Stub: no-op unless project dialogs are open
    }

    // -----------------------------------------------------------------------
    // Panel rendering stubs
    // -----------------------------------------------------------------------

    fn render_viewport(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Viewport");
        });
    }

    fn render_outliner(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::SidePanel::left("outliner")
            .min_width(200.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Outliner");
            });
    }

    fn render_inspector_panel(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::SidePanel::right("inspector")
            .min_width(250.0)
            .max_width(350.0)
            .show(ctx, |ui| {
                ui.heading("Inspector");
            });
    }

    fn render_diagnostics_panel(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::SidePanel::right("diagnostics")
            .min_width(250.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Diagnostics");
            });
    }

    fn render_terrain_panel(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::SidePanel::right("terrain")
            .min_width(250.0)
            .max_width(350.0)
            .show(ctx, |ui| {
                ui.heading("Terrain");
            });
    }

    fn render_sky_panel(&mut self, ctx: &eframe::egui::Context) {
        use editor_dto_law::WeatherRegime;
        eframe::egui::SidePanel::right("sky_panel")
            .min_width(300.0)
            .max_width(360.0)
            .show(ctx, |ui| {
                ui.heading("Environment");
                ui.separator();
                ui.label("Time");
                let mut new_time = self.state.sky_editor_state.time_of_day;
                if ui
                    .add(eframe::egui::Slider::new(&mut new_time, 0.0..=24.0).text("hours"))
                    .changed()
                {
                    self.state.sky_editor_state.time_of_day = new_time;
                }
                ui.separator();
                ui.label("Weather");
                let mut selected = self.state.sky_editor_state.weather_regime;
                eframe::egui::ComboBox::from_id_salt("weather_regime")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        for regime in [
                            WeatherRegime::Clear,
                            WeatherRegime::Scattered,
                            WeatherRegime::Overcast,
                            WeatherRegime::IncomingStorm,
                            WeatherRegime::HeavyStorm,
                            WeatherRegime::PostStormCalm,
                            WeatherRegime::FogMorning,
                            WeatherRegime::WindyOvercast,
                        ] {
                            ui.selectable_value(&mut selected, regime, format!("{:?}", regime));
                        }
                    });
                if selected != self.state.sky_editor_state.weather_regime {
                    self.state.sky_editor_state.weather_regime = selected;
                }
                ui.separator();
                ui.label("Clouds");
                let mut coverage = self.state.sky_editor_state.cloud_coverage;
                if ui
                    .add(eframe::egui::Slider::new(&mut coverage, 0.0..=1.0).text("coverage"))
                    .changed()
                {
                    self.state.sky_editor_state.cloud_coverage = coverage;
                }
            });
    }
}
