use std::collections::HashMap;

use crate::model::{
    DestructionError, DisabledReason, FieldConfig, FieldId, ObjectHandle, Overlay, OverlayId,
};
use crate::runtime::capture_system::{CaptureData, ComparisonResult, FieldState};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DestructionAuthoringSuite {
    next_field_id: u64,
    next_overlay_id: u64,
    fields: HashMap<FieldId, FieldConfig>,
    overlays: HashMap<OverlayId, (FieldId, Overlay)>,
    field_overlays: HashMap<FieldId, Vec<OverlayId>>,
    pub fracture_targets: Vec<ObjectHandle>,
}

impl DestructionAuthoringSuite {
    pub fn new() -> Self {
        Self {
            next_field_id: 1,
            next_overlay_id: 1,
            fields: HashMap::new(),
            overlays: HashMap::new(),
            field_overlays: HashMap::new(),
            fracture_targets: Vec::new(),
        }
    }

    /// Configure a destruction field with validation
    /// Requirement 5.1: THE Destruction_Authoring_Suite SHALL provide fields
    pub fn configure_destruction_field(
        &mut self,
        config: FieldConfig,
    ) -> Result<FieldId, DestructionError> {
        // Validate configuration
        config
            .validate()
            .map_err(DestructionError::InvalidConfiguration)?;

        // Create new field
        let field_id = FieldId::new(self.next_field_id);
        self.next_field_id += 1;

        self.fields.insert(field_id, config);
        self.field_overlays.insert(field_id, Vec::new());

        Ok(field_id)
    }

    /// Add an overlay to a destruction field
    /// Requirement 5.1: THE Destruction_Authoring_Suite SHALL provide overlays
    pub fn add_overlay(
        &mut self,
        field_id: FieldId,
        overlay: Overlay,
    ) -> Result<OverlayId, DestructionError> {
        // Check if field exists
        if !self.fields.contains_key(&field_id) {
            return Err(DestructionError::UnknownField);
        }

        // Validate overlay
        overlay.validate().map_err(DestructionError::OverlayError)?;

        // Create new overlay
        let overlay_id = OverlayId::new(self.next_overlay_id);
        self.next_overlay_id += 1;

        self.overlays.insert(overlay_id, (field_id, overlay));
        self.field_overlays
            .get_mut(&field_id)
            .unwrap()
            .push(overlay_id);

        Ok(overlay_id)
    }

    /// Get disabled reasons for a field
    /// Requirement 5.1: THE Destruction_Authoring_Suite SHALL provide disabled reasons
    pub fn get_disabled_reasons(&self, field_id: FieldId) -> Vec<DisabledReason> {
        let mut reasons = Vec::new();

        // Check if field exists
        let Some(field) = self.fields.get(&field_id) else {
            reasons.push(DisabledReason::Custom("Field not found".to_string()));
            return reasons;
        };

        // Check if field is disabled
        if !field.enabled {
            reasons.push(DisabledReason::FieldDisabled);
        }

        // Check configuration validity
        if let Err(msg) = field.validate() {
            reasons.push(DisabledReason::InvalidConfiguration(msg));
        }

        // Check overlay count for performance
        if let Some(overlay_ids) = self.field_overlays.get(&field_id) {
            if overlay_ids.len() > 100 {
                reasons.push(DisabledReason::PerformanceLimit);
            }
        }

        // Check if simulation is enabled but field is not
        if field.simulation_enabled && !field.enabled {
            reasons.push(DisabledReason::InvalidConfiguration(
                "Simulation enabled but field is disabled".to_string(),
            ));
        }

        reasons
    }

    /// Capture current destruction state for regression testing
    /// Requirement 5.1: THE Destruction_Authoring_Suite SHALL provide captures
    pub fn capture_destruction_state(&self) -> Result<CaptureData, DestructionError> {
        let mut capture = CaptureData::new();

        // Capture all field states
        for (field_id, field) in &self.fields {
            let overlay_count = self
                .field_overlays
                .get(field_id)
                .map(|v| v.len())
                .unwrap_or(0);

            let state = FieldState {
                enabled: field.enabled,
                fragment_count: field.max_fragments,
                overlay_count,
                simulation_active: field.simulation_enabled,
            };

            capture.add_field_state(*field_id, state);
        }

        // Add metadata
        capture.add_metadata("field_count".to_string(), self.fields.len().to_string());
        capture.add_metadata("overlay_count".to_string(), self.overlays.len().to_string());
        capture.add_metadata(
            "target_count".to_string(),
            self.fracture_targets.len().to_string(),
        );

        Ok(capture)
    }

    /// Compare two captures with detailed difference report
    /// Requirement 5.1: THE Destruction_Authoring_Suite SHALL provide compare loops
    pub fn compare_captures(
        &self,
        capture1: &CaptureData,
        capture2: &CaptureData,
    ) -> ComparisonResult {
        ComparisonResult::compare(capture1, capture2)
    }

    // Helper methods for testing and inspection

    pub fn get_field(&self, field_id: FieldId) -> Option<&FieldConfig> {
        self.fields.get(&field_id)
    }

    pub fn get_overlay(&self, overlay_id: OverlayId) -> Option<&(FieldId, Overlay)> {
        self.overlays.get(&overlay_id)
    }

    pub fn get_field_overlays(&self, field_id: FieldId) -> Vec<OverlayId> {
        self.field_overlays
            .get(&field_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn overlay_count(&self) -> usize {
        self.overlays.len()
    }
}

impl Default for DestructionAuthoringSuite {
    fn default() -> Self {
        Self::new()
    }
}
