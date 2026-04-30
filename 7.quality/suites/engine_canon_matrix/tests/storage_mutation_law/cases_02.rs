#[test]
fn conflicting_mutations_are_detected_deterministically() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 1,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![ComponentTypeId(1)],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![1, 2, 3],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    assert!(has_conflicting_operations(&payload));

    let mut target = seeded_target();
    let receipt = authoritative_apply(
        &mut target,
        &payload,
        MutationBatchId(6),
        ApplyTransactionId(6),
        MutationApplyMode::Normal,
        MutationConflictPolicy::Reject,
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::ConflictingOperation)
    ));
}

#[test]
fn duplicate_operations_detected() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 1,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![
                DeferredWrite {
                    component: ComponentTypeId(1),
                    bytes: smallvec![1, 2, 3],
                    idempotence: IdempotenceClass::Idempotent,
                },
                DeferredWrite {
                    component: ComponentTypeId(1),
                    bytes: smallvec![4, 5, 6],
                    idempotence: IdempotenceClass::Idempotent,
                },
            ],
        },
    };

    assert!(has_duplicate_operations(&payload));

    let mut target = seeded_target();
    let receipt = authoritative_apply(
        &mut target,
        &payload,
        MutationBatchId(7),
        ApplyTransactionId(7),
        MutationApplyMode::Normal,
        MutationConflictPolicy::Reject,
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::DuplicateOperation)
    ));
}

#[test]
fn mutation_apply_order_is_stable() {
    let payload1 = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 1,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![
                DeferredWrite {
                    component: ComponentTypeId(1),
                    bytes: smallvec![1, 2, 3],
                    idempotence: IdempotenceClass::Idempotent,
                },
                DeferredWrite {
                    component: ComponentTypeId(2),
                    bytes: smallvec![4, 5, 6],
                    idempotence: IdempotenceClass::Idempotent,
                },
            ],
        },
    };

    let payload2 = payload1.clone();

    let plan1 = MutationApplyPlan::from_payload_with_policy(
        &payload1,
        MutationConflictPolicy::Reject,
        MutationApplyMode::Normal,
    )
    .unwrap();
    let plan2 = MutationApplyPlan::from_payload_with_policy(
        &payload2,
        MutationConflictPolicy::Reject,
        MutationApplyMode::Normal,
    )
    .unwrap();

    assert_eq!(plan1.batch_order, plan2.batch_order);
    assert_eq!(plan1.operation_count, plan2.operation_count);
    assert_eq!(plan1.journal_digest(), plan2.journal_digest());

    let mut left = seeded_target();
    let mut right = seeded_target();
    let left_receipt = authoritative_apply(
        &mut left,
        &payload1,
        MutationBatchId(8),
        ApplyTransactionId(8),
        MutationApplyMode::Normal,
        MutationConflictPolicy::Reject,
    );
    let right_receipt = authoritative_apply(
        &mut right,
        &payload2,
        MutationBatchId(8),
        ApplyTransactionId(9),
        MutationApplyMode::Normal,
        MutationConflictPolicy::Reject,
    );

    assert_eq!(left_receipt.after_digest, right_receipt.after_digest);
    assert_eq!(left_receipt.journal_digest, right_receipt.journal_digest);
    assert_ne!(left_receipt.transaction_id, right_receipt.transaction_id);
    assert_ne!(left_receipt.digest, right_receipt.digest);
    assert_eq!(left.deterministic_digest(), right.deterministic_digest());
}

#[test]
fn empty_batch_has_explicit_result() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 1,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![],
        },
    };

    let result = MutationApplyPlan::from_payload(&payload);
    assert_eq!(result.unwrap_err(), MutationError::EmptyChangeset);
}

