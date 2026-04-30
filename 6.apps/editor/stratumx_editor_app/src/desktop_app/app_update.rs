use eframe::egui;

use super::app_state::WorldSummary;
use super::EditorApp;

impl EditorApp {
    pub(crate) fn run_frame(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();
        self.refresh_performance(now);
        self.refresh_quality_status(now);

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
    pub fn cached_world_summary(&mut self) -> Option<WorldSummary> {
        self.state.cached_world_summary.summary.clone()
    }

    /// Clear the cached world summary.
    pub fn clear_cached_world_summary(&mut self) {
        self.state.cached_world_summary = Default::default();
    }

    fn refresh_performance(&mut self, now: std::time::Instant) {
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
    }

    fn refresh_quality_status(&mut self, now: std::time::Instant) {
        if now.saturating_duration_since(self.state.last_quality_status_refresh)
            >= std::time::Duration::from_secs(2)
        {
            let _ = self.state.shell.refresh_quality_artifacts();
            self.state.last_quality_status_refresh = now;
        }
    }
}
