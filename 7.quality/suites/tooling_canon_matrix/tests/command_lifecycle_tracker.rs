use std::thread;
use std::time::Duration;

use stratumx_tooling_l6_1_command_envelopes::{
    CommandEnvelope, CommandLifecycleState, CommandLifecycleTracker,
};

#[test]
fn lifecycle_state_transitions_follow_expected_edges() {
    assert!(CommandLifecycleState::Accepted.can_transition_to(CommandLifecycleState::Validated));
    assert!(CommandLifecycleState::Validated.can_transition_to(CommandLifecycleState::Running));
    assert!(CommandLifecycleState::Running.can_transition_to(CommandLifecycleState::Success));
    assert!(!CommandLifecycleState::Success.can_transition_to(CommandLifecycleState::Running));
}

#[test]
fn lifecycle_state_terminal_and_retry_flags_match_contract() {
    assert!(CommandLifecycleState::Success.is_terminal());
    assert!(CommandLifecycleState::TerminalFailure.is_terminal());
    assert!(!CommandLifecycleState::Running.is_terminal());
    assert!(CommandLifecycleState::RetryableFailure.can_retry());
    assert!(!CommandLifecycleState::TerminalFailure.can_retry());
}

#[test]
fn command_envelope_creation_sets_initial_state() {
    let envelope = CommandEnvelope::new(1, "test.route", vec![1, 2, 3]);
    assert_eq!(envelope.command_id, 1);
    assert_eq!(envelope.route_id, "test.route");
    assert_eq!(envelope.lifecycle_state, CommandLifecycleState::Accepted);
}

#[test]
fn command_envelope_rejects_invalid_state_transitions() {
    let mut envelope = CommandEnvelope::new(1, "test.route", vec![]);
    assert!(envelope
        .transition_to(CommandLifecycleState::Validated, None)
        .is_ok());
    assert_eq!(envelope.lifecycle_state, CommandLifecycleState::Validated);
    assert!(envelope
        .transition_to(CommandLifecycleState::Success, None)
        .is_err());
}

#[test]
fn command_envelope_terminal_and_age_observations_work() {
    let mut envelope = CommandEnvelope::new(1, "test.route", vec![]);
    assert!(!envelope.is_terminal());

    envelope
        .transition_to(CommandLifecycleState::Validated, None)
        .unwrap();
    envelope
        .transition_to(CommandLifecycleState::Running, None)
        .unwrap();
    envelope
        .transition_to(CommandLifecycleState::Success, None)
        .unwrap();
    assert!(envelope.is_terminal());

    let envelope = CommandEnvelope::new(2, "test.route", vec![]);
    thread::sleep(Duration::from_millis(10));
    assert!(envelope.age_ms() >= 10);
}

#[test]
fn lifecycle_tracker_runs_reception_to_success_flow() {
    let mut tracker = CommandLifecycleTracker::new();

    let id = tracker.receive_command("test.route", vec![1, 2, 3]);
    assert_eq!(
        tracker.get_state(id),
        Some(&CommandLifecycleState::Accepted)
    );

    tracker.validate_command(id, true).unwrap();
    assert_eq!(
        tracker.get_state(id),
        Some(&CommandLifecycleState::Validated)
    );

    tracker.start_execution(id).unwrap();
    assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Running));

    tracker.publish_result(id, true, None).unwrap();
    assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Success));
    assert!(tracker.is_complete(id).unwrap());
}

#[test]
fn lifecycle_tracker_marks_validation_failure_terminal() {
    let mut tracker = CommandLifecycleTracker::new();
    let id = tracker.receive_command("test.route", vec![]);

    tracker.validate_command(id, false).unwrap();
    assert_eq!(
        tracker.get_state(id),
        Some(&CommandLifecycleState::TerminalFailure)
    );
}

#[test]
fn lifecycle_tracker_supports_retryable_failures() {
    let mut tracker = CommandLifecycleTracker::new();
    let id = tracker.receive_command("test.route", vec![]);

    tracker.validate_command(id, true).unwrap();
    tracker.start_execution(id).unwrap();
    tracker
        .publish_result(id, false, Some("retry: transient error".to_string()))
        .unwrap();
    assert_eq!(
        tracker.get_state(id),
        Some(&CommandLifecycleState::RetryableFailure)
    );

    tracker.retry_command(id).unwrap();
    assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Running));

    tracker.publish_result(id, true, None).unwrap();
    assert_eq!(tracker.get_state(id), Some(&CommandLifecycleState::Success));
}
