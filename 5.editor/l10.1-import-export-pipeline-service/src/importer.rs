use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptions {
    pub format: String,
    pub scale: f32,
    pub generate_colliders: bool,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            format: String::from("auto"),
            scale: 1.0,
            generate_colliders: false,
        }
    }
}

pub struct Importer;

impl Importer {
    pub fn import_asset(path: &Path, options: ImportOptions) -> Result<ImportResult, ImportError> {
        Ok(ImportResult {
            asset_id: uuid::Uuid::new_v4(),
            asset_type: String::from("mesh"),
            warnings: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub asset_id: uuid::Uuid,
    pub asset_type: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ImportError {
    FileNotFound,
    UnsupportedFormat,
    ParseError(String),
}