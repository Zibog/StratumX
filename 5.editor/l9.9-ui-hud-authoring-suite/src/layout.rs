//! Layout management
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutType {
    Absolute,
    Horizontal,
    Vertical,
    Grid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub layout_type: LayoutType,
    pub padding: f32,
    pub spacing: f32,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            layout_type: LayoutType::Absolute,
            padding: 0.0,
            spacing: 0.0,
        }
    }
}