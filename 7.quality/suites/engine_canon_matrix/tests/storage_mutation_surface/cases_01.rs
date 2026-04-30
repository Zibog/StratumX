#[test]
fn test_idempotence_class_equality() {
    assert_eq!(IdempotenceClass::Idempotent, IdempotenceClass::Idempotent);
    assert_eq!(
        IdempotenceClass::NonIdempotent,
        IdempotenceClass::NonIdempotent
    );
    assert_ne!(
        IdempotenceClass::Idempotent,
        IdempotenceClass::NonIdempotent
    );
}

// === Tag Tests ===

#[test]
fn test_family_tag_creation() {
    let tag = FamilyTag(42);
    assert_eq!(tag.0, 42);
}

#[test]
fn test_region_tag_creation() {
    let tag = RegionTag(12345);
    assert_eq!(tag.0, 12345);
}

#[test]
fn test_tag_equality() {
    assert_eq!(FamilyTag(1), FamilyTag(1));
    assert_ne!(FamilyTag(1), FamilyTag(2));
}

// === DeferredWrite Tests ===

#[test]
fn test_deferred_write_creation() {
    let write = DeferredWrite {
        component: ComponentTypeId(100),
        bytes: smallvec::smallvec![1, 2, 3, 4],
        idempotence: IdempotenceClass::Idempotent,
    };
    assert_eq!(write.component, ComponentTypeId(100));
    assert_eq!(write.bytes.len(), 4);
}

#[test]
fn test_deferred_write_equality() {
    let a = DeferredWrite {
        component: ComponentTypeId(100),
        bytes: smallvec::smallvec![1, 2],
        idempotence: IdempotenceClass::Idempotent,
    };
    let b = DeferredWrite {
        component: ComponentTypeId(100),
        bytes: smallvec::smallvec![1, 2],
        idempotence: IdempotenceClass::Idempotent,
    };
    let c = DeferredWrite {
        component: ComponentTypeId(200),
        bytes: smallvec::smallvec![1, 2],
        idempotence: IdempotenceClass::Idempotent,
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// === MutationBuffer Tests ===

#[test]
fn test_mutation_buffer_default_is_empty() {
    let buffer = MutationBuffer::default();
    // Can't access private writes field, but default() should not panic
    drop(buffer);
}

// === ChangeSet Tests ===

#[test]
fn test_changeset_creation() {
    let cs = ChangeSet {
        structural: smallvec::smallvec![],
        writes: smallvec::smallvec![],
    };
    assert!(cs.structural.is_empty());
    assert!(cs.writes.is_empty());
}

#[test]
fn test_changeset_with_data() {
    let cs = ChangeSet {
        structural: smallvec::smallvec![ComponentTypeId(100), ComponentTypeId(200)],
        writes: smallvec::smallvec![DeferredWrite {
            component: ComponentTypeId(100),
            bytes: smallvec::smallvec![1],
            idempotence: IdempotenceClass::Idempotent,
        }],
    };
    assert_eq!(cs.structural.len(), 2);
    assert_eq!(cs.writes.len(), 1);
}

// === ApplyFlags Tests ===

#[test]
fn test_apply_flags_segmented() {
    let flags = ApplyFlags::SEGMENTED;
    assert!(flags.contains(ApplyFlags::SEGMENTED));
    assert!(!flags.contains(ApplyFlags::ALLOW_TOMBSTONE_COMPACTION));
}

#[test]
fn test_apply_flags_tombstone() {
    let flags = ApplyFlags::ALLOW_TOMBSTONE_COMPACTION;
    assert!(!flags.contains(ApplyFlags::SEGMENTED));
    assert!(flags.contains(ApplyFlags::ALLOW_TOMBSTONE_COMPACTION));
}

#[test]
fn test_apply_flags_both() {
    let flags = ApplyFlags::SEGMENTED | ApplyFlags::ALLOW_TOMBSTONE_COMPACTION;
    assert!(flags.contains(ApplyFlags::SEGMENTED));
    assert!(flags.contains(ApplyFlags::ALLOW_TOMBSTONE_COMPACTION));
    assert_eq!(flags.bits(), 0b0011);
}

#[test]
fn test_apply_flags_empty() {
    let flags = ApplyFlags::empty();
    assert!(flags.is_empty());
}

// === ApplyPayload Tests ===

#[test]
fn test_apply_payload_creation() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(42),
        batch_order: 100,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec::smallvec![],
            writes: smallvec::smallvec![],
        },
    };
    assert_eq!(payload.family_tag.0, 1);
    assert_eq!(payload.region_tag.0, 42);
    assert_eq!(payload.batch_order, 100);
}

// === Factory Function Tests ===

#[test]
fn test_make_apply_payload_returns_ok() {
    let result = make_apply_payload(
        FamilyTag(5),
        RegionTag(10),
        1, // batch_order must be non-zero
        ChangeSet {
            structural: smallvec::smallvec![ComponentTypeId(1)],
            writes: smallvec::smallvec![],
        },
    );
    assert!(result.is_ok());
    let payload = result.unwrap();
    assert_eq!(payload.family_tag.0, 5);
    assert_eq!(payload.region_tag.0, 10);
    assert_eq!(payload.batch_order, 1);
}

