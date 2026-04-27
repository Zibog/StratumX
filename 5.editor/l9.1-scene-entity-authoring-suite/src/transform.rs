//! Transform component for entities

use serde::{Deserialize, Serialize};

/// 3D Transform
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion
    pub scale: [f32; 3],
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn with_position(mut self, position: [f32; 3]) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: [f32; 4]) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: [f32; 3]) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

/// Transform operations
pub struct TransformOps;

impl TransformOps {
    pub fn translate(transform: &mut Transform, delta: [f32; 3]) {
        transform.position[0] += delta[0];
        transform.position[1] += delta[1];
        transform.position[2] += delta[2];
    }

    pub fn set_position(transform: &mut Transform, position: [f32; 3]) {
        transform.position = position;
    }

    pub fn set_rotation(transform: &mut Transform, rotation: [f32; 4]) {
        transform.rotation = rotation;
    }

    pub fn set_scale(transform: &mut Transform, scale: [f32; 3]) {
        transform.scale = scale;
    }
}
