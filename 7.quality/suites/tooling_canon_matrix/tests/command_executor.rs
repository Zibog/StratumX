//! Command Executor: проверка системы выполнения команд

use stratumx_tooling_l6_0_tool_session::{CommandExecutor, ToolingRuntime};
use stratumx_tooling_l6_12_preview_runtime::initialize_vertical_slice_in_executor;
use stratumx_tooling_l6_1_command_envelopes::{CommandLifecycleState, PromotedCommand};

#[test]
fn execute_command() {
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();
    let command_id = executor.submit_command(PromotedCommand::BuildRun).unwrap();

    let bytes = executor.execute_command(command_id, &mut runtime).unwrap();
    let response: String = serde_json::from_slice(&bytes).unwrap();
    let envelope = executor.get_envelope(command_id).unwrap();

    assert_eq!(response, "executed");
    assert_eq!(envelope.route_id, "route.build.run.v1");
    assert_eq!(envelope.lifecycle_state, CommandLifecycleState::Success);
}

#[test]
fn command_validation() {
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();
    let command_id = executor
        .submit_command(PromotedCommand::SceneBootstrap)
        .unwrap();

    let err = executor
        .execute_command(command_id, &mut runtime)
        .unwrap_err();
    let envelope = executor.get_envelope(command_id).unwrap();

    assert!(err.to_string().contains("No vertical slice session"));
    assert_eq!(
        envelope.lifecycle_state,
        CommandLifecycleState::RetryableFailure
    );
    assert!(envelope
        .error_message
        .as_deref()
        .unwrap_or_default()
        .contains("No vertical slice session"));
}

#[test]
fn command_rollback() {
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();
    initialize_vertical_slice_in_executor(&mut executor).unwrap();
    let command_id = executor
        .submit_command(PromotedCommand::SceneBootstrap)
        .unwrap();

    let bytes = executor.execute_command(command_id, &mut runtime).unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let last_request_id = executor.packet_executor_mut().last_request_id();
    let lifecycle_state = executor.get_envelope(command_id).unwrap().lifecycle_state;

    assert!(payload.get("scene").is_some());
    assert!(payload["scene"].is_object());
    assert_eq!(lifecycle_state, CommandLifecycleState::Success);
    assert_eq!(last_request_id, 1);
}
