//! Panel Registry Types
//!
//! Type definitions for panel registry, lifecycle, dependencies, and workspace state.

use crate::panel_trait::{PanelFactory, PanelId};
use crate::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Panel lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelLifecycle {
    /// Panel not created (no instance exists)
    Cold,
    /// Panel created but not visible (instance exists, not rendered)
    Warm,
    /// Panel visible and active (instance exists and rendered)
    Hot,
}

/// Panel dependencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelDependency {
    /// Panel requires a selection to be active
    RequiresSelection,
    /// Panel requires a project to be open
    RequiresProject,
    /// Panel requires a world to be open
    RequiresWorld,
}

/// Panel definition
#[derive(Clone)]
pub struct PanelDefinition {
    /// Unique panel identifier
    pub panel_id: PanelId,
    /// Human-readable display name
    pub display_name: String,
    /// Default position and size for this panel
    pub default_position: PanelGeometry,
    /// Factory function to create panel instances
    pub factory: Arc<PanelFactory>,
    /// Dependencies that must be met for this panel to be usable
    pub dependencies: Vec<PanelDependency>,
}

/// Panel geometry for docking layout
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PanelGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub dock_position: DockPosition,
}

/// Dock position for panels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DockPosition {
    Floating,
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl Default for PanelGeometry {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 200.0,
            dock_position: DockPosition::Floating,
        }
    }
}

impl PanelGeometry {
    pub fn new(x: f32, y: f32, width: f32, height: f32, dock_position: DockPosition) -> Self {
        Self {
            x,
            y,
            width,
            height,
            dock_position,
        }
    }

    pub fn docked(x: f32, y: f32, width: f32, height: f32, dock_position: DockPosition) -> Self {
        Self {
            x,
            y,
            width,
            height,
            dock_position,
        }
    }

    pub fn floating(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            dock_position: DockPosition::Floating,
        }
    }
}

/// Workspace state for layout persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    #[serde(default)]
    pub(crate) open_panels: Vec<PanelId>,
    #[serde(default)]
    pub(crate) focused_panel: Option<PanelId>,
    #[serde(default)]
    pub(crate) panel_geometries: HashMap<PanelId, PanelGeometry>,
}

/// Docking configuration for workspace layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingConfig {
    #[serde(default)]
    pub splits: Vec<SplitConfig>,
    #[serde(default = "default_snap_distance")]
    pub snap_distance: f32,
}

fn default_snap_distance() -> f32 {
    20.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    #[serde(default)]
    pub direction: SplitDirection,
    #[serde(default)]
    pub ratio: f32,
    #[serde(default)]
    pub panels: Vec<PanelId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl Default for DockingConfig {
    fn default() -> Self {
        Self {
            splits: Vec::new(),
            snap_distance: 20.0,
        }
    }
}
