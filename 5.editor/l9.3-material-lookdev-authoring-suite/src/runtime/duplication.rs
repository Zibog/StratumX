use crate::model::{MaterialRegistry, ObjectHandle, ToolingError};

impl MaterialRegistry {
    pub fn duplicate_profile(
        &mut self,
        source: ObjectHandle,
        new_name: String,
    ) -> Result<ObjectHandle, ToolingError> {
        let source_profile = self
            .get_profile(source)
            .ok_or_else(|| ToolingError::Message("Profile not found".into()))?
            .clone();

        let handle = ObjectHandle::new(self.next_profile_id);
        self.next_profile_id += 1;

        let mut profile = source_profile;
        profile.handle = handle;
        profile.name = new_name;

        self.profiles.insert(handle, profile);
        Ok(handle)
    }
}
