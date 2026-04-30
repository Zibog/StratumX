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
