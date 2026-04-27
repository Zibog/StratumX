use crate::model::{
    EditorProduct, MaterialProfile, MaterialRegistry, ObjectClass, ObjectHandle,
    RuntimeCheapnessRung, ToolingError,
};
use crate::validation::{inspect_branch_coverage, validate_profile_exists};

impl EditorProduct {
    pub fn create_object_with_class(
        &mut self,
        _label: impl Into<String>,
        _class: ObjectClass,
    ) -> Result<ObjectHandle, ToolingError> {
        self.next_handle += 1;
        Ok(ObjectHandle::new(self.next_handle))
    }

    pub fn create_material(
        &mut self,
        label: impl Into<String>,
    ) -> Result<ObjectHandle, ToolingError> {
        self.create_object_with_class(label, ObjectClass::Material)
    }
}

impl MaterialRegistry {
    pub fn new() -> Self {
        Self {
            profiles: Default::default(),
            next_profile_id: 1,
        }
    }

    pub fn create_profile(&mut self, name: String) -> ObjectHandle {
        let handle = ObjectHandle::new(self.next_profile_id);
        self.next_profile_id += 1;

        let profile = MaterialProfile {
            handle,
            name,
            material_archetype_ref: None,
            surface_family_ref: None,
            response_profile_ref: None,
            physical_response_family: None,
            persistence_family: None,
            proof_family: None,
            visual_response: None,
            acoustic_profile: None,
            light_response: None,
            microdetail_profile: None,
            weather_modulation: None,
            texture_slots: Default::default(),
            cheap_runtime_rung: RuntimeCheapnessRung::Unset,
        };

        self.profiles.insert(handle, profile);
        handle
    }

    pub fn get_profile(&self, handle: ObjectHandle) -> Option<&MaterialProfile> {
        self.profiles.get(&handle)
    }

    pub fn inspect_branch_coverage(
        &self,
        handle: ObjectHandle,
    ) -> Result<crate::model::BranchCoverage, ToolingError> {
        let profile = validate_profile_exists(&self.profiles, handle)?;
        Ok(inspect_branch_coverage(profile))
    }
}
