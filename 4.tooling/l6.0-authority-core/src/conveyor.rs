// Tooling Conveyor - Import/Cook/Certification pipelines
//
// Manages the flow of assets through tooling processing stages:
// - Import: ingest raw assets from source files
// - Cook: process and transform into optimized formats
// - Certification: validate and mark as production-ready

use std::collections::HashMap;

/// Stage in the tooling conveyor pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConveyorStage {
    /// Raw asset ingested from source
    Imported,
    /// Asset processed through cooking pipeline
    Cooked,
    /// Asset validated and certified for production
    Certified,
}

/// Status of an asset in the conveyor
#[derive(Debug, Clone)]
pub struct ConveyorItem {
    pub asset_id: String,
    pub source_path: String,
    pub current_stage: ConveyorStage,
    pub metadata: HashMap<String, String>,
    pub errors: Vec<String>,
}

impl ConveyorItem {
    pub fn new(asset_id: impl Into<String>, source_path: impl Into<String>) -> Self {
        Self {
            asset_id: asset_id.into(),
            source_path: source_path.into(),
            current_stage: ConveyorStage::Imported,
            metadata: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }

    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// Import pipeline - ingests raw assets from source files
pub struct ImportPipeline {
    items: Vec<ConveyorItem>,
}

impl Default for ImportPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportPipeline {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Import an asset from its source path
    pub fn import(
        &mut self,
        asset_id: impl Into<String>,
        source_path: impl Into<String>,
    ) -> Result<&ConveyorItem, String> {
        let source_path_str = source_path.into();
        if source_path_str.is_empty() {
            return Err("Source path cannot be empty".to_string());
        }

        let item = ConveyorItem::new(asset_id, &source_path_str);
        self.items.push(item);
        Ok(self.items.last().unwrap())
    }

    /// Get all imported items
    pub fn items(&self) -> &[ConveyorItem] {
        &self.items
    }

    /// Get a specific item by asset ID
    pub fn get_item(&self, asset_id: &str) -> Option<&ConveyorItem> {
        self.items.iter().find(|i| i.asset_id == asset_id)
    }
}

/// Cook pipeline - processes and transforms assets into optimized formats
pub struct CookPipeline {
    items: Vec<ConveyorItem>,
}

impl Default for CookPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl CookPipeline {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Cook an item from the import stage
    pub fn cook(&mut self, item: ConveyorItem) -> Result<ConveyorItem, String> {
        if item.errors.iter().any(|e| e.contains("import")) {
            return Err("Cannot cook item with import errors".to_string());
        }

        let mut cooked = item;
        cooked.current_stage = ConveyorStage::Cooked;
        cooked.set_metadata(
            "cooked_at",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        );
        self.items.push(cooked);
        Ok(self.items.last().unwrap().clone())
    }

    /// Get all cooked items
    pub fn items(&self) -> &[ConveyorItem] {
        &self.items
    }
}

/// Certification pipeline - validates and marks assets as production-ready
pub struct CertificationPipeline {
    certified_items: Vec<ConveyorItem>,
}

impl Default for CertificationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl CertificationPipeline {
    pub fn new() -> Self {
        Self {
            certified_items: Vec::new(),
        }
    }

    /// Certify a cooked item as production-ready
    pub fn certify(&mut self, item: ConveyorItem) -> Result<ConveyorItem, String> {
        if item.current_stage != ConveyorStage::Cooked {
            return Err(format!(
                "Cannot certify item in {:?} stage - must be Cooked",
                item.current_stage
            ));
        }

        if !item.errors.is_empty() {
            return Err("Cannot certify item with errors".to_string());
        }

        let mut certified = item;
        certified.current_stage = ConveyorStage::Certified;
        certified.set_metadata(
            "certified_at",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        );
        self.certified_items.push(certified);
        Ok(self.certified_items.last().unwrap().clone())
    }

    /// Get all certified items
    pub fn items(&self) -> &[ConveyorItem] {
        &self.certified_items
    }

    /// Check if an asset is certified
    pub fn is_certified(&self, asset_id: &str) -> bool {
        self.certified_items.iter().any(|i| i.asset_id == asset_id)
    }
}

/// Full tooling conveyor orchestrating import -> cook -> certification
pub struct ToolingConveyor {
    pub import: ImportPipeline,
    pub cook: CookPipeline,
    pub certification: CertificationPipeline,
}

impl Default for ToolingConveyor {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolingConveyor {
    pub fn new() -> Self {
        Self {
            import: ImportPipeline::new(),
            cook: CookPipeline::new(),
            certification: CertificationPipeline::new(),
        }
    }

    /// Run an asset through the full conveyor pipeline
    pub fn process_asset(
        &mut self,
        asset_id: impl Into<String>,
        source_path: impl Into<String>,
    ) -> Result<ConveyorItem, String> {
        let asset_id = asset_id.into();

        // Phase 1: Import
        let imported = self.import.import(&asset_id, source_path)?;
        let imported_item = imported.clone();

        // Phase 2: Cook
        let cooked = self.cook.cook(imported_item)?;

        // Phase 3: Certification
        let certified = self.certification.certify(cooked)?;

        Ok(certified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_pipeline() {
        let mut pipeline = ImportPipeline::new();

        let item = pipeline
            .import("mat_001", "/assets/materials/concrete.mat")
            .unwrap();
        assert_eq!(item.asset_id, "mat_001");
        assert_eq!(item.current_stage, ConveyorStage::Imported);

        assert!(pipeline.import("mat_002", "").is_err());
    }

    #[test]
    fn test_cook_pipeline() {
        let mut cook = CookPipeline::new();

        let item = ConveyorItem::new("mat_001", "/assets/materials/concrete.mat");
        let cooked = cook.cook(item).unwrap();
        assert_eq!(cooked.current_stage, ConveyorStage::Cooked);
    }

    #[test]
    fn test_certification_pipeline() {
        let mut cert = CertificationPipeline::new();

        // Must be cooked first
        let item = ConveyorItem::new("mat_001", "/assets/materials/concrete.mat");
        assert!(cert.certify(item).is_err());

        // Create a cooked item
        let mut cooked = ConveyorItem::new("mat_001", "/assets/materials/concrete.mat");
        cooked.current_stage = ConveyorStage::Cooked;

        let certified = cert.certify(cooked).unwrap();
        assert_eq!(certified.current_stage, ConveyorStage::Certified);
        assert!(cert.is_certified("mat_001"));
    }

    #[test]
    fn test_full_conveyor_pipeline() {
        let mut conveyor = ToolingConveyor::new();

        let result = conveyor.process_asset("mat_001", "/assets/materials/concrete.mat");
        assert!(result.is_ok());

        let certified = result.unwrap();
        assert_eq!(certified.current_stage, ConveyorStage::Certified);
        assert_eq!(certified.asset_id, "mat_001");
    }
}
