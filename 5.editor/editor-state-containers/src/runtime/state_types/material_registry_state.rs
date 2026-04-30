//! Material registry state type

use crate::*;

#[derive(Debug, Clone)]
pub struct MaterialRegistryState {
    pub profiles: Vec<MaterialProfile>,
    pub material_profiles: Vec<MaterialProfile>,
}

impl MaterialRegistryState {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            material_profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: MaterialProfile) {
        self.profiles.push(profile.clone());
        self.material_profiles.push(profile);
    }

    pub fn get_profile(&self, id: &MaterialProfileId) -> Option<&MaterialProfile> {
        self.profiles.iter().find(|p| &p.profile_id == id)
    }
}

impl Default for MaterialRegistryState {
    fn default() -> Self {
        Self::new()
    }
}
