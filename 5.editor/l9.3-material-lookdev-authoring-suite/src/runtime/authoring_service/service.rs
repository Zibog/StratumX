//! Material Authoring Service Implementation

// CheapnessReport and MaterialDiagnostics reserved for future diagnostics integration
#[allow(unused_imports)]
use super::types::{CheapnessReport, MaterialDiagnostics};
use crate::model::{
    BranchCoverage, EntityId, MaterialRegistry, ObjectHandle, RuntimeCheapnessRung, TextureSlot,
    ToolingError,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialAuthoringService {
    pub(super) registry: MaterialRegistry,
    pub(super) bindings: HashMap<EntityId, ObjectHandle>,
    pub(super) coverage: HashMap<ObjectHandle, BranchCoverage>,
}

impl MaterialAuthoringService {
    pub fn new() -> Self {
        Self {
            registry: MaterialRegistry::new(),
            bindings: HashMap::new(),
            coverage: HashMap::new(),
        }
    }

    pub fn create_material(&mut self, name: String) -> ObjectHandle {
        let handle = self.registry.create_profile(name);
        let _ = self.refresh_coverage(handle);
        handle
    }

    pub fn duplicate_material(
        &mut self,
        source: ObjectHandle,
        new_name: String,
    ) -> Result<ObjectHandle, ToolingError> {
        let handle = self.registry.duplicate_profile(source, new_name)?;
        self.refresh_coverage(handle)?;
        Ok(handle)
    }

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

    pub fn unbind_material(&mut self, entity_id: EntityId) -> Option<ObjectHandle> {
        self.bindings.remove(&entity_id)
    }

    pub fn get_entity_material(&self, entity_id: EntityId) -> Option<ObjectHandle> {
        self.bindings.get(&entity_id).copied()
    }

    pub fn get_registry(&self) -> &MaterialRegistry {
        &self.registry
    }

    pub fn get_bindings(&self) -> &HashMap<EntityId, ObjectHandle> {
        &self.bindings
    }

    pub fn get_coverage(&self) -> &HashMap<ObjectHandle, BranchCoverage> {
        &self.coverage
    }

    pub fn refresh_coverage(&mut self, handle: ObjectHandle) -> Result<(), ToolingError> {
        let coverage = self.registry.inspect_branch_coverage(handle)?;
        self.coverage.insert(handle, coverage);
        Ok(())
    }

    fn bind_registry_field<F>(&mut self, handle: ObjectHandle, f: F) -> Result<(), ToolingError>
    where
        F: FnOnce(&mut MaterialRegistry, ObjectHandle) -> Result<(), ToolingError>,
    {
        f(&mut self.registry, handle)?;
        self.refresh_coverage(handle)
    }

    pub fn assign_material_archetype(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.assign_material_archetype(h, value))
    }

    pub fn bind_surface_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_surface_family(h, value))
    }

    pub fn bind_response_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_response_profile(h, value))
    }

    pub fn bind_physical_response_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_physical_response_family(h, value))
    }

    pub fn bind_persistence_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_persistence_family(h, value))
    }

    pub fn bind_proof_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_proof_family(h, value))
    }

    pub fn bind_visual_response(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_visual_response(h, value))
    }

    pub fn bind_acoustic_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_acoustic_profile(h, value))
    }

    pub fn bind_light_response(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_light_response(h, value))
    }

    pub fn bind_microdetail_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_microdetail_profile(h, value))
    }

    pub fn bind_weather_modulation(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.bind_weather_modulation(h, value))
    }

    pub fn set_cheap_runtime_rung(
        &mut self,
        handle: ObjectHandle,
        rung: RuntimeCheapnessRung,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |r, h| r.set_cheap_runtime_rung(h, rung))
    }

    pub fn bind_material_family(
        &mut self,
        handle: ObjectHandle,
        family: String,
    ) -> Result<(), ToolingError> {
        if !family.contains('.') {
            return Err(ToolingError::Message(
                "Material family must be namespaced".into(),
            ));
        }
        self.bind_surface_family(handle, family)
    }

    pub fn set_response_profile(
        &mut self,
        handle: ObjectHandle,
        profile: String,
    ) -> Result<(), ToolingError> {
        if !profile.contains('.') {
            return Err(ToolingError::Message(
                "Response profile must be namespaced".into(),
            ));
        }
        self.bind_response_profile(handle, profile)
    }

    pub fn assign_texture(
        &mut self,
        handle: ObjectHandle,
        slot: TextureSlot,
        path: String,
    ) -> Result<(), ToolingError> {
        let ext = path
            .rsplit('.')
            .next()
            .map(|e| e.to_ascii_lowercase())
            .ok_or_else(|| ToolingError::Message("Texture path must have an extension".into()))?;
        if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "tga" | "dds") {
            return Err(ToolingError::Message(format!(
                "Unsupported texture format: {}",
                ext
            )));
        }
        self.registry.assign_texture(handle, slot, path)?;
        self.refresh_coverage(handle)
    }

    pub fn get_profile_names(&self) -> Vec<String> {
        self.registry
            .profiles
            .values()
            .map(|p| p.name.clone())
            .collect()
    }
}

impl Default for MaterialAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}
