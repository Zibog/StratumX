//! Audio executor source binding operations.
//!
//! Handles binding audio sources to world positions and configuring
//! their acoustic properties.

use crate::tooling_tool_session::runtime::ToolingRuntime;
use crate::tooling_tool_session::types::*;

impl ToolingRuntime {
    /// Bind audio source to world position.
    /// This sets the 3D position of an audio source in the world.
    pub fn bind_audio_source_position(
        &mut self,
        handle: ObjectHandle,
        position: [f32; 3],
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_x = obj.fields.get("position_x").cloned();
        let old_y = obj.fields.get("position_y").cloned();
        let old_z = obj.fields.get("position_z").cloned();

        obj.fields
            .insert("position_x".to_string(), position[0].to_string());
        obj.fields
            .insert("position_y".to_string(), position[1].to_string());
        obj.fields
            .insert("position_z".to_string(), position[2].to_string());

        // Record mutations if within a transaction
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: "position_x".to_string(),
                    value: Some(position[0].to_string()),
                },
            );
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: "position_y".to_string(),
                    value: Some(position[1].to_string()),
                },
            );
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: "position_z".to_string(),
                    value: Some(position[2].to_string()),
                },
            );

            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: "position_x".to_string(),
                    value: old_x,
                },
            );
            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: "position_y".to_string(),
                    value: old_y,
                },
            );
            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: "position_z".to_string(),
                    value: old_z,
                },
            );
        }
        Ok(())
    }

    /// Set acoustic profile for audio source.
    /// This configures the acoustic properties of an audio source.
    pub fn set_audio_acoustic_profile(
        &mut self,
        handle: ObjectHandle,
        profile: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("acoustic_profile").cloned();
        obj.fields
            .insert("acoustic_profile".to_string(), profile.clone());

        // Record mutation if within a transaction
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: "acoustic_profile".to_string(),
                    value: Some(profile),
                },
            );
            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: "acoustic_profile".to_string(),
                    value: old_value,
                },
            );
        }
        Ok(())
    }

    /// Assign emitter class to audio source.
    /// This categorizes the audio source for mixing and ducking purposes.
    pub fn assign_audio_emitter_class(
        &mut self,
        handle: ObjectHandle,
        emitter_class: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("emitter_class").cloned();
        obj.fields
            .insert("emitter_class".to_string(), emitter_class.clone());

        // Record mutation if within a transaction
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(
                transaction_id,
                Mutation::FieldSet {
                    handle,
                    field: "emitter_class".to_string(),
                    value: Some(emitter_class),
                },
            );
            self.add_rollback_operation(
                transaction_id,
                RollbackOperation::RestoreField {
                    handle,
                    field: "emitter_class".to_string(),
                    value: old_value,
                },
            );
        }
        Ok(())
    }
}
