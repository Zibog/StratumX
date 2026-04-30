use eframe::egui;
use stratumx_editor_l7_0_editor_command_spine::app_actions::{ActionDispatcher, ActionPayload};

use super::EditorApp;

impl EditorApp {
    pub fn dispatch_action(&mut self, action_id: &str, payload: Option<ActionPayload>) {
        self.dispatch_phase4_action(action_id, payload);
    }

    /// Dispatch action through the Phase 4 command spine.
    pub fn dispatch_phase4_action(&mut self, action_id: &str, payload: Option<ActionPayload>) {
        if let Some(command) = ActionDispatcher::dispatch(action_id, payload) {
            let _ = self.state.shell.submit_command(command);
        }
    }

    /// Handle world action (open, save, close, panel toggle, etc.).
    pub fn handle_world_action(&mut self, action_id: &str) {
        self.dispatch_phase4_action(action_id, None);
    }

    pub fn flush_shell_commands_to_host(&mut self) {
        // Thin host: no complex command flushing.
        // All commands are processed immediately through action adapters.
    }

    pub(crate) fn handle_shortcuts(&mut self, ctx: &egui::Context) {
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
