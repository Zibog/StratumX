use crate::importer::*;
use crate::exporter::*;
use std::path::Path;

pub struct ImportExportService {
    import_queue: Vec<String>,
}

impl ImportExportService {
    pub fn new() -> Self {
        Self {
            import_queue: Vec::new(),
        }
    }

    pub fn import_asset(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        Importer::import_asset(path, ImportOptions::default())
    }

    pub fn export_asset(&self, asset_id: uuid::Uuid, path: &Path) -> Result<(), ExportError> {
        Exporter::export_asset(asset_id, path, ExportOptions::default())
    }
}

impl Default for ImportExportService {
    fn default() -> Self {
        Self::new()
    }
}