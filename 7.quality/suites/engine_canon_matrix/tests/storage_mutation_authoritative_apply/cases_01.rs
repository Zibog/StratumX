#[test]
fn invalid_batch_order_fails_validation() {
    let mut target = seeded_target();
    let target_before = target.deterministic_digest();
    let payload = engine_storage_mutation::ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(2),
        batch_order: 0,
        flags: ApplyFlags::SEGMENTED,
        change_set: engine_storage_mutation::ChangeSet {
            structural: smallvec![],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![1, 2, 3],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(100, 200, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::InvalidBatchOrder)
    ));
    assert_eq!(target_before, target.deterministic_digest());
    assert_eq!(receipt.before_digest, receipt.after_digest);
}

#[test]
fn failed_validation_does_not_mutate_state() {
    failed_validation_does_not_mutate_target();
}

#[test]
fn idempotent_batch_replay_is_stable() {
    idempotent_replay_is_stable();
}

#[test]
fn conflicting_batch_is_rejected_with_exact_reason() {
    conflicting_non_idempotent_write_rejected();
}

#[test]
fn apply_receipt_contains_batch_tick_epoch_region_and_digest() {
    let mut target = seeded_target();
    let payload = payload_with_writes(
        42,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![1, 2, 3],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(
            100,
            200,
            MutationApplyMode::Normal,
            MutationConflictPolicy::LastWriteWins,
        ),
    );

    assert!(receipt.is_success());
    assert_eq!(receipt.batch_id, MutationBatchId(100));
    assert_eq!(receipt.transaction_id, ApplyTransactionId(200));
    assert_eq!(receipt.batch_order, 42);
    assert_eq!(receipt.family_tag, FamilyTag(5));
    assert_eq!(receipt.region_tag, RegionTag(10));
    assert_ne!(receipt.before_digest, StableDigest64::ZERO);
    assert_ne!(receipt.after_digest, StableDigest64::ZERO);
    assert_ne!(receipt.journal_digest, StableDigest64::ZERO);
    assert_ne!(receipt.digest, StableDigest64::ZERO);
}

#[test]
fn empty_batch_is_rejected() {
    let mut target = seeded_target();
    let payload = make_apply_payload(
        FamilyTag(1),
        RegionTag(2),
        100,
        MutationBuffer::new().into_change_set(SmallVec::new()),
    )
    .unwrap();

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(1, 1, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::EmptyChangeset)
    ));
}

#[test]
fn batch_order_is_stable() {
    let payload = payload_with_writes(
        100,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![1],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    assert_eq!(payload.batch_order, 100);
}

#[test]
fn successful_authoritative_apply_mutates_target() {
    let mut target = seeded_target();
    let initial_digest = target.deterministic_digest();
    let payload = payload_with_writes(
        7,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![7, 8, 9],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(7, 8, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(receipt.is_success());
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[7, 8, 9][..])
    );
    assert_ne!(target.deterministic_digest(), initial_digest);
    assert_eq!(receipt.before_digest, initial_digest);
    assert_eq!(receipt.after_digest, target.deterministic_digest());
    assert_ne!(receipt.journal_digest, StableDigest64::ZERO);
}

