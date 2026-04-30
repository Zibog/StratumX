//! Workspace owner types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub String);

impl PanelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PanelGeometry {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn floating(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn docked(x: f32, y: f32, width: f32, height: f32, _dock_position: crate::DockPosition) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockingConfig {
    pub docking_enabled: bool,
    pub snap_distance: f32,
    pub min_panel_size: (f32, f32),
}

impl Default for DockingConfig {
    fn default() -> Self {
        Self {
            docking_enabled: true,
            snap_distance: 10.0,
            min_panel_size: (100.0, 100.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceOwner {
    pub schema_version: u32,
    pub open_panel_ids: Vec<PanelId>,
    pub panel_positions: HashMap<PanelId, PanelGeometry>,
    pub focused_panel: Option<PanelId>,
    pub docking_configuration: DockingConfig,
}

impl Default for WorkspaceOwner {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceOwner {
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;
    pub fn new() -> Self {
        Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            open_panel_ids: Vec::new(),
            panel_positions: HashMap::new(),
            focused_panel: None,
            docking_configuration: DockingConfig {
                docking_enabled: true,
                snap_distance: 10.0,
                min_panel_size: (100.0, 100.0),
            },
        }
    }
    pub fn add_panel(&mut self, panel_id: PanelId, geometry: PanelGeometry) {
        if !self.open_panel_ids.contains(&panel_id) {
            self.open_panel_ids.push(panel_id.clone());
        }
        self.panel_positions.insert(panel_id, geometry);
    }
    pub fn set_focused_panel(&mut self, panel: Option<PanelId>) {
        self.focused_panel = panel;
    }
    pub fn to_persistence_view(&self) -> crate::persistence::WorkspacePersistenceView {
        crate::persistence::WorkspacePersistenceView {
            schema_version: self.schema_version,
            open_panel_ids: self.open_panel_ids.iter().map(|p| p.0.clone()).collect(),
            panel_positions: self
                .panel_positions
                .iter()
                .map(|(k, v)| {
                    (
                        k.0.clone(),
                        crate::persistence::PanelGeometryStub {
                            x: v.x,
                            y: v.y,
                            width: v.width,
                            height: v.height,
                        },
                    )
                })
                .collect(),
            focused_panel: self.focused_panel.as_ref().map(|p| p.0.clone()),
            docking_configuration: crate::persistence::DockingConfigStub,
        }
    }
}
