//! Shell Runtime - thin shell-owned state and orchestration.

use std::path::{Path, PathBuf};

use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

use crate::{
    command_palette_state::CommandPaletteState,
    docking_manager::DockingManager,
    file_dialogs::{show_file_dialog, FileDialogKind},
    layout_persistence::LayoutState,
    project_dialogs::{ProjectDialogKind, ProjectDialogsState},
    stage_strip::{StageStrip, WorkspaceStage},
    status_bar::{MessageType, StatusBar},
};

pub struct ShellRuntime {
    pub workspace_path: Option<PathBuf>,
    pub project_path: Option<PathBuf>,
    pub status_bar: StatusBar,
    pub command_palette: CommandPaletteState,
    pub project_dialogs: ProjectDialogsState,
    pub stage_strip: StageStrip,
    docking_manager: DockingManager,
    pending_commands: Vec<(u64, PromotedCommand)>,
    next_command_id: u64,
    generated_artifacts_root: PathBuf,
}

impl ShellRuntime {
    pub fn new() -> Self {
        let mut runtime = Self {
            workspace_path: None,
            project_path: None,
            status_bar: StatusBar::new(),
            command_palette: CommandPaletteState::new(),
            project_dialogs: ProjectDialogsState::default(),
            stage_strip: StageStrip::new(),
            docking_manager: DockingManager::new(),
            pending_commands: Vec::new(),
            next_command_id: 0,
            generated_artifacts_root: default_generated_artifacts_root(),
        };
        let _ = runtime.refresh_quality_artifacts();
        runtime
    }

    pub fn generated_artifacts_root(&self) -> &Path {
        &self.generated_artifacts_root
    }

    pub fn set_generated_artifacts_root(&mut self, path: PathBuf) {
        self.generated_artifacts_root = path;
    }

    pub fn refresh_quality_artifacts(&mut self) -> Result<(), String> {
        self.status_bar
            .load_quality_summary_from_generated(&self.generated_artifacts_root)
    }

    pub fn submit_command(&mut self, command: PromotedCommand) -> Result<u64, String> {
        self.next_command_id = self.next_command_id.saturating_add(1);
        let command_id = self.next_command_id;
        self.pending_commands.push((command_id, command));
        Ok(command_id)
    }

    pub fn drain_pending_commands(&mut self) -> Vec<(u64, PromotedCommand)> {
        std::mem::take(&mut self.pending_commands)
    }

    pub fn set_status(&mut self, message: impl Into<String>, kind: MessageType) {
        self.status_bar.set_message(message, kind);
    }

    pub fn clear_status(&mut self) {
        self.status_bar.clear_message();
    }

    pub fn get_open_panels(&self) -> Vec<String> {
        self.docking_manager.visible_panels()
    }

    pub fn toggle_panel(&mut self, panel_id: &str) {
        self.docking_manager.toggle_panel(panel_id);
    }

    pub fn activate_stage(&mut self, stage: WorkspaceStage) {
        self.stage_strip.set_active(stage);
        let panels = self.stage_strip.panels_for_active_stage();
        self.docking_manager.set_visible_panels(panels);
    }

    pub fn activate_stage_by_id(&mut self, stage_id: &str) -> bool {
        if !self.stage_strip.set_active_by_id(stage_id) {
            return false;
        }

        let panels = self.stage_strip.panels_for_active_stage();
        self.docking_manager.set_visible_panels(panels);
        true
    }

    pub fn show_file_dialog(&self, kind: FileDialogKind) -> Option<PathBuf> {
        show_file_dialog(kind)
    }

    pub fn show_project_dialog(&mut self, kind: ProjectDialogKind) {
        self.project_dialogs.show(kind);
    }

    pub fn submit_new_project_dialog(&mut self) -> Result<u64, String> {
        let command = self.project_dialogs.build_new_project_command()?;
        self.project_dialogs.close(ProjectDialogKind::NewProject);
        self.submit_command(command)
    }

    pub fn submit_open_world_dialog(&mut self) -> Result<u64, String> {
        let command = self.project_dialogs.build_open_world_command()?;
        self.project_dialogs.close(ProjectDialogKind::OpenWorld);
        self.submit_command(command)
    }

    pub fn open_command_palette(&mut self) {
        self.command_palette.open();
    }

    pub fn close_command_palette(&mut self) {
        self.command_palette.close();
    }

    pub fn command_palette_input(&mut self, query: &str) {
        self.command_palette.input(query);
    }

    pub fn command_palette_up(&mut self, item_count: usize) {
        self.command_palette.move_up(item_count);
    }

    pub fn command_palette_down(&mut self, item_count: usize) {
        self.command_palette.move_down(item_count);
    }

    pub fn save_workspace_layout(&self, path: &Path) -> Result<LayoutState, String> {
        let layout = self.docking_manager.to_layout_state();
        layout.save(path.to_string_lossy().as_ref())?;
        Ok(layout)
    }

    pub fn restore_workspace_layout(&mut self, layout: LayoutState) -> Result<(), String> {
        self.docking_manager.restore_layout(layout);
        Ok(())
    }

    pub fn load_workspace_layout(&mut self, path: &Path) -> Result<LayoutState, String> {
        let layout = LayoutState::load(path.to_string_lossy().as_ref())?;
        self.restore_workspace_layout(layout.clone())?;
        Ok(layout)
    }
}

impl Default for ShellRuntime {
    fn default() -> Self {
        Self::new()
    }
}

pub fn default_generated_artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("7.quality")
        .join("data")
        .join("generated")
}
