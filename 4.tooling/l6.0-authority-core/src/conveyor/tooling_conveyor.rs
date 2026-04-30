use super::{CertificationPipeline, ConveyorItem, CookPipeline, ImportPipeline};

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

        let imported = self.import.import(&asset_id, source_path)?;
        let imported_item = imported.clone();
        let cooked = self.cook.cook(imported_item)?;

        self.certification.certify(cooked)
    }
}
