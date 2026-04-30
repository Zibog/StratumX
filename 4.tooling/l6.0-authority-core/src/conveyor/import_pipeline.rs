use super::ConveyorItem;

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
        let source_path = source_path.into();
        if source_path.is_empty() {
            return Err("Source path cannot be empty".to_string());
        }

        let item = ConveyorItem::new(asset_id, &source_path);
        self.items.push(item);
        Ok(self.items.last().unwrap())
    }

    /// Get all imported items
    pub fn items(&self) -> &[ConveyorItem] {
        &self.items
    }

    /// Get a specific item by asset ID
    pub fn get_item(&self, asset_id: &str) -> Option<&ConveyorItem> {
        self.items.iter().find(|item| item.asset_id == asset_id)
    }
}
