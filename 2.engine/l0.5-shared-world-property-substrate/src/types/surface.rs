use serde::{Deserialize, Serialize};

use super::PropertyType;

/// Surface field data (per-surface).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceField {
    /// Property type.
    pub property: PropertyType,
    /// Surface ID.
    pub surface_id: u64,
    /// Per-vertex or per-face values.
    pub values: Vec<f32>,
}

/// Volume field data (3D region).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeField {
    /// Property type.
    pub property: PropertyType,
    /// Volume ID.
    pub volume_id: u64,
    /// Bounding box (min_x, min_y, min_z, max_x, max_y, max_z).
    pub bounds: (f32, f32, f32, f32, f32, f32),
    /// Uniform value or interpolated field.
    pub value: f32,
}
