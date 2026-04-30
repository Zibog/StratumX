use super::{unix_timestamp_secs, ConveyorItem, ConveyorStage};

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
        if item.errors.iter().any(|error| error.contains("import")) {
            return Err("Cannot cook item with import errors".to_string());
        }

        let mut cooked = item;
        cooked.current_stage = ConveyorStage::Cooked;
        cooked.set_metadata("cooked_at", unix_timestamp_secs());
        self.items.push(cooked);
        Ok(self.items.last().unwrap().clone())
    }

    /// Get all cooked items
    pub fn items(&self) -> &[ConveyorItem] {
        &self.items
    }
}
