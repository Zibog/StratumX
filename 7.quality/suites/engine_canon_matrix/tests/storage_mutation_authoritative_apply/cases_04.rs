use engine_core::StableDigestBuilder;
use engine_storage_mutation::{AppliedJournal, MutationApplyTransaction};
use std::collections::BTreeMap;

struct NonCloneMutationTarget {
    bytes: SmallVec<[u8; 32]>,
    journals: BTreeMap<StableDigest64, AppliedJournal>,
    stage_failure: Option<MutationFailureReason>,
}

struct NonCloneTransaction<'a> {
    target: &'a mut NonCloneMutationTarget,
    staged_bytes: SmallVec<[u8; 32]>,
    staged_journals: BTreeMap<StableDigest64, AppliedJournal>,
    stage_failure: Option<MutationFailureReason>,
}

impl NonCloneMutationTarget {
    fn new(bytes: SmallVec<[u8; 32]>) -> Self {
        Self {
            bytes,
            journals: BTreeMap::new(),
            stage_failure: None,
        }
    }

    fn digest(bytes: &[u8]) -> StableDigest64 {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine_storage_mutation.non_clone_target")
            .write_u64(bytes.len() as u64)
            .write_bytes(bytes);
        digest.finish()
    }
}

impl MutationApplyTarget for NonCloneMutationTarget {
    type Transaction<'a>
        = NonCloneTransaction<'a>
    where
        Self: 'a;

    fn component_bytes(&self, component: ComponentTypeId) -> Option<&[u8]> {
        (component == ComponentTypeId(1)).then_some(self.bytes.as_slice())
    }

    fn has_component(&self, component: ComponentTypeId) -> bool {
        component == ComponentTypeId(1)
    }

    fn stable_state_digest(&self) -> StableDigest64 {
        Self::digest(&self.bytes)
    }

    fn applied_journal(&self, journal_digest: StableDigest64) -> Option<AppliedJournal> {
        self.journals.get(&journal_digest).copied()
    }

    fn begin_apply<'a>(&'a mut self, _context: ApplyContext) -> Self::Transaction<'a> {
        let staged_bytes = self.bytes.clone();
        let staged_journals = self.journals.clone();
        let stage_failure = self.stage_failure.take();
        NonCloneTransaction {
            target: self,
            staged_bytes,
            staged_journals,
            stage_failure,
        }
    }
}

impl MutationApplyTransaction for NonCloneTransaction<'_> {
    fn stage_structural_removal(
        &mut self,
        component: ComponentTypeId,
    ) -> Result<(), MutationFailureReason> {
        if component != ComponentTypeId(1) {
            return Err(MutationFailureReason::MissingTarget);
        }
        self.staged_bytes.clear();
        Ok(())
    }

    fn stage_write(&mut self, write: &DeferredWrite) -> Result<(), MutationFailureReason> {
        if let Some(reason) = self.stage_failure {
            return Err(reason);
        }
        if write.component != ComponentTypeId(1) {
            return Err(MutationFailureReason::IllegalOperation);
        }
        self.staged_bytes = write.bytes.clone();
        Ok(())
    }

    fn projected_state_digest(&self) -> StableDigest64 {
        NonCloneMutationTarget::digest(&self.staged_bytes)
    }

    fn commit(self, journal: AppliedJournal) {
        let Self {
            target,
            staged_bytes,
            mut staged_journals,
            stage_failure: _,
        } = self;
        staged_journals.insert(journal.journal_digest, journal);
        target.bytes = staged_bytes;
        target.journals = staged_journals;
    }

    fn rollback(self) {}
}

#[test]
fn authoritative_apply_does_not_require_clone_bound() {
    let mut target = NonCloneMutationTarget::new(smallvec![1, 2, 3]);
    let payload = payload_with_writes(
        21,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![7, 7, 7],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(21, 22, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(receipt.is_success());
    assert_eq!(target.component_bytes(ComponentTypeId(1)), Some(&[7, 7, 7][..]));
}

#[test]
fn transaction_apply_success_commits_target() {
    successful_authoritative_apply_mutates_target();
}

#[test]
fn transaction_apply_validation_failure_rolls_back() {
    failed_validation_does_not_mutate_target();
}

#[test]
fn transaction_apply_stage_failure_rolls_back() {
    let mut target = NonCloneMutationTarget::new(smallvec![3, 3, 3]);
    target.stage_failure = Some(MutationFailureReason::IllegalOperation);
    let initial_digest = target.stable_state_digest();
    let payload = payload_with_writes(
        23,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![8, 8, 8],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(23, 24, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::IllegalOperation)
    ));
    assert_eq!(target.stable_state_digest(), initial_digest);
    assert_eq!(target.component_bytes(ComponentTypeId(1)), Some(&[3, 3, 3][..]));
}

#[test]
fn transaction_apply_dry_run_rolls_back() {
    dry_run_does_not_mutate();
}

#[test]
fn in_memory_target_uses_clone_projection_only_internally() {
    let mut target = seeded_target();
    let transaction = target.begin_apply(apply_context(
        31,
        32,
        MutationApplyMode::Normal,
        MutationConflictPolicy::Reject,
    ));

    assert!(std::any::type_name_of_val(&transaction).contains("CloneProjectionApplyTransaction"));
    let _: CloneProjectionApplyTransaction<'_> = transaction;
}

#[test]
fn receipt_digest_is_identity_complete() {
    let payload = payload_with_writes(
        51,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![4, 5, 6],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );
    let mut committed = seeded_target();
    let mut dry_run = seeded_target();
    let committed_receipt = authoritative_apply(
        &mut committed,
        &payload,
        apply_context(51, 52, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );
    let dry_run_receipt = authoritative_apply(
        &mut dry_run,
        &payload,
        apply_context(51, 52, MutationApplyMode::DryRun, MutationConflictPolicy::Reject),
    );

    assert_eq!(committed_receipt.after_digest, dry_run_receipt.after_digest);
    assert_ne!(committed_receipt.mode, dry_run_receipt.mode);
    assert_ne!(committed_receipt.digest, dry_run_receipt.digest);
}
