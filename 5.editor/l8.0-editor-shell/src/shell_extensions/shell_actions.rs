//! Thin shell action helpers for menu buttons and shortcuts.

use stratumx_editor_l8_0_editor_shell::{status_bar::MessageType, ProjectDialogKind};

use super::EditorApp;
use crate::editor_actions::ActionPayload;

impl EditorApp {
    pub fn handle_world_action(&mut self, action_id: &str) {
        match action_id {
            "file.open_world" => {
                self.state
                    .shell
                    .show_project_dialog(ProjectDialogKind::OpenWorld);
            }
            "file.save" => {
                self.dispatch_save_world();
            }
            "file.new_project" => {
                self.state
                    .shell
                    .show_project_dialog(ProjectDialogKind::NewProject);
            }
            "world.play" | "world.pause" | "world.stop" | "world.simulate" => {
                self.dispatch_phase4_action(action_id, None);
            }
            "panel.viewport" => self.state.shell.toggle_panel("viewport"),
            "panel.outliner" => self.state.shell.toggle_panel("outliner"),
            "panel.inspector" => self.state.shell.toggle_panel("inspector"),
            "panel.diagnostics" => self.state.shell.toggle_panel("diagnostics"),
            "panel.terrain" => self.state.shell.toggle_panel("terrain"),
            "panel.environment" => self.state.shell.toggle_panel("environment"),
            _ => {}
        }
    }

    pub fn dispatch_save_world(&mut self) {
        let save_path = if let Some(workspace_path) = &self.state.shell.workspace_path {
            workspace_path.clone()
        } else if let Some(project_path) = &self.state.shell.project_path {
            project_path.clone()
        } else {
            self.shell_status(
                "Cannot save without an active world path",
                MessageType::Error,
            );
            return;
        };

        self.dispatch_phase4_action(
            "file.save",
            Some(ActionPayload::String(
                save_path.to_string_lossy().to_string(),
            )),
        );
    }
}
