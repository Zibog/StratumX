use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::{MaterialProfile, ObjectHandle};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    pub(crate) next_handle: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MaterialRegistry {
    pub(crate) profiles: BTreeMap<ObjectHandle, MaterialProfile>,
    pub(crate) next_profile_id: u64,
}
