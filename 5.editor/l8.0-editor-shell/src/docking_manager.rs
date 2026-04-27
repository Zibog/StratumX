//! Docking manager for shell-owned panel layout and drag/drop state.

use std::collections::BTreeMap;

use crate::panel_registry::{DockPosition, DockingConfig, PanelGeometry};

use crate::layout_persistence::LayoutState;

pub struct DockingManager {
    open_panels: Vec<String>,
    panel_geometries: BTreeMap<String, PanelGeometry>,
    focused_panel: Option<String>,
    docking_configuration: DockingConfig,
}

impl DockingManager {
    pub fn new() -> Self {
        let open_panels = vec![
            "viewport".to_string(),
            "outliner".to_string(),
            "inspector".to_string(),
            "diagnostics".to_string(),
            "terrain".to_string(),
            "environment".to_string(),
        ];
        let layout = LayoutState::from_visible_panels(&open_panels);
        Self {
            open_panels,
            panel_geometries: layout.panel_geometries(),
            focused_panel: layout.focused_panel,
            docking_configuration: layout.docking_configuration,
        }
    }

    pub fn visible_panels(&self) -> Vec<String> {
        self.open_panels.clone()
    }

    pub fn focused_panel(&self) -> Option<&str> {
        self.focused_panel.as_deref()
    }

    pub fn panel_geometry(&self, panel_id: &str) -> Option<&PanelGeometry> {
        self.panel_geometries.get(panel_id)
    }

    pub fn panel_geometries(&self) -> &BTreeMap<String, PanelGeometry> {
        &self.panel_geometries
    }

    pub fn docking_configuration(&self) -> &DockingConfig {
        &self.docking_configuration
    }

    pub fn set_docking_configuration(&mut self, config: DockingConfig) {
        self.docking_configuration = config;
    }

    pub fn toggle_panel(&mut self, panel_id: &str) {
        if let Some(index) = self.open_panels.iter().position(|panel| panel == panel_id) {
            self.open_panels.remove(index);
            if self.focused_panel.as_deref() == Some(panel_id) {
                self.focused_panel = self.open_panels.last().cloned();
            }
            return;
        }

        self.ensure_geometry(panel_id);
        self.open_panels.push(panel_id.to_string());
        self.focus_panel(panel_id);
    }

    pub fn set_visible_panels(&mut self, panels: &[&str]) {
        self.open_panels = panels.iter().map(|panel| (*panel).to_string()).collect();
        for panel in panels {
            self.ensure_geometry(panel);
        }
        self.focused_panel = panels.first().map(|panel| (*panel).to_string());
    }

    pub fn register_panel(&mut self, panel_id: &str, geometry: PanelGeometry) {
        self.panel_geometries.insert(panel_id.to_string(), geometry);
        if !self.open_panels.iter().any(|panel| panel == panel_id) {
            self.open_panels.push(panel_id.to_string());
        }
    }

    pub fn focus_panel(&mut self, panel_id: &str) {
        if self.open_panels.iter().any(|panel| panel == panel_id) {
            self.focused_panel = Some(panel_id.to_string());
        }
    }

    pub fn dock_panel(&mut self, panel_id: &str, position: DockPosition, viewport_size: [f32; 2]) {
        self.ensure_geometry(panel_id);
        if let Some(geometry) = self.panel_geometries.get_mut(panel_id) {
            geometry.dock_position = position;
            match position {
                DockPosition::Left => {
                    geometry.x = 0.0;
                    geometry.y = 24.0;
                    geometry.width = viewport_size[0] * 0.18;
                    geometry.height = viewport_size[1] - 24.0;
                }
                DockPosition::Right => {
                    geometry.width = viewport_size[0] * 0.22;
                    geometry.height = viewport_size[1] - 24.0;
                    geometry.x = viewport_size[0] - geometry.width;
                    geometry.y = 24.0;
                }
                DockPosition::Top => {
                    geometry.x = 0.0;
                    geometry.y = 24.0;
                    geometry.width = viewport_size[0];
                    geometry.height = viewport_size[1] * 0.2;
                }
                DockPosition::Bottom => {
                    geometry.width = viewport_size[0];
                    geometry.height = viewport_size[1] * 0.2;
                    geometry.x = 0.0;
                    geometry.y = viewport_size[1] - geometry.height;
                }
                DockPosition::Center => {
                    geometry.x = viewport_size[0] * 0.18;
                    geometry.y = 24.0;
                    geometry.width = viewport_size[0] * 0.64;
                    geometry.height = viewport_size[1] - 24.0;
                }
                DockPosition::Floating => {}
            }
        }
    }

    pub fn drag_panel(
        &mut self,
        panel_id: &str,
        target_position: (f32, f32),
        viewport_size: [f32; 2],
    ) {
        self.ensure_geometry(panel_id);
        let snap = self.docking_configuration.snap_distance;
        if let Some(geometry) = self.panel_geometries.get_mut(panel_id) {
            geometry.x = target_position.0;
            geometry.y = target_position.1;
            geometry.dock_position = if target_position.0 <= snap {
                DockPosition::Left
            } else if target_position.0 + geometry.width >= viewport_size[0] - snap {
                DockPosition::Right
            } else if target_position.1 <= snap {
                DockPosition::Top
            } else if target_position.1 + geometry.height >= viewport_size[1] - snap {
                DockPosition::Bottom
            } else {
                DockPosition::Floating
            };
        }
    }

    pub fn to_layout_state(&self) -> LayoutState {
        LayoutState::from_panels_and_docking(
            &self.open_panels,
            &self.panel_geometries,
            self.focused_panel.clone(),
            self.docking_configuration.clone(),
        )
    }

    pub fn restore_layout(&mut self, layout: LayoutState) {
        self.open_panels = layout.visible_panel_ids();
        self.panel_geometries = layout.panel_geometries();
        self.focused_panel = layout.focused_panel;
        self.docking_configuration = layout.docking_configuration;
    }

    fn ensure_geometry(&mut self, panel_id: &str) {
        if self.panel_geometries.contains_key(panel_id) {
            return;
        }

        let layout = LayoutState::from_visible_panels(&[panel_id.to_string()]);
        let mut geometries = layout.panel_geometries();
        if let Some(geometry) = geometries.remove(panel_id) {
            self.panel_geometries.insert(panel_id.to_string(), geometry);
        }
    }
}

impl Default for DockingManager {
    fn default() -> Self {
        Self::new()
    }
}
