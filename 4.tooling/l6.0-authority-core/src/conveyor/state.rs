use super::artifacts::ConveyorItem;
use super::recovery::record_stage_timestamp;
use super::steps::ConveyorStage;
use super::validation::{ensure_certifiable, ensure_cookable, validate_source_path};

/// Import pipeline - ingests raw assets from source files.
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

    /// Import an asset from its source path.
    pub fn import(
        &mut self,
        asset_id: impl Into<String>,
        source_path: impl Into<String>,
    ) -> Result<&ConveyorItem, String> {
        let source_path = source_path.into();
        validate_source_path(&source_path)?;

        let item = ConveyorItem::new(asset_id, &source_path);
        self.items.push(item);
        Ok(self.items.last().unwrap())
    }

    /// Get all imported items.
    pub fn items(&self) -> &[ConveyorItem] {
        &self.items
    }

    /// Get a specific item by asset ID.
    pub fn get_item(&self, asset_id: &str) -> Option<&ConveyorItem> {
        self.items.iter().find(|item| item.asset_id == asset_id)
    }
}

/// Cook pipeline - processes and transforms assets into optimized formats.
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

    /// Cook an item from the import stage.
    pub fn cook(&mut self, item: ConveyorItem) -> Result<ConveyorItem, String> {
        ensure_cookable(&item)?;

        let mut cooked = item;
        cooked.current_stage = ConveyorStage::Cooked;
        record_stage_timestamp(&mut cooked, "cooked_at");
        self.items.push(cooked);
        Ok(self.items.last().unwrap().clone())
    }

    /// Get all cooked items.
    pub fn items(&self) -> &[ConveyorItem] {
        &self.items
    }
}

/// Certification pipeline - validates and marks assets as production-ready.
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

    /// Certify a cooked item as production-ready.
    pub fn certify(&mut self, item: ConveyorItem) -> Result<ConveyorItem, String> {
        ensure_certifiable(&item)?;

        let mut certified = item;
        certified.current_stage = ConveyorStage::Certified;
        record_stage_timestamp(&mut certified, "certified_at");
        self.certified_items.push(certified);
        Ok(self.certified_items.last().unwrap().clone())
    }

    /// Get all certified items.
    pub fn items(&self) -> &[ConveyorItem] {
        &self.certified_items
    }

    /// Check if an asset is certified.
    pub fn is_certified(&self, asset_id: &str) -> bool {
        self.certified_items
            .iter()
            .any(|item| item.asset_id == asset_id)
    }
}

/// Full tooling conveyor orchestrating import -> cook -> certification.
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

    /// Run an asset through the full conveyor pipeline.
    pub fn process_asset(
        &mut self,
        asset_id: impl Into<String>,
        source_path: impl Into<String>,
    ) -> Result<ConveyorItem, String> {
        let asset_id = asset_id.into();

        let imported = self.import.import(&asset_id, source_path)?;
        let cooked = self.cook.cook(imported.clone())?;

        self.certification.certify(cooked)
    }
}
