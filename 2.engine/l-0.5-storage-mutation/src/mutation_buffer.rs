use crate::{ChangeSet, DeferredWrite, IdempotenceClass, MutationBuffer};
use engine_core::ComponentTypeId;
use smallvec::SmallVec;

impl Default for MutationBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl MutationBuffer {
    pub fn new() -> Self {
        Self {
            writes: SmallVec::new(),
        }
    }

    pub fn stage_write(&mut self, write: DeferredWrite) {
        if let Some(existing) = self.writes.iter_mut().find(|w| {
            w.component == write.component
                && w.idempotence == IdempotenceClass::Idempotent
                && write.idempotence == IdempotenceClass::Idempotent
        }) {
            *existing = write;
            return;
        }
        self.writes.push(write);
    }

    pub fn into_change_set(self, structural: SmallVec<[ComponentTypeId; 8]>) -> ChangeSet {
        ChangeSet {
            structural,
            writes: self.writes,
        }
    }
}
