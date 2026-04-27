use engine_core::ComponentTypeId;
use engine_handle::StableEntityHandle;
use engine_identity::{IdentityAllocator, IdentityDomain};
use engine_storage_access::{
    make_write_window, AccessDescriptor, AccessMode, ScratchClass, TraversalPlanId,
};
use engine_storage_layout::LocalityClass;
use engine_storage_mutation::{
    make_apply_payload, queue_deferred_writes, DeferredWrite, IdempotenceClass, MutationBuffer,
};
use smallvec::smallvec;

fn write_window(staged_handoff: bool) -> engine_storage_access::WriteWindow {
    let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
    let handle = StableEntityHandle::new(alloc.issue_entity().unwrap());
    make_write_window(
        AccessDescriptor {
            mode: AccessMode::STAGED,
            plan_id: TraversalPlanId(44),
            locality: LocalityClass::Partition,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: staged_handoff,
        },
        handle,
    )
    .unwrap()
}

#[test]
fn non_idempotent_writes_preserve_both_records() {
    let mut buffer = MutationBuffer::new();
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![2],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = buffer.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}

#[test]
fn queue_deferred_writes_rejects_missing_handoff() {
    let buffer = MutationBuffer::new();
    let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
    let handle = StableEntityHandle::new(alloc.issue_entity().unwrap());
    let bad_window = make_write_window(
        AccessDescriptor {
            mode: AccessMode::STAGED,
            plan_id: TraversalPlanId(45),
            locality: LocalityClass::Partition,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: false,
        },
        handle,
    )
    .unwrap();
    assert!(queue_deferred_writes(&bad_window, buffer).is_err());
}

#[test]
fn apply_payload_requires_non_zero_batch_order() {
    let payload = make_apply_payload(
        engine_storage_mutation::FamilyTag(1),
        engine_storage_mutation::RegionTag(1),
        0,
        MutationBuffer::new().into_change_set(smallvec![]),
    );
    assert!(payload.is_err());
    assert!(queue_deferred_writes(&write_window(true), MutationBuffer::new()).is_ok());
}
