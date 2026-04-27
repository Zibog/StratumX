use crate::{ApplyPayload, ChangeSet, DeferredWrite, MutationBuffer};
use engine_core::ComponentTypeId;

/// Read-only interface for mutation buffers.
impl MutationBuffer {
    /// Get all staged writes for inspection.
    pub fn writes(&self) -> &[DeferredWrite] {
        &self.writes
    }

    /// Check if a specific component has a staged write.
    pub fn has_write_for(&self, component: ComponentTypeId) -> bool {
        self.writes.iter().any(|w| w.component == component)
    }
}

/// Read-only interface for changesets.
impl ChangeSet {
    /// Get all structural changes.
    pub fn structural_changes(&self) -> &[ComponentTypeId] {
        &self.structural
    }

    /// Get all deferred writes.
    pub fn deferred_writes(&self) -> &[DeferredWrite] {
        &self.writes
    }

    /// Check if changeset has any changes.
    pub fn has_changes(&self) -> bool {
        !self.structural.is_empty() || !self.writes.is_empty()
    }

    /// Count total changes.
    pub fn change_count(&self) -> usize {
        self.structural.len() + self.writes.len()
    }
}

/// Read-only interface for apply payloads.
impl ApplyPayload {
    /// Get all changes this payload will apply.
    pub fn changes(&self) -> &ChangeSet {
        &self.change_set
    }

    /// Is this payload segmented?
    pub fn is_segmented(&self) -> bool {
        self.flags.contains(crate::ApplyFlags::SEGMENTED)
    }

    /// Check total changes this payload contains.
    pub fn total_changes(&self) -> usize {
        self.change_set.change_count()
    }
}
