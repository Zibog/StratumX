// Tooling Runtime Core - struct definition and basic operations

use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolingRuntime {
    pub(super) next_object: u64,
    pub(super) next_transaction: u64,
    pub(super) next_proposal: u64,
    pub(super) queued_selection: Vec<ObjectHandle>,
    pub(super) objects: BTreeMap<ObjectHandle, ToolObject>,
    pub(super) transactions: Vec<ToolTransaction>,
    pub(super) active_transaction: Option<u64>,
    pub(super) last_build: Option<BuildArtifact>,
    pub(super) releases: Vec<ReleasePackage>,
    pub(super) active_preview: Option<PreviewSession>,
}

impl Default for ToolingRuntime {
    fn default() -> Self {
        Self {
            next_object: 1,
            next_transaction: 1,
            next_proposal: 1,
            queued_selection: Vec::new(),
            objects: BTreeMap::new(),
            transactions: Vec::new(),
            active_transaction: None,
            last_build: None,
            releases: Vec::new(),
            active_preview: None,
        }
    }
}

impl ToolingRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queue_selection(&mut self, selection: Vec<ObjectHandle>) {
        self.queued_selection = selection;
    }

    pub fn queued_selection(&self) -> &[ObjectHandle] {
        &self.queued_selection
    }

    pub fn create_object(
        &mut self,
        label: impl Into<String>,
        class: ObjectClass,
    ) -> Result<ObjectHandle, ToolingError> {
        let handle = ObjectHandle(self.next_object);
        self.next_object += 1;

        let object = ToolObject {
            handle,
            label: label.into(),
            class,
            active: true,
            fields: BTreeMap::new(),
            tags: std::collections::BTreeSet::new(),
        };

        self.objects.insert(handle, object.clone());

        // Record mutation if within a transaction
        if let Some(transaction_id) = self.active_transaction {
            self.record_mutation(transaction_id, Mutation::ObjectCreated { handle, class });
            self.add_rollback_operation(transaction_id, RollbackOperation::DeleteObject { handle });
        }

        Ok(handle)
    }

    pub(super) fn object_mut(
        &mut self,
        handle: ObjectHandle,
    ) -> Result<&mut ToolObject, ToolingError> {
        self.objects
            .get_mut(&handle)
            .ok_or(ToolingError::UnknownObject)
    }

    /// Record a mutation in the current transaction
    pub(super) fn record_mutation(&mut self, transaction_id: u64, mutation: Mutation) {
        if let Some(transaction) = self
            .transactions
            .iter_mut()
            .find(|t| t.order == transaction_id)
        {
            transaction.mutation_set.push(mutation);
        }
    }

    /// Add a rollback operation to the current transaction
    pub(super) fn add_rollback_operation(
        &mut self,
        transaction_id: u64,
        operation: RollbackOperation,
    ) {
        if let Some(transaction) = self
            .transactions
            .iter_mut()
            .find(|t| t.order == transaction_id)
        {
            transaction.rollback_binding.operations.push(operation);
        }
    }

    /// Get the number of transactions recorded
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get all transactions (for testing)
    pub fn transactions(&self) -> &[ToolTransaction] {
        &self.transactions
    }

    /// Get all objects (for testing)
    pub fn objects(&self) -> &BTreeMap<ObjectHandle, ToolObject> {
        &self.objects
    }
}
