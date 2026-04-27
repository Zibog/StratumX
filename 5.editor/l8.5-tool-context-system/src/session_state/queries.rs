//! Session State Queries

use crate::selection_state::SelectionState;
use crate::tool_mode::ToolMode;
use crate::{PanelId, WorldIdentity};
use std::path::PathBuf;

use super::types::SessionState;

impl SessionState {
    pub fn get_active_world(&self) -> Option<&WorldIdentity> {
        self.active_world.as_ref()
    }

    pub fn get_open_panels(&self) -> &[PanelId] {
        &self.open_panels
    }

    pub fn get_focused_panel(&self) -> Option<&PanelId> {
        self.focused_panel.as_ref()
    }

    pub fn get_selection_state(&self) -> &SelectionState {
        &self.selection_state
    }

    pub fn get_tool_mode(&self) -> ToolMode {
        self.tool_mode
    }

    pub fn get_recently_opened_worlds(&self) -> &[PathBuf] {
        &self.recently_opened_worlds
    }
}
