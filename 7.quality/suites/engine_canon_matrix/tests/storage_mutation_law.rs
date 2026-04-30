// Storage Mutation Law Tests
//
// Real authoritative apply behavior for L-0.5 storage mutation:
// - validation before mutation
// - atomic apply / no partial mutation on failure
// - conflict policy enforcement
// - replay guard semantics
// - deterministic journal and receipt digests

use engine_core::ComponentTypeId;
use engine_storage_mutation::{
    authoritative_apply, has_conflicting_operations, has_duplicate_operations, ApplyContext,
    ApplyFlags, ApplyOutcome, ApplyPayload, ChangeSet, DeferredWrite, FamilyTag, IdempotenceClass,
    InMemoryMutationApplyTarget, MutationApplyMode, MutationApplyPlan, MutationConflictPolicy,
    MutationError, MutationFailureReason, RegionTag,
};
use smallvec::smallvec;

fn seeded_target() -> InMemoryMutationApplyTarget {
    InMemoryMutationApplyTarget::from_seeded_components([
        (ComponentTypeId(1), smallvec![1, 1, 1]),
        (ComponentTypeId(2), smallvec![2, 2, 2]),
    ])
}

fn apply_context(
    batch_id: u64,
    transaction_id: u64,
    mode: MutationApplyMode,
    conflict_policy: MutationConflictPolicy,
) -> ApplyContext {
    ApplyContext {
        batch_id: engine_storage_mutation::MutationBatchId(batch_id),
        transaction_id: engine_storage_mutation::ApplyTransactionId(transaction_id),
        mode,
        conflict_policy,
    }
}

include!("storage_mutation_law/cases_01.rs");
include!("storage_mutation_law/cases_02.rs");
include!("storage_mutation_law/cases_03.rs");
