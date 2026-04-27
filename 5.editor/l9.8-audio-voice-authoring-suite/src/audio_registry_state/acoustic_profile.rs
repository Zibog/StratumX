use crate::AcousticProfileId;
use serde::{Deserialize, Serialize};

/// Acoustic profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticProfile {
    /// Profile ID from registry
    pub profile_id: AcousticProfileId,

    /// Profile name
    pub profile_name: String,

    /// Reverb amount (0.0 - 1.0)
    pub reverb: f32,

    /// Echo delay in seconds
    pub echo_delay: f32,

    /// Absorption coefficient (0.0 - 1.0)
    pub absorption: f32,
}

impl AcousticProfile {
    /// Creates a new acoustic profile
    pub fn new(profile_id: AcousticProfileId, profile_name: String) -> Self {
        Self {
            profile_id,
            profile_name,
            reverb: 0.0,
            echo_delay: 0.0,
            absorption: 0.5,
        }
    }
}
