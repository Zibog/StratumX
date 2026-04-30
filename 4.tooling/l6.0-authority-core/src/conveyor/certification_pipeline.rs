use super::{unix_timestamp_secs, ConveyorItem, ConveyorStage};

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
        certified.set_metadata("certified_at", unix_timestamp_secs());
        self.certified_items.push(certified);
        Ok(self.certified_items.last().unwrap().clone())
    }

    /// Get all certified items
    pub fn items(&self) -> &[ConveyorItem] {
        &self.certified_items
    }

    /// Check if an asset is certified
    pub fn is_certified(&self, asset_id: &str) -> bool {
        self.certified_items
            .iter()
            .any(|item| item.asset_id == asset_id)
    }
}
