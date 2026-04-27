//! Workspace State Management
//!
//! Handles workspace state persistence and layout management.

use super::types::{PanelGeometry, WorkspaceState};
use crate::panel_trait::PanelId;
use std::collections::HashMap;

impl WorkspaceState {
    pub fn new() -> Self {
        Self {
            open_panels: Vec::new(),
            focused_panel: None,
            panel_geometries: HashMap::new(),
        }
    }

    pub fn get_open_panels(&self) -> Vec<&PanelId> {
        self.open_panels.iter().collect()
    }

    pub fn add_open_panel(&mut self, panel_id: PanelId) {
        if !self.open_panels.contains(&panel_id) {
            self.open_panels.push(panel_id);
        }
    }

    pub fn remove_open_panel(&mut self, panel_id: &PanelId) {
        self.open_panels.retain(|id| id != panel_id);
    }

    pub fn set_focused_panel(&mut self, panel_id: Option<PanelId>) {
        self.focused_panel = panel_id;
    }

    pub fn get_focused_panel(&self) -> Option<&PanelId> {
        self.focused_panel.as_ref()
    }

    pub fn set_panel_geometry(&mut self, panel_id: PanelId, geometry: PanelGeometry) {
        self.panel_geometries.insert(panel_id, geometry);
    }

    pub fn get_panel_geometry(&self, panel_id: &PanelId) -> Option<&PanelGeometry> {
        self.panel_geometries.get(panel_id)
    }

    pub fn serialize_to_file(&self, path: &std::path::Path) -> Result<(), String> {
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize workspace state: {}", e))?;
        std::fs::write(path, serialized)
            .map_err(|e| format!("Failed to write workspace state: {}", e))
    }

    pub fn deserialize_from_file(path: &std::path::Path) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read workspace state: {}", e))?;
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse workspace state: {}", e))
    }
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self::new()
    }
}
