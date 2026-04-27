//! Audio executor profile binding operations.
//!
//! Handles binding audio zone profiles and ducking policies.

use crate::tooling_tool_session::runtime::ToolingRuntime;
use crate::tooling_tool_session::types::*;

impl ToolingRuntime {
    /// Bind zone profile to audio zone.
    /// This configures the reverb and acoustic properties of an audio zone.
    pub fn bind_audio_zone_profile(
        &mut self,
        handle: ObjectHandle,
        reverb_profile: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("reverb_profile").cloned();
        obj.fields
            .insert("reverb_profile".to_string(), reverb_profile.clone());

        // Record mutation if within a transaction
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: "reverb_profile".to_string(),
                    value: Some(reverb_profile),
                },
            );
            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: "reverb_profile".to_string(),
                    value: old_value,
                },
            );
        }
        Ok(())
    }

    /// Bind ducking policy.
    /// This configures priority-based audio ducking behavior.
    pub fn bind_audio_ducking_policy(&mut self, handle: ObjectHandle) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let was_bound = obj.tags.contains("ducking_policy_bound");
        obj.tags.insert("ducking_policy_bound".to_string());

        // Record mutation if within a transaction
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::TagAdded {
                    handle,
                    tag: "ducking_policy_bound".to_string(),
                },
            );
            if !was_bound {
                self.add_rollback_operation(
                    transaction_id,
                    RollbackOperation::RemoveTag {
                        handle,
                        tag: "ducking_policy_bound".to_string(),
                    },
                );
            }
        }
        Ok(())
    }
}
