//! Session State Types

use crate::selection_state::SelectionState;
use crate::tool_mode::ToolMode;
use crate::{PanelId, WorldIdentity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub active_world: Option<WorldIdentity>,
    pub open_panels: Vec<PanelId>,
    pub focused_panel: Option<PanelId>,
    pub selection_state: SelectionState,
    pub tool_mode: ToolMode,
    pub recently_opened_worlds: Vec<PathBuf>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            active_world: None,
            open_panels: Vec::new(),
            focused_panel: None,
            selection_state: SelectionState::default(),
            tool_mode: ToolMode::Select,
            recently_opened_worlds: Vec::new(),
        }
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}
