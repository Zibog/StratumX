//! Audio Zone - reverb/occlusion zone.

use serde::{Deserialize, Serialize};

use crate::model::ObjectHandle;

/// Audio Zone - reverb/occlusion zone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioZone {
    pub handle: ObjectHandle,
    pub name: String,
    pub reverb_profile: Option<String>,
    pub indoor: bool,
}
