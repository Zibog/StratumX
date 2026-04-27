#![allow(unused_imports)]
use super::*;
use engine_core::{ComponentTypeId, Generation};
use engine_handle::StableEntityHandle;
use engine_identity::EntityId;
use engine_storage_access::{
    AccessDescriptor, AccessMode, ScratchClass, TraversalPlanId, WriteWindow,
};
use engine_storage_layout::LocalityClass;
use smallvec::smallvec;

#[test]
fn idempotent_stage_overwrites_previous() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![2],
        idempotence: IdempotenceClass::Idempotent,
    });
    assert_eq!(b.into_change_set(smallvec![]).writes.len(), 1);
}
#[test]
fn non_idempotent_stage_keeps_both() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![2],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    assert_eq!(b.into_change_set(smallvec![]).writes.len(), 2);
}
#[test]
fn queue_deferred_writes_requires_handoff() {
    let h = StableEntityHandle::new(EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    });
    let w = WriteWindow {
        descriptor: AccessDescriptor {
            mode: AccessMode::WRITE,
            plan_id: TraversalPlanId(1),
            locality: LocalityClass::Cache,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: false,
        },
        anchor: h,
    };
    assert!(queue_deferred_writes(&w, MutationBuffer::new()).is_err());
}
#[test]
fn apply_payload_requires_non_zero_batch_order() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 0, c).is_err());
}
#[test]
fn apply_payload_defaults_to_segmented_flag() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    let p = make_apply_payload(FamilyTag(1), RegionTag(1), 1, c).unwrap();
    assert!(p.flags.contains(ApplyFlags::SEGMENTED));
}
