#[test]
fn failed_validation_does_not_mutate_target() {
    let mut target = seeded_target();
    let initial_digest = target.deterministic_digest();
    let payload = engine_storage_mutation::ApplyPayload {
        family_tag: FamilyTag(5),
        region_tag: RegionTag(10),
        batch_order: 0,
        flags: ApplyFlags::SEGMENTED,
        change_set: engine_storage_mutation::ChangeSet {
            structural: smallvec![],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![3, 2, 1],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(9, 9, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::InvalidBatchOrder)
    ));
    assert_eq!(target.deterministic_digest(), initial_digest);
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[9, 9, 9][..])
    );
    assert_eq!(receipt.before_digest, initial_digest);
    assert_eq!(receipt.after_digest, initial_digest);
}

#[test]
fn conflicting_non_idempotent_write_rejected() {
    let mut target = seeded_target();
    let initial_digest = target.deterministic_digest();
    let payload = payload_with_writes(
        100,
        smallvec![
            DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![1],
                idempotence: IdempotenceClass::NonIdempotent,
            },
            DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![2],
                idempotence: IdempotenceClass::NonIdempotent,
            },
        ],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(
            10,
            11,
            MutationApplyMode::Normal,
            MutationConflictPolicy::LastWriteWins,
        ),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::ConflictingOperation)
    ));
    assert_eq!(target.deterministic_digest(), initial_digest);
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[9, 9, 9][..])
    );
}

#[test]
fn idempotent_replay_is_stable() {
    let mut target = InMemoryMutationApplyTarget::new();
    let payload = payload_with_writes(
        100,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![1, 2, 3],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    let receipt1 = authoritative_apply(
        &mut target,
        &payload,
        apply_context(1, 1, MutationApplyMode::Normal, MutationConflictPolicy::LastWriteWins),
    );
    let digest_after_first = target.deterministic_digest();

    let receipt2 = authoritative_apply(
        &mut target,
        &payload,
        apply_context(1, 2, MutationApplyMode::Replay, MutationConflictPolicy::LastWriteWins),
    );

    assert!(receipt1.is_success());
    assert!(receipt2.is_success());
    assert_eq!(digest_after_first, target.deterministic_digest());
    assert_eq!(receipt1.after_digest, receipt2.after_digest);
    assert_eq!(receipt1.journal_digest, receipt2.journal_digest);
    assert_ne!(receipt1.transaction_id, receipt2.transaction_id);
    assert_ne!(receipt1.digest, receipt2.digest);
}

#[test]
fn non_idempotent_replay_without_guard_rejected() {
    let mut target = seeded_target();
    let payload = payload_with_writes(
        200,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![7],
            idempotence: IdempotenceClass::NonIdempotent,
        }],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(2, 3, MutationApplyMode::Replay, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::ReplayGuardRequired)
    ));
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[9, 9, 9][..])
    );
}

