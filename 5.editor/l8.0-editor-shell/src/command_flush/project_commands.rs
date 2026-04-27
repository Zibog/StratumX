//! Project and World Command Handlers

use super::super::EditorApp;
use std::path::{Path, PathBuf};
use stratumx_editor_l8_0_editor_shell::status_bar::MessageType;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

impl EditorApp {
    pub(super) fn handle_project_create(
        &mut self,
        project_name: String,
        project_root: String,
        world_name: String,
    ) -> Result<String, String> {
        match self.runtime_host.create_project_with_world(
            &project_name,
            &project_root,
            &world_name,
        ) {
            Ok(world_dir) => {
                self.state.shell.project_path =
                    Some(world_dir.parent().unwrap().to_path_buf());
                self.state.shell.workspace_path = Some(world_dir.clone());
                self.request_world_open(&world_dir);
                Ok(format!("Project created: {}", project_name))
            }
            Err(e) => Err(e),
        }
    }

    pub(super) fn apply_world_open(&mut self, path: PathBuf) -> Result<String, String> {
        self.runtime_host.open_world_from_path(&path)?;
        self.state.shell.workspace_path = Some(path.clone());
        self.state.shell.project_path = path.parent().map(Path::to_path_buf);
        self.sync_ui_from_world();
        self.runtime_host.mark_terrain_gpu_dirty();
        self.invalidate_cached_world_summary();
        Ok(format!("World opened: {}", path.display()))
    }

    pub(super) fn apply_world_save(&mut self, path: PathBuf) -> Result<String, String> {
        self.runtime_host.save_world_to_path(&path)?;
        self.state.shell.workspace_path = Some(path.clone());
        self.state.shell.project_path = path.parent().map(Path::to_path_buf);
        self.shell_status(
            &format!("World saved: {}", path.display()),
            MessageType::Success,
        );
        Ok(format!("World saved: {}", path.display()))
    }

    pub(super) fn apply_project_save(&mut self, save_path: Option<String>) -> Result<String, String> {
        let project_path = self
            .state
            .shell
            .project_path
            .clone()
            .ok_or_else(|| "No project is currently open".to_string())?;

        let _effective_save_path = match save_path {
            Some(path) => PathBuf::from(path),
            None => self
                .state
                .shell
                .workspace_path
                .clone()
                .unwrap_or_else(|| project_path.join("worlds").join("default")),
        };

        // Save the current world if one is open
        if let Some(ref workspace_path) = self.state.shell.workspace_path {
            let result = self.runtime_host.save_world_to_path(workspace_path);
            match result {
                Ok(_) => {
                    self.shell_status(
                        &format!("Project and world saved: {}", project_path.display()),
                        MessageType::Success,
                    );
                    Ok(format!("Project saved: {}", project_path.display()))
                }
                Err(e) => {
                    self.shell_status(&format!("World save failed: {}", e), MessageType::Error);
                    Err(format!("Failed to save world: {}", e))
                }
            }
        } else {
            self.shell_status(
                &format!("Project saved (no world open): {}", project_path.display()),
                MessageType::Success,
            );
            Ok(format!("Project saved: {}", project_path.display()))
        }
    }
}
