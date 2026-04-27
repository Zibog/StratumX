//! Workspace Mutations
//!
//! Mutation methods for WorkspaceOwner — add/remove panels,
//! update geometry, set focus, configure docking.

use super::workspace_owner::{DockingConfig, PanelGeometry, PanelId, WorkspaceOwner};

impl WorkspaceOwner {
    /// Adds a panel to the workspace
    pub fn add_panel(&mut self, panel_id: PanelId, geometry: PanelGeometry) {
        if !self.open_panel_ids.contains(&panel_id) {
            self.open_panel_ids.push(panel_id.clone());
        }
        self.panel_positions.insert(panel_id, geometry);
    }

    /// Removes a panel from the workspace
    pub fn remove_panel(&mut self, panel_id: &PanelId) {
        self.open_panel_ids.retain(|p| p != panel_id);
        self.panel_positions.remove(panel_id);

        // Clear focus if this panel was focused
        if self.focused_panel.as_ref() == Some(panel_id) {
            self.focused_panel = None;
        }
    }

    /// Updates panel geometry
    pub fn update_panel_geometry(&mut self, panel_id: PanelId, geometry: PanelGeometry) {
        self.panel_positions.insert(panel_id, geometry);
    }

    /// Sets the focused panel
    pub fn set_focused_panel(&mut self, panel_id: Option<PanelId>) {
        self.focused_panel = panel_id;
    }

    /// Adds a panel to the open panels list
    pub fn add_open_panel(&mut self, panel_id: PanelId) {
        if !self.open_panel_ids.contains(&panel_id) {
            self.open_panel_ids.push(panel_id);
        }
    }

    /// Removes a panel from the open panels list
    pub fn remove_open_panel(&mut self, panel_id: &PanelId) {
        self.open_panel_ids.retain(|p| p != panel_id);

        // Clear focus if this panel was focused
        if self.focused_panel.as_ref() == Some(panel_id) {
            self.focused_panel = None;
        }
    }

    /// Sets the docking configuration
    pub fn set_docking_config(&mut self, config: DockingConfig) {
        self.docking_configuration = config;
    }

    /// Enables or disables docking
    pub fn set_docking_enabled(&mut self, enabled: bool) {
        self.docking_configuration.docking_enabled = enabled;
    }

    /// Sets the snap distance for docking
    pub fn set_snap_distance(&mut self, distance: f32) {
        self.docking_configuration.snap_distance = distance;
    }

    /// Sets the minimum panel size
    pub fn set_min_panel_size(&mut self, width: f32, height: f32) {
        self.docking_configuration.min_panel_size = (width, height);
    }
}
