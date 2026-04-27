use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Material profile ID
///
/// Profile IDs must come from Material_Registry, never generated at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialProfileId(pub Uuid);

impl MaterialProfileId {
    /// Creates a new material profile ID
    ///
    /// Note: This should only be called with IDs from Material_Registry
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for MaterialProfileId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

/// Surface family ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SurfaceFamilyId(pub String);

impl SurfaceFamilyId {
    /// Creates a new surface family ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SurfaceFamilyId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for SurfaceFamilyId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}
