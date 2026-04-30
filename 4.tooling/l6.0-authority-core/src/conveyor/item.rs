use super::ConveyorStage;
use std::collections::HashMap;

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
