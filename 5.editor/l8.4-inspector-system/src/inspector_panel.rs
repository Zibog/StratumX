//! Minimal inspector for world, terrain, and environment.

use eframe::egui;

use super::EditorApp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionContext {
    #[default]
    World,
    Terrain,
    Sky,
}

impl EditorApp {
    pub fn render_inspector(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("inspector")
            .min_width(320.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Inspector");
                    ui.selectable_value(
                        &mut self.state.inspector_context,
                        SelectionContext::World,
                        "World",
                    );
                    ui.selectable_value(
                        &mut self.state.inspector_context,
                        SelectionContext::Terrain,
                        "Terrain",
                    );
                    ui.selectable_value(
                        &mut self.state.inspector_context,
                        SelectionContext::Sky,
                        "Environment",
                    );
                });

                ui.separator();

                match self.state.inspector_context {
                    SelectionContext::World => self.render_world_inspector(ui),
                    SelectionContext::Terrain => self.render_terrain_inspector(ui),
                    SelectionContext::Sky => self.render_sky_inspector(ui),
                }
            });
    }

    fn render_world_inspector(&mut self, ui: &mut egui::Ui) {
        if let Some(summary) = self.world_summary() {
            ui.label(format!("Name: {}", summary.label));
            ui.label(format!(
                "Terrain: {}x{}",
                summary.terrain.resolution[0], summary.terrain.resolution[1]
            ));
            ui.label(format!(
                "World Size: {:.0} x {:.0}",
                summary.terrain.world_size[0], summary.terrain.world_size[1]
            ));
        } else {
            ui.label("No world loaded");
        }
    }

    fn render_terrain_inspector(&mut self, ui: &mut egui::Ui) {
        if let Some(summary) = self.world_summary() {
            ui.label(format!(
                "Resolution: {}x{}",
                summary.terrain.resolution[0], summary.terrain.resolution[1]
            ));
            ui.label(format!(
                "Chunks: {}x{}",
                summary.terrain.chunk_grid[0], summary.terrain.chunk_grid[1]
            ));
            ui.label(format!("Layers: {}", summary.terrain.layer_count));
        } else {
            ui.label("No terrain loaded");
        }
    }

    fn render_sky_inspector(&mut self, ui: &mut egui::Ui) {
        let mut new_time = self.state.sky_editor_state.time_of_day;
        if ui
            .add(egui::Slider::new(&mut new_time, 0.0..=24.0).text("Time"))
            .changed()
        {
            self.state.sky_editor_state.time_of_day = new_time;
            self.request_set_time(new_time);
        }

        let mut selected_weather = self.state.sky_editor_state.weather_regime;
        egui::ComboBox::from_label("Weather")
            .selected_text(format!("{:?}", selected_weather))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::Clear,
                    "Clear",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::Scattered,
                    "Scattered",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::Overcast,
                    "Overcast",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::IncomingStorm,
                    "Incoming Storm",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::HeavyStorm,
                    "Heavy Storm",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::PostStormCalm,
                    "Post-Storm Calm",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::FogMorning,
                    "Fog Morning",
                );
                ui.selectable_value(
                    &mut selected_weather,
                    editor_dto_law::WeatherRegime::WindyOvercast,
                    "Windy Overcast",
                );
            });
        if selected_weather != self.state.sky_editor_state.weather_regime {
            self.state.sky_editor_state.weather_regime = selected_weather;
            self.request_set_weather(format!("{:?}", selected_weather));
        }

        let mut cloud_coverage = self.state.sky_editor_state.cloud_coverage;
        if ui
            .add(egui::Slider::new(&mut cloud_coverage, 0.0..=1.0).text("Clouds"))
            .changed()
        {
            self.state.sky_editor_state.cloud_coverage = cloud_coverage;
            self.request_set_cloud_coverage(cloud_coverage);
        }
    }
}
