//! Material Executor - material inspection operations
//!
//! Branch coverage inspection and state change recording.

use super::super::runtime::ToolingRuntime;
use super::super::types::*;

impl ToolingRuntime {
    /// Inspect branch coverage for material profile
    pub fn inspect_material_branch_coverage(
        &self,
        handle: ObjectHandle,
    ) -> Result<BranchCoverage, ToolingError> {
        let obj = self
            .objects
            .get(&handle)
            .ok_or(ToolingError::Message("Material profile not found".into()))?;

        let visual_complete = obj.fields.contains_key("visual_response");
        let acoustic_complete = obj.fields.contains_key("acoustic_profile");
        let light_complete = obj.fields.contains_key("light_response");
        let physical_complete = visual_complete && acoustic_complete;
        let runtime_complete = obj
            .fields
            .get("cheap_runtime_rung")
            .and_then(|v: &String| v.parse::<u32>().ok())
            .map(|r| r > 0)
            .unwrap_or(false);

        let mut missing = Vec::new();
        if !visual_complete {
            missing.push("visual_response".to_string());
        }
        if !acoustic_complete {
            missing.push("acoustic_profile".to_string());
        }
        if !light_complete {
            missing.push("light_response".to_string());
        }
        if !runtime_complete {
            missing.push("cheap_runtime_rung".to_string());
        }

        let mut invalid = Vec::new();
        if obj.fields.contains_key("weather_modulation") && !visual_complete {
            invalid.push("weather_modulation requires visual_response".to_string());
        }

        Ok(BranchCoverage {
            physical_complete,
            visual_complete,
            acoustic_complete,
            light_complete,
            runtime_complete,
            missing_bindings: missing,
            invalid_combinations: invalid,
        })
    }

    /// Record material state change to snapshot plane.
    /// Current behavior is a no-op: mutations are accepted here, but snapshot capture is not attached.
    pub fn record_material_state_change(
        &mut self,
        _handle: ObjectHandle,
    ) -> Result<(), ToolingError> {
        Ok(())
    }

    // Helper: record field mutation with rollback
    pub(crate) fn record_mutation_for_field(
        &mut self,
        handle: ObjectHandle,
        field: &str,
        new_value: Option<String>,
        old_value: Option<String>,
    ) {
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: field.to_string(),
                    value: new_value,
                },
            );
            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: field.to_string(),
                    value: old_value,
                },
            );
        }
    }
}
