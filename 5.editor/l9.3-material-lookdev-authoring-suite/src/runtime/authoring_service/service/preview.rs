//! Material Authoring Service - Preview Functionality
//!
//! Handles material preview and texture assignment.

use super::session::MaterialAuthoringService;
use crate::model::{ObjectHandle, TextureSlot, ToolingError};

impl MaterialAuthoringService {
    /// Assign a texture to a material slot
    pub fn assign_texture(
        &mut self,
        handle: ObjectHandle,
        slot: TextureSlot,
        path: String,
    ) -> Result<(), ToolingError> {
        validate_texture_path(&path)?;
        self.registry.assign_texture(handle, slot, path)?;
        self.refresh_coverage(handle)
    }
}

/// Validate texture file path
fn validate_texture_path(path: &str) -> Result<(), ToolingError> {
    let extension = path
        .rsplit('.')
        .next()
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| ToolingError::Message("Texture path must have an extension".into()))?;

    if matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "tga" | "dds") {
        Ok(())
    } else {
        Err(ToolingError::Message(format!(
            "Unsupported texture format: {}",
            extension
        )))
    }
}
