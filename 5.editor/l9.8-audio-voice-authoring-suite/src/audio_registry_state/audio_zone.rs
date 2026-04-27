use crate::AudioZoneId;
use serde::{Deserialize, Serialize};

/// Audio zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioZone {
    /// Zone ID from registry
    pub zone_id: AudioZoneId,

    /// Zone name
    pub zone_name: String,

    /// Zone position in world space
    pub position: [f32; 3],

    /// Zone radius
    pub radius: f32,

    /// Attenuation factor
    pub attenuation: f32,
}

impl AudioZone {
    /// Creates a new audio zone
    pub fn new(zone_id: AudioZoneId, zone_name: String) -> Self {
        Self {
            zone_id,
            zone_name,
            position: [0.0, 0.0, 0.0],
            radius: 10.0,
            attenuation: 1.0,
        }
    }
}
