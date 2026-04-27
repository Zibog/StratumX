use crate::AudioSourceId;
use serde::{Deserialize, Serialize};

/// Audio source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    /// Source ID from registry
    pub source_id: AudioSourceId,

    /// Source name
    pub source_name: String,

    /// Source type
    pub source_type: AudioSourceType,

    /// Volume (0.0 - 1.0)
    pub volume: f32,

    /// Pitch multiplier
    pub pitch: f32,

    /// Whether the source is active
    pub active: bool,
}

impl AudioSource {
    /// Creates a new audio source
    pub fn new(
        source_id: AudioSourceId,
        source_name: String,
        source_type: AudioSourceType,
    ) -> Self {
        Self {
            source_id,
            source_name,
            source_type,
            volume: 1.0,
            pitch: 1.0,
            active: true,
        }
    }
}

/// Audio source type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioSourceType {
    /// Ambient background sound
    Ambient,

    /// Impact/collision sound
    Impact,

    /// Continuous sound (engine, wind, etc.)
    Continuous,

    /// Triggered sound effect
    Trigger,
}
