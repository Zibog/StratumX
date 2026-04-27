//! UI Styling system
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub background_color: Color,
    pub text_color: Color,
    pub font_size: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background_color: Color::rgba(0.2, 0.2, 0.2, 1.0),
            text_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            font_size: 14.0,
            border_width: 0.0,
            border_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}