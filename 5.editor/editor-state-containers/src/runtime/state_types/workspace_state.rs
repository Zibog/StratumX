//! Workspace state type

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub workspace_owner: owners::workspace_owner::WorkspaceOwner,
    pub schema_version: u32,
    pub open_panel_ids: Vec<owners::workspace_owner::PanelId>,
    pub panel_positions: std::collections::HashMap<
        owners::workspace_owner::PanelId,
        owners::workspace_owner::PanelGeometry,
    >,
    pub focused_panel: Option<owners::workspace_owner::PanelId>,
    pub docking_configuration: owners::workspace_owner::DockingConfig,
}

impl WorkspaceState {
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    pub fn new() -> Self {
        let workspace_owner = owners::workspace_owner::WorkspaceOwner::new();
        Self {
            schema_version: workspace_owner.schema_version,
            open_panel_ids: workspace_owner.open_panel_ids.clone(),
            panel_positions: workspace_owner.panel_positions.clone(),
            focused_panel: workspace_owner.focused_panel.clone(),
            docking_configuration: workspace_owner.docking_configuration.clone(),
            workspace_owner,
        }
    }

    pub fn add_panel(
        &mut self,
        panel_id: owners::workspace_owner::PanelId,
        geometry: owners::workspace_owner::PanelGeometry,
    ) {
        self.workspace_owner
            .add_panel(panel_id.clone(), geometry.clone());
        self.open_panel_ids = self.workspace_owner.open_panel_ids.clone();
        self.panel_positions = self.workspace_owner.panel_positions.clone();
    }

    pub fn set_focused_panel(&mut self, panel: Option<owners::workspace_owner::PanelId>) {
        self.workspace_owner.set_focused_panel(panel.clone());
        self.focused_panel = panel;
    }

    pub fn get_open_panels(&self) -> Vec<owners::workspace_owner::PanelId> {
        self.open_panel_ids.clone()
    }

    pub fn serialize_to_file(&self, path: &std::path::Path) -> Result<(), String> {
        let bytes = serde_json::to_vec_pretty(self).map_err(|error| error.to_string())?;
        std::fs::write(path, bytes).map_err(|error| error.to_string())
    }

    pub fn deserialize_from_file(path: &std::path::Path) -> Result<Self, String> {
        let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self::new()
    }
}
