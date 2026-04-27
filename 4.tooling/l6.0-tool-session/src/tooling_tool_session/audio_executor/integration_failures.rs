//! Audio executor integration failures.
//!
//! These methods keep audio route metadata explicit and reversible inside
//! the tooling runtime's snapshot, artifact, and world-route surfaces.

use crate::tooling_tool_session::runtime::ToolingRuntime;
use crate::tooling_tool_session::types::*;

impl ToolingRuntime {
    /// Record audio state change to snapshot plane.
    /// This integrates audio changes with the snapshot plane for state capture.
    /// **Requirement 9.5:** Audio operations SHALL integrate with snapshot plane.
    pub fn record_audio_state_change(&mut self, handle: ObjectHandle) -> Result<(), ToolingError> {
        let snapshot_ref = self.allocate_audio_ref("snapshot");
        self.set_audio_field(handle, "audio_snapshot_state", "dirty".to_string())?;
        self.set_audio_field(handle, "audio_snapshot_ref", snapshot_ref)?;
        self.add_audio_tag(handle, "audio_snapshot_dirty")?;
        Ok(())
    }

    /// Store audio evidence in artifact plane.
    /// This records audio configuration and state for certification and audit.
    /// **Requirement 9.5:** Audio operations SHALL integrate with artifact plane.
    pub fn store_audio_artifact(
        &mut self,
        handle: ObjectHandle,
        artifact_type: &str,
    ) -> Result<(), ToolingError> {
        let artifact_type = artifact_type.trim();
        if artifact_type.is_empty() {
            return Err(ToolingError::PreconditionFailed(
                DisabledReason::InvalidInput("artifact type must not be empty".to_string()),
            ));
        }

        let artifact_ref = format!(
            "audio-artifact/{artifact_type}/{}",
            self.allocate_audio_ref("artifact")
        );
        self.set_audio_field(handle, "audio_artifact_type", artifact_type.to_string())?;
        self.set_audio_field(handle, "audio_artifact_ref", artifact_ref)?;
        self.add_audio_tag(handle, "audio_artifact_stored")?;
        Ok(())
    }

    /// Propagate audio state to world truth (L0).
    /// This ensures audio changes flow through SDK to engine world truth.
    /// **Requirement 9.3:** Audio changes SHALL propagate to world truth through tooling planes.
    /// **Requirement 9.7:** Audio persistence SHALL route through world truth.
    pub fn propagate_audio_to_world_truth(
        &mut self,
        handle: ObjectHandle,
    ) -> Result<(), ToolingError> {
        let route_ref = self.allocate_audio_ref("world-route");
        self.set_audio_field(handle, "audio_world_truth_status", "routed".to_string())?;
        self.set_audio_field(handle, "audio_world_truth_route_ref", route_ref)?;
        self.set_audio_field(
            handle,
            "audio_world_truth_owner",
            "engine.world.audio".to_string(),
        )?;
        self.add_audio_tag(handle, "audio_world_truth_routed")?;
        Ok(())
    }

    fn allocate_audio_ref(&mut self, family: &str) -> String {
        let proposal = self.next_proposal;
        self.next_proposal += 1;
        format!("audio/{family}/{proposal:08}")
    }

    fn set_audio_field(
        &mut self,
        handle: ObjectHandle,
        field: &'static str,
        value: String,
    ) -> Result<(), ToolingError> {
        let old_value = {
            let obj = self.object_mut(handle)?;
            obj.fields.insert(field.to_string(), value.clone())
        };

        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: field.to_string(),
                    value: Some(value),
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

        Ok(())
    }

    fn add_audio_tag(
        &mut self,
        handle: ObjectHandle,
        tag: &'static str,
    ) -> Result<(), ToolingError> {
        let inserted = {
            let obj = self.object_mut(handle)?;
            obj.tags.insert(tag.to_string())
        };

        if inserted {
            if let Some(transaction_id) = self.active_transaction {
                self.record_mutation(
                    transaction_id,
                    Mutation::TagAdded {
                        handle,
                        tag: tag.to_string(),
                    },
                );
                self.add_rollback_operation(
                    transaction_id,
                    RollbackOperation::RemoveTag {
                        handle,
                        tag: tag.to_string(),
                    },
                );
            }
        }

        Ok(())
    }
}
