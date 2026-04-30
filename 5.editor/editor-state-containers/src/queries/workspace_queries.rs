//! Workspace query types

use super::ReadModel;
use crate::owners::workspace_owner::{PanelGeometry, WorkspaceOwner};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WorkspaceLayoutView {
    pub panel_count: usize,
    pub focused_panel: Option<String>,
}

impl ReadModel<WorkspaceOwner> for WorkspaceLayoutView {
    fn build(owner: &WorkspaceOwner) -> Self {
        Self {
            panel_count: owner.open_panel_ids.len(),
            focused_panel: owner.focused_panel.as_ref().map(|p| p.0.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspacePanelPositionsView {
    pub panel_positions: HashMap<String, PanelGeometry>,
}

impl ReadModel<WorkspaceOwner> for WorkspacePanelPositionsView {
    fn build(owner: &WorkspaceOwner) -> Self {
        Self {
            panel_positions: owner
                .panel_positions
                .iter()
                .map(|(panel_id, geometry)| (panel_id.0.clone(), geometry.clone()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceDockingSummaryView {
    pub total_panels: usize,
    pub docking_enabled: bool,
}

impl ReadModel<WorkspaceOwner> for WorkspaceDockingSummaryView {
    fn build(owner: &WorkspaceOwner) -> Self {
        Self {
            total_panels: owner.open_panel_ids.len(),
            docking_enabled: owner.docking_configuration.docking_enabled,
        }
    }
}
