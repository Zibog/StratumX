//! Material Authoring Service - Validation Logic
//!
//! Validates material properties and cheapness constraints.

use super::session::MaterialAuthoringService;
use crate::model::{ObjectHandle, RuntimeCheapnessRung, TextureSlot, ToolingError};
use crate::runtime::authoring_service::types::CheapnessReport;

impl MaterialAuthoringService {
    /// Bind material family with validation
    pub fn bind_material_family(
        &mut self,
        handle: ObjectHandle,
        family: String,
    ) -> Result<(), ToolingError> {
        require_namespaced_value("Material family", &family)?;
        self.bind_surface_family(handle, family)
    }

    /// Set response profile with validation
    pub fn set_response_profile(
        &mut self,
        handle: ObjectHandle,
        profile: String,
    ) -> Result<(), ToolingError> {
        require_namespaced_value("Response profile", &profile)?;
        self.bind_response_profile(handle, profile)
    }

    /// Validate material cheapness constraints
    pub fn validate_cheapness(
        &self,
        handle: ObjectHandle,
    ) -> Result<CheapnessReport, ToolingError> {
        let profile = self
            .registry
            .get_profile(handle)
            .ok_or_else(|| ToolingError::Message("Material profile not found".into()))?;

        let estimated_texture_memory_bytes: u64 = profile
            .texture_slots
            .keys()
            .map(|slot| match slot {
                TextureSlot::Albedo | TextureSlot::Normal => 8 * 1024 * 1024,
                _ => 4 * 1024 * 1024,
            })
            .sum();

        let mut issues = Vec::new();
        if profile.cheap_runtime_rung == RuntimeCheapnessRung::Unset {
            issues.push("cheap runtime rung is not configured".into());
        }
        if profile.texture_slots.len() > 4 {
            issues.push("too many texture assignments".into());
        }
        if estimated_texture_memory_bytes > 24 * 1024 * 1024 {
            issues.push("texture memory budget exceeded".into());
        }

        Ok(CheapnessReport {
            passes: issues.is_empty(),
            estimated_texture_memory_bytes,
            texture_count: profile.texture_slots.len(),
            issues,
        })
    }
}

/// Require a namespaced value (contains '.')
fn require_namespaced_value(label: &str, value: &str) -> Result<(), ToolingError> {
    if value.contains('.') {
        Ok(())
    } else {
        Err(ToolingError::Message(format!("{label} must be namespaced")))
    }
}
