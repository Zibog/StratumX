//! Editor Viewport System
//!
//! This module provides the viewport system for the editor, including
//! viewport state, camera control, scene binding, and inspector integration.

mod service;
mod types;

pub use service::EditorProduct;
pub use types::{
    CameraTransform, InspectorSystem, SceneId, ViewportData, ViewportProjection, ViewportSystem,
};
