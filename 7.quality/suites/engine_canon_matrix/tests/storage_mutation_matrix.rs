mod common;
use common::*;

#[test]
fn storage_mutation_buffer_semantics_0() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_1() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_2() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_3() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_4() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_5() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_6() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_7() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_8() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_9() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_10() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_11() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_12() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_13() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_14() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_15() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_16() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_17() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_18() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_19() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_20() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_21() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_22() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_buffer_semantics_23() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::NonIdempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 2);
}
#[test]
fn storage_mutation_buffer_semantics_24() {
    let mut b = MutationBuffer::new();
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![1, 2, 3],
        idempotence: IdempotenceClass::Idempotent,
    });
    b.stage_write(DeferredWrite {
        component: ComponentTypeId(1),
        bytes: smallvec![4, 5, 6],
        idempotence: IdempotenceClass::Idempotent,
    });
    let set = b.into_change_set(smallvec![]);
    assert_eq!(set.writes.len(), 1);
}
#[test]
fn storage_mutation_apply_payload_batch_order_0() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 1, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_1() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 2, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_2() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 3, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_3() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 4, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_4() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 5, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_5() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 6, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_6() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 7, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_7() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 8, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_8() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 9, c).is_ok());
}
#[test]
fn storage_mutation_apply_payload_batch_order_9() {
    let c = ChangeSet {
        structural: smallvec![],
        writes: smallvec![],
    };
    assert!(make_apply_payload(FamilyTag(1), RegionTag(1), 10, c).is_ok());
}
