use stratumx_tooling_l6_0_authority_core::{
    ErrorClass, RecoveryContext, RecoveryManager, RecoveryStrategy, RecoveryTarget,
};

#[test]
fn recovery_target_creation_preserves_strategy_and_class() {
    let target = RecoveryTarget::new(
        ErrorClass::ResourceNotFound,
        RecoveryStrategy::Skip,
        "Resource not found",
    );
    assert_eq!(target.error_class, ErrorClass::ResourceNotFound);
    assert_eq!(target.strategy, RecoveryStrategy::Skip);
}

#[test]
fn default_recovery_targets_match_expected_strategy() {
    let target = RecoveryTarget::default_for_error_class(ErrorClass::InvalidLifecycleTransition);
    assert_eq!(target.strategy, RecoveryStrategy::Abort);

    let target = RecoveryTarget::default_for_error_class(ErrorClass::PreviewConnectionFailure);
    assert!(matches!(
        target.strategy,
        RecoveryStrategy::RetryAfterDelay(_)
    ));
}

#[test]
fn recovery_context_aborts_after_retry_budget_is_exceeded() {
    let mut context = RecoveryContext::new("initialize", "Failed to initialize")
        .with_error_class(ErrorClass::ExternalDependencyFailure)
        .with_max_retries(2);

    assert_eq!(context.retry_count, 0);
    assert!(!context.is_max_retries_exceeded());

    context.increment_retry();
    context.increment_retry();
    assert!(context.is_max_retries_exceeded());

    let target = context.get_recovery_target();
    assert_eq!(target.strategy, RecoveryStrategy::Abort);
}

#[test]
fn recovery_manager_retries_until_budget_is_exhausted() {
    let mut manager = RecoveryManager::new();
    manager.register_error("init", "timeout", ErrorClass::Timeout);

    assert!(manager.try_retry("init").is_some());
    assert!(manager.try_retry("init").is_some());
    assert!(manager.try_retry("init").is_some());
    assert!(manager.try_retry("init").is_none());
}

#[test]
fn recovery_manager_marks_rollback_when_requested() {
    let mut manager = RecoveryManager::new();
    manager.register_error("init", "corruption", ErrorClass::RegistryCorruption);

    assert!(manager.rollback("init").is_ok());
    let state = manager.get_error_state("init").unwrap();
    assert!(state.requires_rollback);
}

#[test]
fn recovery_manager_clears_and_purges_error_state() {
    let mut manager = RecoveryManager::new();
    manager.register_error("init", "timeout", ErrorClass::Timeout);

    assert!(!manager.is_error_cleared("init"));
    manager.clear_error("init").unwrap();
    assert!(manager.is_error_cleared("init"));

    manager.purge_cleared("init");
    assert!(manager.get_error_state("init").is_none());
}

#[test]
fn recovery_manager_only_requires_rollback_for_destructive_failures() {
    let mut manager = RecoveryManager::new();
    manager.register_error("init", "corruption", ErrorClass::RegistryCorruption);

    let state = manager.get_error_state("init").unwrap();
    assert!(state.requires_rollback);

    manager.register_error("query", "not found", ErrorClass::ResourceNotFound);
    let state = manager.get_error_state("query").unwrap();
    assert!(!state.requires_rollback);
}
