use crate::MaterialProfileId;
use serde::{Deserialize, Serialize};

/// Material profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProfile {
    /// Profile ID from registry
    pub profile_id: MaterialProfileId,

    /// Profile name
    pub profile_name: String,

    /// Base color (RGBA)
    pub base_color: [f32; 4],

    /// Metallic factor (0.0 - 1.0)
    pub metallic: f32,

    /// Roughness factor (0.0 - 1.0)
    pub roughness: f32,

    /// Emissive color (RGB)
    pub emissive: [f32; 3],

    /// Normal map strength
    pub normal_strength: f32,
}

impl MaterialProfile {
    /// Creates a new material profile
    pub fn new(profile_id: MaterialProfileId, profile_name: String) -> Self {
        Self {
            profile_id,
            profile_name,
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            emissive: [0.0, 0.0, 0.0],
            normal_strength: 1.0,
        }
    }
}
