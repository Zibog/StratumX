//! FUTURE_STUB: this crate is intentionally not product-integrated yet.
//! It must not be counted as implemented editor functionality until wired into the active product spine.

//! Role: viewport overlays, gizmos, and visual manipulation handles.
//! Owns: gizmo rendering, transform handles, and selection overlays when integration begins.

pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GizmoType {
    #[default]
    Translate,
    Rotate,
    Scale,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GizmoState {
    pub active: bool,
    pub gizmo_type: GizmoType,
    pub world_position: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OverlayAndGizmoSystem {
    pub gizmo: GizmoState,
    pub overlays: Vec<String>,
    pub snap_to_grid: bool,
    pub grid_size: f32,
}

impl OverlayAndGizmoSystem {
    pub fn new() -> Self {
        Self {
            gizmo: GizmoState::default(),
            overlays: Vec::new(),
            snap_to_grid: false,
            grid_size: 1.0,
        }
    }
}
