//! Material Authoring Service - Command Handlers
//!
//! Handles material property binding commands.

use super::session::MaterialAuthoringService;
use crate::model::{ObjectHandle, RuntimeCheapnessRung, ToolingError};

impl MaterialAuthoringService {
    /// Assign material archetype
    pub fn assign_material_archetype(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.assign_material_archetype(material, value)
        })
    }

    /// Bind surface family
    pub fn bind_surface_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_surface_family(material, value)
        })
    }

    /// Bind response profile
    pub fn bind_response_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_response_profile(material, value)
        })
    }

    /// Bind physical response family
    pub fn bind_physical_response_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_physical_response_family(material, value)
        })
    }

    /// Bind persistence family
    pub fn bind_persistence_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_persistence_family(material, value)
        })
    }

    /// Bind proof family
    pub fn bind_proof_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_proof_family(material, value)
        })
    }

    /// Bind visual response
    pub fn bind_visual_response(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_visual_response(material, value)
        })
    }

    /// Bind acoustic profile
    pub fn bind_acoustic_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_acoustic_profile(material, value)
        })
    }

    /// Bind light response
    pub fn bind_light_response(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_light_response(material, value)
        })
    }

    /// Bind microdetail profile
    pub fn bind_microdetail_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_microdetail_profile(material, value)
        })
    }

    /// Bind weather modulation
    pub fn bind_weather_modulation(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.bind_weather_modulation(material, value)
        })
    }

    /// Set cheap runtime rung
    pub fn set_cheap_runtime_rung(
        &mut self,
        handle: ObjectHandle,
        rung: RuntimeCheapnessRung,
    ) -> Result<(), ToolingError> {
        self.bind_registry_field(handle, |registry, material| {
            registry.set_cheap_runtime_rung(material, rung)
        })
    }
}
