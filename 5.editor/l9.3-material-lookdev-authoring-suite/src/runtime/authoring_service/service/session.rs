//! Material Authoring Service - Session Management
//!
//! Manages session lifecycle, state, and entity bindings.

use crate::model::{BranchCoverage, EntityId, MaterialRegistry, ObjectHandle, ToolingError};
use std::collections::HashMap;

/// Material Authoring Service
///
/// Manages material profiles, entity bindings, and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialAuthoringService {
    pub(super) registry: MaterialRegistry,
    pub(super) bindings: HashMap<EntityId, ObjectHandle>,
    pub(super) coverage: HashMap<ObjectHandle, BranchCoverage>,
}

impl MaterialAuthoringService {
    /// Create a new material authoring service
    pub fn new() -> Self {
        Self {
            registry: MaterialRegistry::new(),
            bindings: HashMap::new(),
            coverage: HashMap::new(),
        }
    }

    /// Bind a material to an entity
    pub fn bind_material(
        &mut self,
        entity_id: EntityId,
        material_handle: ObjectHandle,
    ) -> Result<(), ToolingError> {
        if self.registry.get_profile(material_handle).is_none() {
            return Err(ToolingError::Message("Material profile not found".into()));
        }
        self.bindings.insert(entity_id, material_handle);
        Ok(())
    }

    /// Unbind a material from an entity
    pub fn unbind_material(&mut self, entity_id: EntityId) -> Option<ObjectHandle> {
        self.bindings.remove(&entity_id)
    }

    /// Get the material bound to an entity
    pub fn get_entity_material(&self, entity_id: EntityId) -> Option<ObjectHandle> {
        self.bindings.get(&entity_id).copied()
    }

    /// Get the material registry
    pub fn get_registry(&self) -> &MaterialRegistry {
        &self.registry
    }

    /// Get all entity bindings
    pub fn get_bindings(&self) -> &HashMap<EntityId, ObjectHandle> {
        &self.bindings
    }

    /// Get all profile names
    pub fn get_profile_names(&self) -> Vec<String> {
        self.registry
            .profiles
            .values()
            .map(|profile| profile.name.clone())
            .collect()
    }

    /// Create a new material profile
    pub fn create_material(&mut self, name: String) -> ObjectHandle {
        let handle = self.registry.create_profile(name);
        let _ = self.refresh_coverage(handle);
        handle
    }

    /// Duplicate an existing material profile
    pub fn duplicate_material(
        &mut self,
        source: ObjectHandle,
        new_name: String,
    ) -> Result<ObjectHandle, ToolingError> {
        let handle = self.registry.duplicate_profile(source, new_name)?;
        self.refresh_coverage(handle)?;
        Ok(handle)
    }
}

impl Default for MaterialAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}
