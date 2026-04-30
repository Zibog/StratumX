use editor_dto_law::WeatherRegime;
use eframe::egui;

use super::EditorApp;

impl EditorApp {
    pub(super) fn render_viewport(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Viewport");
        });
    }

    pub(super) fn render_outliner(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("outliner")
            .min_width(200.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Outliner");
            });
    }

    pub(super) fn render_inspector_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("inspector")
            .min_width(250.0)
            .max_width(350.0)
            .show(ctx, |ui| {
                ui.heading("Inspector");
            });
    }

    pub(super) fn render_terrain_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("terrain")
            .min_width(250.0)
            .max_width(350.0)
            .show(ctx, |ui| {
                ui.heading("Terrain");
            });
    }

    pub(super) fn render_sky_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("sky_panel")
            .min_width(300.0)
            .max_width(360.0)
            .show(ctx, |ui| {
                ui.heading("Environment");
                ui.separator();
                ui.label("Time");
                let mut new_time = self.state.sky_editor_state.time_of_day;
                if ui
                    .add(egui::Slider::new(&mut new_time, 0.0..=24.0).text("hours"))
                    .changed()
                {
                    self.state.sky_editor_state.time_of_day = new_time;
                }
                ui.separator();
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
                }
                ui.separator();
                ui.label("Clouds");
                let mut coverage = self.state.sky_editor_state.cloud_coverage;
                if ui
                    .add(egui::Slider::new(&mut coverage, 0.0..=1.0).text("coverage"))
                    .changed()
                {
                    self.state.sky_editor_state.cloud_coverage = coverage;
                }
            });
    }
}
