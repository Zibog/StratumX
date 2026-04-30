//! Material Authoring Service - Diagnostics
//!
//! Provides diagnostic information for material profiles.

use super::session::MaterialAuthoringService;
use crate::model::{ObjectHandle, ToolingError};
use crate::runtime::authoring_service::types::MaterialDiagnostics;

impl MaterialAuthoringService {
    /// Get diagnostics for a material profile
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
