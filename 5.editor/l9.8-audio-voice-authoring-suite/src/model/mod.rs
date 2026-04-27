//! Audio model - IDs, enums, immutable data structures.
//!
//! Contains all data types representing audio domain concepts.
//! These types are serializable and form the immutable truth of the audio system.

mod ids;
mod policy;
mod preview;
mod registry;
mod registry_preview;
mod source;
mod zone;

pub use ids::ObjectHandle;
pub use policy::DuckingPolicy;
pub use preview::{
    AudibilityPreview, ObstructionOcclusionResult, TransitionResult, VoiceSubtitleLegality,
};
pub use registry::AudioRegistry;
pub use source::AudioSource;
pub use zone::AudioZone;

use serde::{Deserialize, Serialize};

/// Top-level audio voice authoring suite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioVoiceAuthoringSuite {
    pub audio_nodes: Vec<ObjectHandle>,
    pub audio_registry: AudioRegistry,
}

impl Default for AudioVoiceAuthoringSuite {
    fn default() -> Self {
        Self {
            audio_nodes: Vec::new(),
            audio_registry: AudioRegistry::new(),
        }
    }
}
