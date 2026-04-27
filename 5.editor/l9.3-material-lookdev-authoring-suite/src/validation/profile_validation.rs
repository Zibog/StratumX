use std::collections::BTreeMap;

use crate::model::{MaterialProfile, ObjectHandle, ToolingError};

pub fn validate_profile_exists(
    profiles: &BTreeMap<ObjectHandle, MaterialProfile>,
    handle: ObjectHandle,
) -> Result<&MaterialProfile, ToolingError> {
    profiles
        .get(&handle)
        .ok_or_else(|| ToolingError::Message("Profile not found".into()))
}
