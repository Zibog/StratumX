use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub format: String,
    pub compression: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: String::from("gltf"),
            compression: true,
        }
    }
}

pub struct Exporter;

impl Exporter {
    pub fn export_asset(asset_id: uuid::Uuid, path: &Path, options: ExportOptions) -> Result<(), ExportError> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ExportError {
    AssetNotFound,
    WriteError(String),
}