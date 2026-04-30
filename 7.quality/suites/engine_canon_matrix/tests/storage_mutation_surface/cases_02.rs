#[test]
fn test_make_apply_payload_preserves_changeset() {
    let cs = ChangeSet {
        structural: smallvec::smallvec![ComponentTypeId(42)],
        writes: smallvec::smallvec![DeferredWrite {
            component: ComponentTypeId(42),
            bytes: smallvec::smallvec![1, 2, 3],
            idempotence: IdempotenceClass::NonIdempotent,
        }],
    };

    let result = make_apply_payload(FamilyTag(1), RegionTag(1), 1, cs.clone());
    assert!(result.is_ok());
    let payload = result.unwrap();
    assert_eq!(payload.change_set.structural.len(), cs.structural.len());
    assert_eq!(payload.change_set.writes.len(), cs.writes.len());
}

// === BitFlags Tests ===

#[test]
fn test_apply_flags_from_bits_valid() {
    let flags = ApplyFlags::from_bits(0b0011);
    assert!(flags.is_some());
}

#[test]
fn test_apply_flags_from_bits_invalid() {
    let flags = ApplyFlags::from_bits(0b1000);
    assert!(flags.is_none());
}

#[test]
fn test_apply_flags_bits_match_constants() {
    assert_eq!(ApplyFlags::SEGMENTED.bits(), 0b0001);
    assert_eq!(ApplyFlags::ALLOW_TOMBSTONE_COMPACTION.bits(), 0b0010);
}
