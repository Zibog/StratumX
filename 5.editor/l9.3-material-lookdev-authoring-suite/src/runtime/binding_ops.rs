use crate::model::{
    MaterialRegistry, ObjectHandle, RuntimeCheapnessRung, TextureSlot, ToolingError,
};

impl MaterialRegistry {
    pub fn assign_material_archetype(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.material_archetype_ref = Some(value);
        Ok(())
    }

    pub fn bind_surface_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.surface_family_ref = Some(value);
        Ok(())
    }

    pub fn bind_response_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.response_profile_ref = Some(value);
        Ok(())
    }

    pub fn bind_physical_response_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.physical_response_family = Some(value);
        Ok(())
    }

    pub fn bind_persistence_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.persistence_family = Some(value);
        Ok(())
    }

    pub fn bind_proof_family(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.proof_family = Some(value);
        Ok(())
    }

    pub fn bind_visual_response(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.visual_response = Some(value);
        Ok(())
    }

    pub fn bind_acoustic_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.acoustic_profile = Some(value);
        Ok(())
    }

    pub fn bind_light_response(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.light_response = Some(value);
        Ok(())
    }

    pub fn bind_microdetail_profile(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.microdetail_profile = Some(value);
        Ok(())
    }

    pub fn bind_weather_modulation(
        &mut self,
        handle: ObjectHandle,
        value: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.weather_modulation = Some(value);
        Ok(())
    }

    pub fn set_cheap_runtime_rung(
        &mut self,
        handle: ObjectHandle,
        rung: RuntimeCheapnessRung,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.cheap_runtime_rung = rung;
        Ok(())
    }

    pub fn assign_texture(
        &mut self,
        handle: ObjectHandle,
        slot: TextureSlot,
        path: String,
    ) -> Result<(), ToolingError> {
        let profile = self
            .profiles
            .get_mut(&handle)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?;
        profile.texture_slots.insert(slot, path);
        Ok(())
    }
}
