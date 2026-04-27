use super::MaterialProfile; // Use the local MaterialProfile from material_registry_state
use crate::{MaterialBinding, MaterialProfileId, ResponseTable, SurfaceFamilyId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Material registry state container
///
/// Owns material-level state including profiles, bindings, and response tables.
/// All material data comes from Material_Registry, never generated at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialRegistryState {
    /// Material profiles indexed by profile ID
    pub material_profiles: HashMap<MaterialProfileId, MaterialProfile>,

    /// Surface family bindings
    pub surface_bindings: HashMap<SurfaceFamilyId, MaterialBinding>,

    /// Response tables for material profiles
    pub response_tables: HashMap<MaterialProfileId, ResponseTable>,
}

impl MaterialRegistryState {
    /// Creates a new empty material registry state
    pub fn new() -> Self {
        Self {
            material_profiles: HashMap::new(),
            surface_bindings: HashMap::new(),
            response_tables: HashMap::new(),
        }
    }

    /// Adds or updates a material profile
    pub fn add_profile(&mut self, profile: MaterialProfile) {
        self.material_profiles
            .insert(profile.profile_id.clone(), profile);
    }

    /// Checks if a profile exists
    pub fn has_profile(&self, profile_id: &MaterialProfileId) -> bool {
        self.material_profiles.contains_key(profile_id)
    }

    /// Removes a material profile
    pub fn remove_profile(&mut self, profile_id: &MaterialProfileId) {
        self.material_profiles.remove(profile_id);
    }

    /// Gets a material profile by ID
    pub fn get_profile(&self, profile_id: &MaterialProfileId) -> Option<&MaterialProfile> {
        self.material_profiles.get(profile_id)
    }

    /// Adds or updates a surface binding
    pub fn add_binding(&mut self, surface_family_id: SurfaceFamilyId, binding: MaterialBinding) {
        self.surface_bindings.insert(surface_family_id, binding);
    }

    /// Removes a surface binding
    pub fn remove_binding(&mut self, surface_family_id: &SurfaceFamilyId) {
        self.surface_bindings.remove(surface_family_id);
    }

    /// Gets a surface binding by surface family ID
    pub fn get_binding(&self, surface_family_id: &SurfaceFamilyId) -> Option<&MaterialBinding> {
        self.surface_bindings.get(surface_family_id)
    }

    /// Adds or updates a response table
    pub fn add_response_table(&mut self, profile_id: MaterialProfileId, table: ResponseTable) {
        self.response_tables.insert(profile_id, table);
    }

    /// Gets a response table by profile ID
    pub fn get_response_table(&self, profile_id: &MaterialProfileId) -> Option<&ResponseTable> {
        self.response_tables.get(profile_id)
    }
}

impl Default for MaterialRegistryState {
    fn default() -> Self {
        Self::new()
    }
}
