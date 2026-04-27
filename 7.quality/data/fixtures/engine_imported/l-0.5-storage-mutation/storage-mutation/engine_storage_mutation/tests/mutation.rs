use engine_core::ComponentTypeId;
use engine_handle::StableEntityHandle;
use engine_identity::{IdentityAllocator, IdentityDomain};
use engine_storage_access::{
    make_write_window, AccessDescriptor, AccessMode, ScratchClass, TraversalPlanId,
};
use engine_storage_layout::LocalityClass;
use engine_storage_mutation::{
    make_apply_payload, queue_deferred_writes, DeferredWrite, FamilyTag, IdempotenceClass,
    MutationBuffer, RegionTag,
};
use smallvec::{smallvec, SmallVec};

#[test]
fn mutation_buffer_coalesces_idempotent_writes() {
    let mut buffer = MutationBuffer::new();
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(7),
        bytes: smallvec![1, 2],
        idempotence: IdempotenceClass::Idempotent,
    });
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(7),
        bytes: smallvec![9],
        idempotence: IdempotenceClass::Idempotent,
    });

    let set = buffer.into_change_set(SmallVec::new());
    assert_eq!(set.writes.len(), 1);
    assert_eq!(set.writes[0].bytes.as_slice(), &[9]);
}

#[test]
fn deferred_writes_require_staged_window() {
    let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
    let handle = StableEntityHandle::new(alloc.issue_entity().unwrap());
    let descriptor = AccessDescriptor {
        mode: AccessMode::STAGED,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Partition,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };
    let window = make_write_window(descriptor, handle).unwrap();

    let mut buffer = MutationBuffer::new();
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1],
        idempotence: IdempotenceClass::NonIdempotent,
    });

    assert!(queue_deferred_writes(&window, buffer).is_ok());
}

#[test]
fn apply_payload_is_ordered_and_segmented() {
    let payload = make_apply_payload(
        FamilyTag(2),
        RegionTag(3),
        10,
        MutationBuffer::new().into_change_set(smallvec![]),
    )
    .unwrap();

    assert_eq!(payload.batch_order, 10);
    assert_eq!(payload.family_tag.0, 2);
    assert_eq!(payload.region_tag.0, 3);
    assert!(payload
        .flags
        .contains(engine_storage_mutation::ApplyFlags::SEGMENTED));
}
