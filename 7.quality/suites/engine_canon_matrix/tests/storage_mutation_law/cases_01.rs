#[test]
fn valid_mutation_batch_applies_authoritatively() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 42,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![ComponentTypeId(2)],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![9, 9, 9],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    let plan = MutationApplyPlan::from_payload_with_policy(
        &payload,
        MutationConflictPolicy::Reject,
        MutationApplyMode::Normal,
    )
    .unwrap();

    assert_eq!(plan.batch_order, 42);
    assert_eq!(plan.operation_count, 2);

    let mut target = seeded_target();
    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(1, 1, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(receipt.is_success());
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[9, 9, 9][..])
    );
    assert!(!target.has_component(ComponentTypeId(2)));
}

#[test]
fn invalid_handle_mutation_is_rejected_without_state_change() {
    let payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 1,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![ComponentTypeId(99)],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![4, 4, 4],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    let mut target = seeded_target();
    let before = target.deterministic_digest();
    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(2, 2, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        receipt.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::MissingTarget)
    ));
    assert_eq!(before, target.deterministic_digest());
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[1, 1, 1][..])
    );
}

#[test]
fn stale_generation_mutation_is_rejected() {
    let guarded_payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 2,
        flags: ApplyFlags::SEGMENTED | ApplyFlags::REPLAY_GUARDED,
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![5, 5, 5],
                idempotence: IdempotenceClass::NonIdempotent,
            }],
        },
    };

    let drift_payload = ApplyPayload {
        family_tag: FamilyTag(1),
        region_tag: RegionTag(100),
        batch_order: 3,
        flags: ApplyFlags::SEGMENTED,
        change_set: ChangeSet {
            structural: smallvec![],
            writes: smallvec![DeferredWrite {
                component: ComponentTypeId(1),
                bytes: smallvec![8, 8, 8],
                idempotence: IdempotenceClass::Idempotent,
            }],
        },
    };

    let mut target = seeded_target();
    let first = authoritative_apply(
        &mut target,
        &guarded_payload,
        apply_context(3, 3, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );
    assert!(first.is_success());

    let drift = authoritative_apply(
        &mut target,
        &drift_payload,
        apply_context(4, 4, MutationApplyMode::Normal, MutationConflictPolicy::Reject),
    );
    assert!(drift.is_success());

    let replay = authoritative_apply(
        &mut target,
        &guarded_payload,
        apply_context(5, 5, MutationApplyMode::Replay, MutationConflictPolicy::Reject),
    );

    assert!(matches!(
        replay.outcome,
        ApplyOutcome::ValidationFailed(MutationFailureReason::ReplayStateMismatch)
    ));
}

