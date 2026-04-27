use super::{DockingConfig, PanelGeometry, PanelId, WorkspaceOwner};

impl WorkspaceOwner {
    /// Gets panel geometry.
    pub fn get_panel_geometry(&self, panel_id: &PanelId) -> Option<&PanelGeometry> {
        self.panel_positions.get(panel_id)
    }

    /// Gets all open panels.
    pub fn get_open_panels(&self) -> &[PanelId] {
        &self.open_panel_ids
    }

    /// Gets the currently focused panel.
    pub fn get_focused_panel(&self) -> Option<&PanelId> {
        self.focused_panel.as_ref()
    }

    /// Gets the docking configuration.
    pub fn get_docking_config(&self) -> &DockingConfig {
        &self.docking_configuration
    }

    /// Converts to persistence view for serialization.
    pub fn to_persistence_view(&self) -> crate::persistence::WorkspacePersistenceView {
        crate::persistence::WorkspacePersistenceView::new(
            self.schema_version,
            self.open_panel_ids.clone(),
            self.panel_positions.clone(),
            self.focused_panel.clone(),
            self.docking_configuration.clone(),
        )
    }
}
