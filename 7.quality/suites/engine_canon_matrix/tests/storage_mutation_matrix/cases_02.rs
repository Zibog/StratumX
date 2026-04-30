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
