//! Editor Tool Context System

pub mod focus_state;
pub mod selection_state;
pub mod session_errors;
pub mod session_state;
pub mod state_modification;
pub mod tool_mode;
pub mod types;

pub use focus_state::*;
pub use selection_state::*;
pub use session_errors::*;
pub use session_state::*;
pub use state_modification::*;
pub use tool_mode::*;
pub use types::*;

pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorHost {
    pub project_id: Option<String>,
    pub session_id: Option<String>,
    pub active_world_id: Option<String>,
    pub editor_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolContext {
    pub active_tool: String,
    pub tool_settings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorSession {
    pub host: EditorHost,
    pub active_tool: ToolContext,
    pub open_panels: Vec<String>,
    pub focused_panel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorToolContextSystem {
    pub session: EditorSession,
}

impl EditorToolContextSystem {
    pub fn new() -> Self {
        Self {
            session: EditorSession::default(),
        }
    }

    pub fn has_project(&self) -> bool {
        self.session.host.project_id.is_some()
    }

    pub fn has_active_world(&self) -> bool {
        self.session.host.active_world_id.is_some()
    }

    pub fn has_selection(&self) -> bool {
        false
    }
}
