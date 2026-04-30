#[test]
fn partial_failure_does_not_report_fake_success() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 1,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![ComponentTypeId(99)],
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

    let mut target = seeded_target();
    let before = target.deterministic_digest();
    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(9, 9, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::MissingTarget)
    ));
    assert_eq!(target.deterministic_digest(), before);
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[1, 1, 1][..])
    );
    assert_eq!(
        target.component_bytes(ComponentTypeId(2)),
        Some(&[2, 2, 2][..])
    );
}

#[test]
fn last_write_wins_normalizes_idempotent_duplicates() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(1),
        batch_order: 1,
        flags: ApplyFlags::empty(),
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![
                DeferredWrite {
                    component: ComponentTypeId(7),
                    bytes: smallvec![1],
                    idempotence: IdempotenceClass::Idempotent,
                },
                DeferredWrite {
                    component: ComponentTypeId(7),
                    bytes: smallvec![2],
                    idempotence: IdempotenceClass::Idempotent,
                },
            ],
        },
    };

    let plan = MutationApplyPlan::from_payload_with_policy(
        &payload,
        MutationConflictPolicy::LastWriteWins,
        MutationApplyMode::Normal,
    )
    .unwrap();

    assert_eq!(plan.operation_count, 1);
    assert_eq!(plan.deferred_writes()[0].bytes.as_slice(), &[2]);
}

#[test]
fn replay_plan_requires_guard_for_non_idempotent_writes() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(1),
        batch_order: 1,
        flags: ApplyFlags::empty(),
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(7),
                bytes: smallvec![1],
                idempotence: IdempotenceClass::NonIdempotent,
            }],
        },
    };

    let error = MutationApplyPlan::from_payload_with_policy(
        &payload,
        MutationConflictPolicy::Reject,
        MutationApplyMode::Replay,
    )
    .unwrap_err();

    assert_eq!(error, MutationError::ReplayGuardRequired);
}

#[test]
fn zero_batch_order_rejected() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 0,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![1, 2, 3],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    let result = MutationApplyPlan::from_payload(&payload);
    assert_eq!(result.unwrap_err(), MutationError::InvalidBatchOrder);
}
