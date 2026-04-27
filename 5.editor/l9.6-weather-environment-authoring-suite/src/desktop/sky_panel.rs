//! Launch-critical sky and environment controls.

use editor_dto_law::WeatherRegime;
use eframe::egui;

use super::EditorApp;

impl EditorApp {
    pub fn render_sky_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("sky_panel")
            .min_width(300.0)
            .max_width(360.0)
            .show(ctx, |ui| {
                ui.heading("Environment");
                ui.separator();
                self.render_sky_time(ui);
                ui.separator();
                self.render_sky_weather(ui);
                ui.separator();
                self.render_sky_clouds(ui);
                ui.separator();
                self.render_light_preview(ui);
            });
    }

    fn render_sky_time(&mut self, ui: &mut egui::Ui) {
        ui.label("Time");
        let mut new_time = self.state.sky_editor_state.time_of_day;
        let changed = ui
            .add(egui::Slider::new(&mut new_time, 0.0..=24.0).text("hours"))
            .changed();
        if changed {
            self.state.sky_editor_state.time_of_day = new_time;
            self.request_set_time(new_time);
        }
    }

    fn render_sky_weather(&mut self, ui: &mut egui::Ui) {
        ui.label("Weather");
        let mut selected = self.state.sky_editor_state.weather_regime;
        egui::ComboBox::from_id_salt("weather_regime")
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
            self.request_set_weather(format!("{:?}", selected));
        }
    }

    fn render_sky_clouds(&mut self, ui: &mut egui::Ui) {
        ui.label("Clouds");
        let mut coverage = self.state.sky_editor_state.cloud_coverage;
        if ui
            .add(egui::Slider::new(&mut coverage, 0.0..=1.0).text("coverage"))
            .changed()
        {
            self.state.sky_editor_state.cloud_coverage = coverage;
            self.request_set_cloud_coverage(coverage);
        }
    }

    fn render_light_preview(&mut self, ui: &mut egui::Ui) {
        ui.label("Preview");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.state.viewport_preview_state.view_mode,
                ViewMode::Final,
                "Final",
            );
            ui.selectable_value(
                &mut self.state.viewport_preview_state.view_mode,
                ViewMode::Albedo,
                "Albedo",
            );
            ui.selectable_value(
                &mut self.state.viewport_preview_state.view_mode,
                ViewMode::Normal,
                "Normal",
            );
        });
        ui.checkbox(
            &mut self.state.viewport_preview_state.show_shadow_map,
            "Shadow Map",
        );
        ui.checkbox(
            &mut self.state.viewport_preview_state.show_light_cache,
            "Light Cache",
        );
    }
}

#[derive(Debug, Clone)]
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

#[derive(Default)]
pub struct ViewportPreviewState {
    pub view_mode: ViewMode,
    pub show_shadow_map: bool,
    pub show_light_cache: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Final,
    Albedo,
    Normal,
}
