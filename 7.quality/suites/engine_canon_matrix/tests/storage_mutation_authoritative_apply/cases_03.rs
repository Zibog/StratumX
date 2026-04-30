#[test]
fn dry_run_does_not_mutate() {
    let mut target = seeded_target();
    let initial_digest = target.deterministic_digest();
    let payload = payload_with_writes(
        300,
        smallvec![DeferredWrite {
            component: ComponentTypeId(1),
            bytes: smallvec![5, 5, 5],
            idempotence: IdempotenceClass::Idempotent,
        }],
    );

    let receipt = authoritative_apply(
        &mut target,
        &payload,
        apply_context(3, 4, MutationApplyMode::DryRun, MutationConflictPolicy::Reject),
    );

    assert!(receipt.is_success());
    assert_eq!(target.deterministic_digest(), initial_digest);
    assert_eq!(
        target.component_bytes(ComponentTypeId(1)),
        Some(&[9, 9, 9][..])
    );
    assert_eq!(receipt.before_digest, initial_digest);
    assert_ne!(receipt.after_digest, initial_digest);
}
