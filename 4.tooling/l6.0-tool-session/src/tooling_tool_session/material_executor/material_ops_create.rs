//! Material Executor - material creation operations
//!
//! Material profile creation and duplication operations.

use super::super::runtime::ToolingRuntime;
use super::super::types::*;

impl ToolingRuntime {
    /// Initialize material authority
    pub fn initialize_material_authority(&mut self) -> Result<(), ToolingError> {
        Ok(())
    }

    /// Dispose material authority
    pub fn dispose_material_authority(&mut self) -> Result<(), ToolingError> {
        Ok(())
    }

    /// Create a material profile through MaterialAuthority
    pub fn create_material_profile(&mut self, name: String) -> Result<ObjectHandle, ToolingError> {
        let handle = self.create_object(&name, ObjectClass::Material)?;
        Ok(handle)
    }

    /// Duplicate a material profile through MaterialAuthority
    pub fn duplicate_material_profile(
        &mut self,
        source: ObjectHandle,
        new_name: String,
    ) -> Result<ObjectHandle, ToolingError> {
        if !self.objects.contains_key(&source) {
            return Err(ToolingError::Message(
                "Source material profile not found".into(),
            ));
        }
        let handle = self.create_object(&new_name, ObjectClass::Material)?;
        Ok(handle)
    }
}
