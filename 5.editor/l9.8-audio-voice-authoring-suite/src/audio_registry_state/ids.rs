use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Audio source ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AudioSourceId(pub Uuid);

impl AudioSourceId {
    /// Creates a new audio source ID
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

/// Audio zone ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AudioZoneId(pub String);

impl AudioZoneId {
    /// Creates a new audio zone ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Acoustic profile ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AcousticProfileId(pub Uuid);

impl AcousticProfileId {
    /// Creates a new acoustic profile ID
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}
