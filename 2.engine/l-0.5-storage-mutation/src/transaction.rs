use engine_core::{ComponentTypeId, StableDigest64};

use crate::{
    AppliedJournal, ApplyContext, DeferredWrite, InMemoryMutationApplyTarget, MutationFailureReason,
};

pub trait MutationApplyTransaction {
    fn stage_structural_removal(
        &mut self,
        component: ComponentTypeId,
    ) -> Result<(), MutationFailureReason>;
    fn stage_write(&mut self, write: &DeferredWrite) -> Result<(), MutationFailureReason>;
    fn projected_state_digest(&self) -> StableDigest64;
    fn commit(self, journal: AppliedJournal);
    fn rollback(self);
}

pub struct CloneProjectionApplyTransaction<'a> {
    target: &'a mut InMemoryMutationApplyTarget,
    projected: InMemoryMutationApplyTarget,
    _context: ApplyContext,
}

impl<'a> CloneProjectionApplyTransaction<'a> {
    pub(crate) fn new(target: &'a mut InMemoryMutationApplyTarget, context: ApplyContext) -> Self {
        let projected = target.clone();
        Self {
            target,
            projected,
            _context: context,
        }
    }
}

impl MutationApplyTransaction for CloneProjectionApplyTransaction<'_> {
    fn stage_structural_removal(
        &mut self,
        component: ComponentTypeId,
    ) -> Result<(), MutationFailureReason> {
        self.projected.components.remove(&component);
        Ok(())
    }

    fn stage_write(&mut self, write: &DeferredWrite) -> Result<(), MutationFailureReason> {
        self.projected
            .components
            .insert(write.component, write.bytes.clone());
        Ok(())
    }

    fn projected_state_digest(&self) -> StableDigest64 {
        self.projected.stable_state_digest()
    }

    fn commit(self, journal: AppliedJournal) {
        let Self {
            target,
            mut projected,
            _context: _,
        } = self;
        projected
            .applied_journals
            .insert(journal.journal_digest, journal);
        *target = projected;
    }

    fn rollback(self) {}
}
