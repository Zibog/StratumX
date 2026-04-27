// Tooling Runtime Operations - transaction management

use super::runtime_core_impl::ToolingRuntime;
use super::types::*;

impl ToolingRuntime {
    /// Begin a new transaction for command execution
    /// Returns transaction ID for tracking
    pub fn begin_transaction(&mut self, command_id: String) -> u64 {
        let transaction_id = self.next_transaction;
        self.next_transaction += 1;

        let transaction = ToolTransaction {
            order: transaction_id,
            origin: CommandOrigin::User,
            budget: BudgetClass::Interactive,
            summary: format!("Transaction {} for command: {}", transaction_id, command_id),
            mutation_set: Vec::new(),
            rollback_binding: RollbackBinding {
                operations: Vec::new(),
            },
            committed: false,
        };

        self.transactions.push(transaction);
        self.active_transaction = Some(transaction_id);
        transaction_id
    }

    /// Commit a transaction
    /// Marks the transaction as committed and invalidates affected snapshots
    pub fn commit_transaction(&mut self, transaction_id: u64) -> Result<(), ToolingError> {
        if let Some(transaction) = self
            .transactions
            .iter_mut()
            .find(|t| t.order == transaction_id)
        {
            transaction.committed = true;
            self.active_transaction = None;
            // Affected snapshots are invalidated based on mutation_set (handled by snapshot plane)
            Ok(())
        } else {
            Err(ToolingError::Message(format!(
                "Transaction {} not found",
                transaction_id
            )))
        }
    }

    /// Rollback a transaction
    /// Applies rollback operations in reverse order to restore previous state
    pub fn rollback_transaction(&mut self, transaction_id: u64) -> Result<(), ToolingError> {
        let transaction = self
            .transactions
            .iter()
            .find(|t| t.order == transaction_id)
            .ok_or_else(|| {
                ToolingError::Message(format!("Transaction {} not found", transaction_id))
            })?
            .clone();

        // Apply rollback operations in reverse order
        for operation in transaction.rollback_binding.operations.iter().rev() {
            match operation {
                RollbackOperation::DeleteObject { handle } => {
                    self.objects.remove(handle);
                }
                RollbackOperation::RestoreObject { handle, object } => {
                    self.objects.insert(*handle, object.clone());
                }
                RollbackOperation::RestoreField {
                    handle,
                    field,
                    value,
                } => {
                    if let Some(obj) = self.objects.get_mut(handle) {
                        if let Some(v) = value {
                            obj.fields.insert(field.clone(), v.clone());
                        } else {
                            obj.fields.remove(field);
                        }
                    }
                }
                RollbackOperation::RemoveTag { handle, tag } => {
                    if let Some(obj) = self.objects.get_mut(handle) {
                        obj.tags.remove(tag);
                    }
                }
                RollbackOperation::AddTag { handle, tag } => {
                    if let Some(obj) = self.objects.get_mut(handle) {
                        obj.tags.insert(tag.clone());
                    }
                }
            }
        }

        self.active_transaction = None;
        Ok(())
    }
}
