//! Workspace persistence view

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePersistenceView {
    pub schema_version: u32,
    pub open_panel_ids: Vec<String>,
    pub panel_positions: HashMap<String, PanelGeometryStub>,
    pub focused_panel: Option<String>,
    pub docking_configuration: DockingConfigStub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelGeometryStub {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingConfigStub;
