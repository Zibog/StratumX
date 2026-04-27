use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{ObjectHandle, RuntimeCheapnessRung};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TextureSlot {
    Albedo,
    Normal,
    Roughness,
    Metallic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialProfile {
    pub handle: ObjectHandle,
    pub name: String,

    // Physical/material-first truth
    pub material_archetype_ref: Option<String>,
    pub surface_family_ref: Option<String>,
    pub response_profile_ref: Option<String>,
    pub physical_response_family: Option<String>,
    pub persistence_family: Option<String>,
    pub proof_family: Option<String>,

    // Existing branches
    pub visual_response: Option<String>,
    pub acoustic_profile: Option<String>,
    pub light_response: Option<String>,
    pub microdetail_profile: Option<String>,
    pub weather_modulation: Option<String>,
    pub texture_slots: BTreeMap<TextureSlot, String>,

    // Runtime
    pub cheap_runtime_rung: RuntimeCheapnessRung,
}
