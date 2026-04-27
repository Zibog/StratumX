use serde::{Deserialize, Serialize};

/// Panel geometry (position and size).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelGeometry {
    /// X position.
    pub x: f32,

    /// Y position.
    pub y: f32,

    /// Width.
    pub width: f32,

    /// Height.
    pub height: f32,

    /// Dock position.
    pub dock_position: DockPosition,
}

impl PanelGeometry {
    /// Creates a new panel geometry.
    pub fn new(x: f32, y: f32, width: f32, height: f32, dock_position: DockPosition) -> Self {
        Self {
            x,
            y,
            width,
            height,
            dock_position,
        }
    }

    /// Creates a floating panel geometry.
    pub fn floating(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::new(x, y, width, height, DockPosition::Floating)
    }

    /// Creates a docked panel geometry.
    pub fn docked(x: f32, y: f32, width: f32, height: f32, position: DockPosition) -> Self {
        Self::new(x, y, width, height, position)
    }
}

impl Default for PanelGeometry {
    fn default() -> Self {
        Self::floating(0.0, 0.0, 800.0, 600.0)
    }
}

/// Dock position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DockPosition {
    /// Floating (not docked).
    Floating,

    /// Docked to left edge.
    Left,

    /// Docked to right edge.
    Right,

    /// Docked to top edge.
    Top,

    /// Docked to bottom edge.
    Bottom,

    /// Docked in center.
    Center,
}

/// Docking configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockingConfig {
    /// Whether docking is enabled.
    pub docking_enabled: bool,

    /// Snap distance for docking (in pixels).
    pub snap_distance: f32,

    /// Minimum panel size.
    pub min_panel_size: (f32, f32),
}

impl DockingConfig {
    /// Creates a new docking configuration with default values.
    pub fn new() -> Self {
        Self {
            docking_enabled: true,
            snap_distance: 10.0,
            min_panel_size: (200.0, 150.0),
        }
    }
}

impl Default for DockingConfig {
    fn default() -> Self {
        Self::new()
    }
}
