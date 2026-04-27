//! Layout Persistence - Save/load layout state
//!
//! Per canon: shell handles persistence, not layout logic.

use crate::panel_registry::{DockPosition, DockingConfig, PanelGeometry};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutState {
    #[serde(default)]
    pub panels: Vec<PanelState>,
    #[serde(default)]
    pub focused_panel: Option<String>,
    #[serde(default)]
    pub docking_configuration: DockingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    pub id: String,
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    #[serde(default = "default_dock_position")]
    pub dock_position: DockPosition,
}

fn default_dock_position() -> DockPosition {
    DockPosition::Floating
}

fn default_panel_geometry(panel_id: &str, index: usize) -> PanelGeometry {
    match panel_id {
        "viewport" => PanelGeometry::docked(280.0, 24.0, 1040.0, 720.0, DockPosition::Center),
        "outliner" => PanelGeometry::docked(0.0, 24.0, 280.0, 720.0, DockPosition::Left),
        "inspector" => PanelGeometry::docked(1320.0, 24.0, 280.0, 720.0, DockPosition::Right),
        "diagnostics" => PanelGeometry::docked(280.0, 744.0, 1040.0, 156.0, DockPosition::Bottom),
        "terrain" => PanelGeometry::docked(1240.0, 24.0, 360.0, 720.0, DockPosition::Right),
        "environment" => PanelGeometry::docked(1240.0, 24.0, 360.0, 720.0, DockPosition::Right),
        _ => PanelGeometry::floating(
            24.0 + ((index % 3) as f32 * 320.0),
            24.0 + ((index / 3) as f32 * 220.0),
            300.0,
            200.0,
        ),
    }
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            panels: Vec::new(),
            focused_panel: None,
            docking_configuration: DockingConfig::default(),
        }
    }
}

impl LayoutState {
    pub fn from_visible_panels(panels: &[String]) -> Self {
        let geometries = panels
            .iter()
            .enumerate()
            .map(|(index, panel_id)| (panel_id.clone(), default_panel_geometry(panel_id, index)))
            .collect::<BTreeMap<_, _>>();
        Self::from_panels_and_docking(
            panels,
            &geometries,
            panels.first().cloned(),
            DockingConfig::default(),
        )
    }

    pub fn from_panels_and_docking(
        panels: &[String],
        geometries: &BTreeMap<String, PanelGeometry>,
        focused_panel: Option<String>,
        docking_configuration: DockingConfig,
    ) -> Self {
        let panels = panels
            .iter()
            .enumerate()
            .map(|(index, panel_id)| {
                let geometry = geometries
                    .get(panel_id)
                    .cloned()
                    .unwrap_or_else(|| default_panel_geometry(panel_id, index));
                PanelState {
                    id: panel_id.clone(),
                    visible: true,
                    x: geometry.x,
                    y: geometry.y,
                    width: geometry.width,
                    height: geometry.height,
                    dock_position: geometry.dock_position,
                }
            })
            .collect();

        Self {
            panels,
            focused_panel,
            docking_configuration,
        }
    }

    pub fn visible_panel_ids(&self) -> Vec<String> {
        self.panels
            .iter()
            .filter(|panel| panel.visible)
            .map(|panel| panel.id.clone())
            .collect()
    }

    pub fn panel_geometries(&self) -> BTreeMap<String, PanelGeometry> {
        self.panels
            .iter()
            .map(|panel| {
                (
                    panel.id.clone(),
                    PanelGeometry::new(
                        panel.x,
                        panel.y,
                        panel.width,
                        panel.height,
                        panel.dock_position,
                    ),
                )
            })
            .collect()
    }

    /// Save layout state to file using JSON persistence.
    pub fn save(&self, path: &str) -> Result<(), String> {
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|error| format!("failed to serialize layout: {error}"))?;
        fs::write(path, serialized).map_err(|error| format!("failed to save layout: {error}"))
    }

    /// Load layout state from file.
    pub fn load(path: &str) -> Result<Self, String> {
        let path = Path::new(path);
        let contents = fs::read_to_string(path)
            .map_err(|error| format!("failed to load layout {}: {error}", path.display()))?;
        serde_json::from_str(&contents)
            .map_err(|error| format!("failed to parse layout {}: {error}", path.display()))
    }
}
