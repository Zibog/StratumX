// Tool Command Application

use super::runtime::ToolingRuntime;
use super::types::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCommand {
    SetLabel {
        handle: ObjectHandle,
        label: String,
    },
    UpsertField {
        handle: ObjectHandle,
        key: String,
        value: String,
    },
    ClearField {
        handle: ObjectHandle,
        key: String,
    },
    AddTag {
        handle: ObjectHandle,
        tag: String,
    },
    RemoveTag {
        handle: ObjectHandle,
        tag: String,
    },
    RetireObject {
        handle: ObjectHandle,
    },
    RestoreObject {
        handle: ObjectHandle,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCommandResult {
    Updated(ObjectHandle),
}

impl ToolingRuntime {
    pub fn apply_command(
        &mut self,
        command: ToolCommand,
        origin: CommandOrigin,
        _approval: ApprovalClass,
        budget: BudgetClass,
    ) -> Result<ToolCommandResult, ToolingError> {
        let (handle, summary) = match command {
            ToolCommand::SetLabel { handle, label } => {
                self.object_mut(handle)?.label = label;
                (handle, "set_label".to_string())
            }
            ToolCommand::UpsertField { handle, key, value } => {
                self.object_mut(handle)?.fields.insert(key, value);
                (handle, "upsert_field".to_string())
            }
            ToolCommand::ClearField { handle, key } => {
                self.object_mut(handle)?.fields.remove(&key);
                (handle, "clear_field".to_string())
            }
            ToolCommand::AddTag { handle, tag } => {
                self.object_mut(handle)?.tags.insert(tag);
                (handle, "add_tag".to_string())
            }
            ToolCommand::RemoveTag { handle, tag } => {
                self.object_mut(handle)?.tags.remove(&tag);
                (handle, "remove_tag".to_string())
            }
            ToolCommand::RetireObject { handle } => {
                self.object_mut(handle)?.active = false;
                (handle, "retire_object".to_string())
            }
            ToolCommand::RestoreObject { handle } => {
                self.object_mut(handle)?.active = true;
                (handle, "restore_object".to_string())
            }
        };

        self.transactions.push(ToolTransaction {
            order: self.next_transaction,
            origin,
            budget,
            summary,
            mutation_set: Vec::new(),
            rollback_binding: RollbackBinding {
                operations: Vec::new(),
            },
            committed: false,
        });
        self.next_transaction += 1;

        Ok(ToolCommandResult::Updated(handle))
    }

    pub fn snapshot(&self) -> Arc<ToolSnapshot> {
        Arc::new(ToolSnapshot {
            objects: self.objects.values().cloned().collect(),
        })
    }

    pub fn validate_snapshot(&self) -> Vec<ValidationDiagnostic> {
        self.objects
            .values()
            .filter(|object| object.label.trim().is_empty())
            .map(|object| ValidationDiagnostic {
                message: format!("object {} has empty label", object.handle.0),
            })
            .collect()
    }

    pub fn ledger(&self) -> &[ToolTransaction] {
        &self.transactions
    }
}
