//! Material Authoring Service - Coverage Cache
//!
//! Manages branch coverage tracking and registry field binding.

use super::session::MaterialAuthoringService;
use crate::model::{BranchCoverage, MaterialRegistry, ObjectHandle, ToolingError};
use std::collections::HashMap;

impl MaterialAuthoringService {
    /// Get coverage map
    pub fn get_coverage(&self) -> &HashMap<ObjectHandle, BranchCoverage> {
        &self.coverage
    }

    /// Refresh coverage for a material handle
    pub fn refresh_coverage(&mut self, handle: ObjectHandle) -> Result<(), ToolingError> {
        let coverage = self.registry.inspect_branch_coverage(handle)?;
        self.coverage.insert(handle, coverage);
        Ok(())
    }

    /// Helper to bind a registry field and refresh coverage
    pub(super) fn bind_registry_field<F>(
        &mut self,
        handle: ObjectHandle,
        apply: F,
    ) -> Result<(), ToolingError>
    where
        F: FnOnce(&mut MaterialRegistry, ObjectHandle) -> Result<(), ToolingError>,
    {
        apply(&mut self.registry, handle)?;
        self.refresh_coverage(handle)
    }
}
