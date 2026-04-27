//! Session State Mutations

use crate::selection_state::SelectionState;
use crate::tool_mode::ToolMode;
use crate::{PanelId, WorldIdentity};
use std::path::PathBuf;

use super::types::SessionState;

impl SessionState {
    pub fn set_active_world(&mut self, world: Option<WorldIdentity>) {
        self.active_world = world
    }

    pub fn add_open_panel(&mut self, panel_id: PanelId) {
        if !self.open_panels.contains(&panel_id) {
            self.open_panels.push(panel_id);
        }
    }

    pub fn remove_open_panel(&mut self, panel_id: &PanelId) {
        self.open_panels.retain(|p| p != panel_id);
    }

    pub fn set_focused_panel(&mut self, panel_id: Option<PanelId>) {
        self.focused_panel = panel_id
    }

    pub fn set_selection(&mut self, selection: SelectionState) {
        self.selection_state = selection
    }

    pub fn set_tool_mode(&mut self, mode: ToolMode) {
        self.tool_mode = mode
    }

    pub fn add_recently_opened_world(&mut self, path: PathBuf) {
        self.recently_opened_worlds.retain(|p| p != &path);
        self.recently_opened_worlds.insert(0, path);
        self.recently_opened_worlds.truncate(10);
    }
}
