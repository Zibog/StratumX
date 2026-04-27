//! Material Diagnostics and Validation

use super::service::MaterialAuthoringService;
use super::types::{CheapnessReport, MaterialDiagnostics};
use crate::model::{ObjectHandle, RuntimeCheapnessRung, TextureSlot, ToolingError};

impl MaterialAuthoringService {
    pub fn validate_cheapness(
        &self,
        handle: ObjectHandle,
    ) -> Result<CheapnessReport, ToolingError> {
        let profile = self
            .registry
            .get_profile(handle)
            .ok_or_else(|| ToolingError::Message("Material profile not found".into()))?;
        let mem: u64 = profile
            .texture_slots
            .keys()
            .map(|s| match s {
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
        if mem > 24 * 1024 * 1024 {
            issues.push("texture memory budget exceeded".into());
        }
        Ok(CheapnessReport {
            passes: issues.is_empty(),
            estimated_texture_memory_bytes: mem,
            texture_count: profile.texture_slots.len(),
            issues,
        })
    }

    pub fn get_diagnostics(
        &self,
        handle: ObjectHandle,
    ) -> Result<MaterialDiagnostics, ToolingError> {
        let coverage = self
            .coverage
            .get(&handle)
            .cloned()
            .or_else(|| self.registry.inspect_branch_coverage(handle).ok())
            .ok_or_else(|| ToolingError::Message("Material profile not found".into()))?;
        let profile = self
            .registry
            .get_profile(handle)
            .ok_or_else(|| ToolingError::Message("Material profile not found".into()))?;
        Ok(MaterialDiagnostics {
            coverage,
            cheapness: self.validate_cheapness(handle)?,
            textures: profile.texture_slots.clone(),
        })
    }
}
