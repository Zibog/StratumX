// Material Authority View Types
// Stack version: SX-CANON/1.0.20/STACK-v25
//
// Shared view types for material authority communication between tooling and editor layers.
// These types enable 4.tooling to query material state without depending on 5.editor.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Material identifier using UUID for global uniqueness
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MaterialId(pub Uuid);

impl MaterialId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Opaque object handle for material profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectHandle(pub u64);

impl ObjectHandle {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}

/// Profile metadata containing name and version information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileMetadata {
    pub name: String,
    pub version: u32,
}

impl ProfileMetadata {
    pub fn new(name: String, version: u32) -> Self {
        Self { name, version }
    }
}

/// View of a single material profile for cross-layer queries
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialProfileView {
    pub id: MaterialId,
    pub handle: ObjectHandle,
    pub metadata: ProfileMetadata,
}

impl MaterialProfileView {
    pub fn new(id: MaterialId, handle: ObjectHandle, metadata: ProfileMetadata) -> Self {
        Self {
            id,
            handle,
            metadata,
        }
    }
}

/// View of the material registry for cross-layer queries
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialRegistryView {
    pub materials: Vec<MaterialProfileView>,
}

impl MaterialRegistryView {
    pub fn new(materials: Vec<MaterialProfileView>) -> Self {
        Self { materials }
    }

    pub fn empty() -> Self {
        Self {
            materials: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.materials.len()
    }

    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }
}
