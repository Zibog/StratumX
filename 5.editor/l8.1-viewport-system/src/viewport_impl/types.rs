//! Viewport System Types

use serde::{Deserialize, Serialize};
use stratumx_tooling::{ObjectHandle, ToolingError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SceneId(pub String);

impl SceneId {
    pub fn new(value: impl Into<String>) -> Result<Self, ToolingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ToolingError::Message(
                "scene binding cannot be empty".into(),
            ));
        }
        Ok(Self(value))
    }

    pub fn proof_region() -> Self {
        Self("proof_region".into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewportProjection {
    Perspective,
    Top,
    Side,
    Front,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraTransform {
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub fov_deg: f32,
}

impl Default for CameraTransform {
    fn default() -> Self {
        Self {
            position: [0.0, 8.0, -12.0],
            look_at: [0.0, 0.0, 0.0],
            fov_deg: 60.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewportData {
    pub active_viewport: usize,
    pub projection: ViewportProjection,
    pub selected: Vec<ObjectHandle>,
    pub object_count: usize,
    pub active_scene: Option<SceneId>,
    pub camera_bookmark: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportSystem {
    pub active_viewport: usize,
    pub projection: ViewportProjection,
    pub selected: Vec<ObjectHandle>,
    pub scene_binding: Option<SceneId>,
    pub camera: CameraTransform,
    pub camera_bookmark: String,
    pub last_valid_data: Option<ViewportData>,
    pub last_sync_error: Option<String>,
}

impl Default for ViewportSystem {
    fn default() -> Self {
        Self {
            active_viewport: 0,
            projection: ViewportProjection::Perspective,
            selected: Vec::new(),
            scene_binding: None,
            camera: CameraTransform::default(),
            camera_bookmark: String::new(),
            last_valid_data: None,
            last_sync_error: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InspectorSystem {
    pub selected: Option<ObjectHandle>,
}
