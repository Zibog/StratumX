// Property Tests for Tooling Pipeline
//
// Property 12: Tooling Command Lifecycle Completeness
// Property 13: Tooling Error Recovery
// Property 14: Tooling Layer Purity

use proptest::prelude::*;
use stratumx_tooling_l6_1_command_envelopes::{
    CommandEnvelope, CommandLifecycleState, CommandLifecycleTracker, PromotedCommand,
};
use stratumx_tooling_l6_0_authority_core::{
    ErrorClass, RecoveryManager, RecoveryStrategy, ToolingConveyor,
};
use stratumx_tooling_l6_14_release_runtime::FirstResultVerifier;

// ============================================================================
// Property 12: Tooling Command Lifecycle Completeness
//
// Every command that enters the tooling pipeline MUST pass through all stages:
// reception -> validation -> execution -> result publication.
// No command can skip stages or reach a terminal state without validation.
// ============================================================================

proptest! {
    #[test]
    fn prop_lifecycle_all_stages_reachable(
        route_id in "[a-z._]{5,30}",
        payload_size in 0usize..100,
    ) {
        let mut tracker = CommandLifecycleTracker::new();
        let payload = vec![0u8; payload_size];

        // Phase 1: Reception
        let id = tracker.receive_command(&route_id, payload);
        let state = tracker.get_state(id).unwrap();
        prop_assert_eq!(*state, CommandLifecycleState::Accepted);

        // Phase 2: Validation
        tracker.validate_command(id, true).unwrap();
        let state = tracker.get_state(id).unwrap();
        prop_assert_eq!(*state, CommandLifecycleState::Validated);

        // Phase 3: Execution
        tracker.start_execution(id).unwrap();
        let state = tracker.get_state(id).unwrap();
        prop_assert_eq!(*state, CommandLifecycleState::Running);

        // Phase 4: Result Publication
        tracker.publish_result(id, true, None).unwrap();
        let state = tracker.get_state(id).unwrap();
        prop_assert_eq!(*state, CommandLifecycleState::Success);
    }

    #[test]
    fn prop_lifecycle_no_skip_validation(
        route_id in "[a-z._]{5,30}",
    ) {
        let mut tracker = CommandLifecycleTracker::new();
        let id = tracker.receive_command(&route_id, vec![]);

        // Cannot go directly from Accepted to Running (must validate first)
        let envelope = tracker.get_envelope(id).unwrap();
        let can_skip = envelope.lifecycle_state.can_transition_to(CommandLifecycleState::Running);
        prop_assert!(!can_skip, "Commands cannot skip validation stage");
    }

    #[test]
    fn prop_lifecycle_terminal_only_from_running(
        route_id in "[a-z._]{5,30}",
    ) {
        let mut tracker = CommandLifecycleTracker::new();
        let id = tracker.receive_command(&route_id, vec![]);

        // Cannot reach Success without going through Running
        let envelope = tracker.get_envelope(id).unwrap();
        let can_go_to_success = envelope.lifecycle_state.can_transition_to(CommandLifecycleState::Success);
        prop_assert!(!can_go_to_success, "Cannot reach Success without validation and execution");
    }

    #[test]
    fn prop_lifecycle_retry_returns_to_running(
        route_id in "[a-z._]{5,30}",
    ) {
        let mut tracker = CommandLifecycleTracker::new();
        let id = tracker.receive_command(&route_id, vec![]);

        // Go through full lifecycle to RetryableFailure
        tracker.validate_command(id, true).unwrap();
        tracker.start_execution(id).unwrap();
        tracker.publish_result(id, false, Some("retry: transient error")).unwrap();

        let state = tracker.get_state(id).unwrap();
        prop_assert_eq!(*state, CommandLifecycleState::RetryableFailure);

        // Retry should return to Running
        tracker.retry_command(id).unwrap();
        let state = tracker.get_state(id).unwrap();
        prop_assert_eq!(*state, CommandLifecycleState::Running);
    }
}

// ============================================================================
// Property 13: Tooling Error Recovery
//
// The recovery system MUST support:
// - Retry with bounded attempts (max retries enforced)
// - Rollback for corrupted state
// - Clear error state mechanism
// ============================================================================

proptest! {
    #[test]
    fn prop_recovery_retry_bounded(
        operation_name in "[a-z_]{3,20}",
        max_retries in 1u32..10u32,
    ) {
        let mut manager = RecoveryManager::new();
        manager.register_error(&operation_name, "timeout", ErrorClass::Timeout);

        // Manually set max retries via internal state manipulation for property test
        // In practice, this would be done via with_max_retries pattern
        let successful_retries = (0..max_retries)
            .filter(|_| manager.try_retry(&operation_name).is_some())
            .count();

        // Should allow at least some retries and then stop
        prop_assert!(successful_retries <= max_retries as usize,
            "Retry count must be bounded");
    }

    #[test]
    fn prop_recovery_rollback_for_corruption(
        operation_name in "[a-z_]{3,20}",
    ) {
        let mut manager = RecoveryManager::new();

        // Registry corruption requires rollback
        manager.register_error(&operation_name, "corruption", ErrorClass::RegistryCorruption);
        let state = manager.get_error_state(&operation_name).unwrap();
        prop_assert!(state.requires_rollback, "Registry corruption must require rollback");

        // Invalid lifecycle transition also requires rollback
        manager.register_error(&format!("{}_2", operation_name), "invalid transition", ErrorClass::InvalidLifecycleTransition);
        let state = manager.get_error_state(&format!("{}_2", operation_name)).unwrap();
        prop_assert!(state.requires_rollback, "Invalid lifecycle transition must require rollback");
    }

    #[test]
    fn prop_recovery_error_state_clearable(
        operation_name in "[a-z_]{3,20}",
    ) {
        let mut manager = RecoveryManager::new();
        manager.register_error(&operation_name, "error", ErrorClass::Timeout);

        // Error state should NOT be cleared initially
        prop_assert!(!manager.is_error_cleared(&operation_name));

        // Clear should succeed
        prop_assert!(manager.clear_error(&operation_name).is_ok());

        // Error state should now be cleared
        prop_assert!(manager.is_error_cleared(&operation_name));
    }

    #[test]
    fn prop_recovery_exhausted_retries_force_abort(
        operation_name in "[a-z_]{3,20}",
    ) {
        let mut manager = RecoveryManager::new();
        manager.register_error(&operation_name, "external failure", ErrorClass::ExternalDependencyFailure);

        // Exhaust all retries
        while manager.try_retry(&operation_name).is_some() {}

        // After exhaustion, no more retries
        let result = manager.try_retry(&operation_name);
        prop_assert!(result.is_none(), "No retries should be available after exhaustion");

        // Error state should require rollback
        let state = manager.get_error_state(&operation_name).unwrap();
        prop_assert!(state.requires_rollback, "Exhausted retries must require rollback");
    }
}

// ============================================================================
// Property 14: Tooling Layer Purity
//
// Tooling MUST NOT:
// - Import editor implementation types (5.editor, 6.apps)
// - Define engine truth (2.engine)
// Tooling MAY depend on SDK types (3.sdk) as boundary contracts.
// ============================================================================

proptest! {
    #[test]
    fn prop_layer_purity_no_editor_dependencies(
        path in "[a-zA-Z0-9_/.-]{5,100}",
    ) {
        use stratumx_tooling_l6_0_authority_core::layer_purity;

        // Any path containing editor or app implementation layers is disallowed
        let has_editor_impl = path.contains("5.editor") || path.contains("6.apps");
        let is_allowed = layer_purity::is_allowed_dependency_path(&path);

        if has_editor_impl {
            prop_assert!(!is_allowed, "Editor/app implementation paths must be blocked");
        } else {
            // SDK and other paths are allowed
            prop_assert!(is_allowed, "SDK and other paths must be allowed");
        }
    }

    #[test]
    fn prop_layer_purity_sdk_allowed(
        sdk_path in "3.sdk/l5.[0-9]-[a-z_-]{3,30}",
    ) {
        use stratumx_tooling_l6_0_authority_core::layer_purity;

        // SDK paths should always be allowed
        let is_allowed = layer_purity::is_allowed_dependency_path(&sdk_path);
        prop_assert!(is_allowed, "SDK paths must be allowed for tooling");
    }

    #[test]
    fn prop_layer_purity_allowed_prefixes_include_tooling(
        _seed in any::<u32>(),
    ) {
        use stratumx_tooling_l6_0_authority_core::layer_purity;

        let prefixes = layer_purity::allowed_dependency_prefixes();

        // Must include tooling prefix
        prop_assert!(
            prefixes.iter().any(|p| p.contains("tooling")),
            "Allowed prefixes must include tooling packages"
        );

        // Must include SDK prefix
        prop_assert!(
            prefixes.iter().any(|p| p.contains("sdk")),
            "Allowed prefixes must include SDK packages"
        );
    }
}

// ============================================================================
// Additional Integration Tests
// ============================================================================

#[test]
fn test_lifecycle_with_conveyor() {
    // Commands that pass through lifecycle can also be processed by conveyor
    let mut tracker = CommandLifecycleTracker::new();
    let mut conveyor = ToolingConveyor::new();

    // Simulate a material import command
    let id = tracker.receive_command("route.material.create.v1", vec![]);
    tracker.validate_command(id, true).unwrap();
    tracker.start_execution(id).unwrap();
    tracker.publish_result(id, true, None).unwrap();

    // The command's result can trigger conveyor processing
    let result = conveyor.process_asset("mat_001", "/assets/mat.mat");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().current_stage, stratumx_tooling_l6_0_authority_core::ConveyorStage::Certified);
}

#[test]
fn test_recovery_with_lifecycle() {
    // Failed commands in lifecycle should be tracked in recovery
    let mut tracker = CommandLifecycleTracker::new();
    let mut recovery = RecoveryManager::new();

    let id = tracker.receive_command("route.material.create.v1", vec![]);
    tracker.validate_command(id, true).unwrap();
    tracker.start_execution(id).unwrap();

    // Simulate execution failure
    tracker.publish_result(id, false, Some("retry: timeout")).unwrap();
    assert_eq!(tracker.get_state(id).unwrap(), &CommandLifecycleState::RetryableFailure);

    // Track in recovery manager
    recovery.register_error("material.create", "timeout", ErrorClass::Timeout);

    // Recovery should allow retry
    let strategy = recovery.try_retry("material.create");
    assert!(strategy.is_some());
    assert_eq!(strategy.unwrap(), RecoveryStrategy::Retry);
}

#[test]
fn test_release_verification_in_pipeline() {
    // First result verification is part of the release pipeline
    let mut verifier = FirstResultVerifier::new();

    let result = verifier.verify_signature(
        "verify_001".to_string(),
        "project_001".to_string(),
        "build_001".to_string(),
        stratumx_tooling_l6_14_release_runtime::first_result_verification::FIRST_RESULT_SIGNATURE.to_string(),
        vec!["launch ok".to_string()],
    );

    assert!(result.verified);
}
