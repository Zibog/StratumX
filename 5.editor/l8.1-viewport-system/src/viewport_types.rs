//! Viewport Types
//!
//! Type definitions for viewport service.

#[derive(Debug, Clone)]
pub struct ViewportConfiguration {
    pub camera_position: [f32; 3],
    pub camera_rotation: [f32; 4],
    pub render_settings: RenderSettings,
    pub viewport_type: ViewportType,
}

impl Default for ViewportConfiguration {
    fn default() -> Self {
        Self {
            camera_position: [0.0, 2.0, -5.0],
            camera_rotation: [0.0, 0.0, 0.0, 1.0],
            render_settings: RenderSettings::default(),
            viewport_type: ViewportType::Perspective,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportType {
    Perspective,
    Orthographic,
    Top,
    Side,
    Front,
}

#[derive(Debug, Clone)]
pub struct RenderSettings {
    pub fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub show_grid: bool,
    pub show_gizmos: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            fov: 75.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            show_grid: true,
            show_gizmos: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportChangeType {
    CameraPosition,
    CameraRotation,
    RenderSettings,
    ActiveViewport,
}

#[derive(Debug, Clone)]
pub struct ViewportChangedEvent {
    pub viewport_id: usize,
    pub change_type: ViewportChangeType,
}
