//! Audio Source - world-placed audio emitter.

use serde::{Deserialize, Serialize};

use crate::model::ObjectHandle;

/// Audio Source - world-placed audio emitter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioSource {
    pub handle: ObjectHandle,
    pub name: String,
    pub position: [i32; 3],
    pub emitter_class: Option<String>,
    pub acoustic_profile: Option<String>,
    pub priority: u32,
}
