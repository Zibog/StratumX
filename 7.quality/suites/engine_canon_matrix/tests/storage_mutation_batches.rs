use engine_core::{ComponentTypeId, EngineCoreError, Generation};
use engine_handle::StableEntityHandle;
use engine_identity::EntityId;
use engine_storage_access::{
    AccessDescriptor, AccessMode, ScratchClass, TraversalPlanId, WriteWindow,
};
use engine_storage_layout::LocalityClass;
use engine_storage_mutation::{
    make_apply_payload, queue_deferred_writes, DeferredWrite, FamilyTag, IdempotenceClass,
    MutationBuffer, RegionTag,
};

fn anchor() -> StableEntityHandle {
    StableEntityHandle::new(EntityId {
        slot: 12,
        generation: Generation(1),
    })
}

#[test]
fn idempotent_writes_coalesce_but_non_idempotent_writes_accumulate() {
    let mut buffer = MutationBuffer::new();
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: vec![1, 2].into(),
        idempotence: IdempotenceClass::Idempotent,
    });
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: vec![9, 9].into(),
        idempotence: IdempotenceClass::Idempotent,
    });
    buffer.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: vec![7].into(),
        idempotence: IdempotenceClass::NonIdempotent,
    });

    let change_set = buffer.into_change_set(vec![ComponentTypeId(99)].into());
    assert_eq!(change_set.structural.as_slice(), &[ComponentTypeId(99)]);
    assert_eq!(change_set.writes.len(), 2);
    assert_eq!(change_set.writes[0].bytes.as_slice(), &[9, 9]);
    assert_eq!(change_set.writes[1].bytes.as_slice(), &[7]);
}

#[test]
fn deferred_writes_require_staged_handoff_window() {
    let window = WriteWindow {
        descriptor: AccessDescriptor {
            mode: AccessMode::STAGED,
            plan_id: TraversalPlanId(3),
            locality: LocalityClass::TraversalLane,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: false,
        },
        anchor: anchor(),
    };

    assert_eq!(
        queue_deferred_writes(&window, MutationBuffer::new()).map(|_| ()),
        Err(EngineCoreError::InvalidDescriptor(
            "deferred writes require staged handoff window",
        ))
    );
}

#[test]
fn apply_payload_requires_non_zero_batch_order() {
    let payload = make_apply_payload(
        FamilyTag(4),
        RegionTag(8),
        0,
        MutationBuffer::new().into_change_set(Vec::<ComponentTypeId>::new().into()),
    );
    assert_eq!(
        payload.map(|_| ()),
        Err(EngineCoreError::InvalidDescriptor(
            "batch order must be non-zero",
        ))
    );
}
