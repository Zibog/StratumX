//! Desktop update loop.

use eframe::egui;

use super::EditorApp;

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();
        let delta_time = now.saturating_duration_since(self.state.last_frame_time);
        self.state.last_frame_time = now;

        let frame_time_ms = delta_time.as_secs_f32() * 1000.0;
        let fps = if delta_time.is_zero() {
            0.0
        } else {
            1.0 / delta_time.as_secs_f32()
        };
        self.state
            .shell
            .status_bar
            .update_performance(fps, frame_time_ms);

        if now.saturating_duration_since(self.state.last_quality_status_refresh)
            >= std::time::Duration::from_secs(2)
        {
            let _ = self.state.shell.refresh_quality_artifacts();
            self.state.last_quality_status_refresh = now;
        }

        self.flush_shell_commands_to_host();
        self.sync_ui_from_world();
        self.handle_shortcuts(ctx);

        self.render_main_menu(ctx);
        self.render_stage_strip(ctx);
        self.render_status_bar(ctx);
        self.render_command_palette(ctx);
        self.render_panels(ctx, frame);
        self.render_open_world_dialog(ctx);

        ctx.request_repaint();
    }
}

impl EditorApp {
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|value| value.clone());

        if input.modifiers.ctrl && input.modifiers.shift && input.key_pressed(egui::Key::P) {
            self.state.shell.open_command_palette();
        }

        if input.modifiers.ctrl && input.key_pressed(egui::Key::O) {
            self.handle_world_action("file.open_world");
        }

        if input.modifiers.ctrl && input.key_pressed(egui::Key::S) {
            self.handle_world_action("file.save");
        }

        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num1) {
            self.handle_world_action("panel.viewport");
        }
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num2) {
            self.handle_world_action("panel.outliner");
        }
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num3) {
            self.handle_world_action("panel.inspector");
        }
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num4) {
            self.handle_world_action("panel.diagnostics");
        }
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num5) {
            self.handle_world_action("panel.terrain");
        }
        if input.modifiers.ctrl && input.key_pressed(egui::Key::Num6) {
            self.handle_world_action("panel.environment");
        }
    }
}
