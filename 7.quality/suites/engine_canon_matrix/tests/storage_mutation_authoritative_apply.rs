use engine_core::{ComponentTypeId, StableDigest64};
use engine_storage_mutation::{
    authoritative_apply, make_apply_payload, ApplyFlags, ApplyOutcome, ApplyTransactionId,
    DeferredWrite, FamilyTag, IdempotenceClass, InMemoryMutationApplyTarget, MutationApplyMode,
    MutationBatchId, MutationBuffer, MutationConflictPolicy, MutationFailureReason, RegionTag,
};
use smallvec::{smallvec, SmallVec};

fn seeded_target() -> InMemoryMutationApplyTarget {
    InMemoryMutationApplyTarget::from_seeded_components([
        (ComponentTypeId(1), smallvec![9, 9, 9]),
        (ComponentTypeId(2), smallvec![4, 4, 4]),
    ])
}

fn payload_with_writes(
    batch_order: u64,
    writes: SmallVec<[DeferredWrite; 16]>,
) -> engine_storage_mutation::ApplyPayload {
    make_apply_payload(
        FamilyTag(5),
        RegionTag(10),
        batch_order,
        MutationBuffer::new().into_change_set(SmallVec::new()),
    )
    .map(|mut payload| {
        payload.change_set.writes = writes;
        payload
    })
    .unwrap()
}

include!("storage_mutation_authoritative_apply/cases_01.rs");
include!("storage_mutation_authoritative_apply/cases_02.rs");
include!("storage_mutation_authoritative_apply/cases_03.rs");
