use crate::{MaterialProfileId, SurfaceFamilyId};
use serde::{Deserialize, Serialize};

/// Material binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialBinding {
    /// Surface family ID
    pub surface_family_id: SurfaceFamilyId,

    /// Bound material profile ID
    pub material_profile_id: MaterialProfileId,

    /// Binding priority
    pub priority: u32,
}

impl MaterialBinding {
    /// Creates a new material binding
    pub fn new(
        surface_family_id: SurfaceFamilyId,
        material_profile_id: MaterialProfileId,
        priority: u32,
    ) -> Self {
        Self {
            surface_family_id,
            material_profile_id,
            priority,
        }
    }
}
