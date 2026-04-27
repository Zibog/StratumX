// HONEST RUNTIME PATH INTEGRATION TEST
// Verifies that scene commands go through ONE real path:
// CommandExecutor -> PacketExecutor -> VerticalSliceSession -> Engine Truth

use stratumx_tooling_l6_0_tool_session::{CommandExecutor, ToolingRuntime};
use stratumx_tooling_l6_12_preview_runtime::initialize_vertical_slice_in_executor;
use stratumx_tooling_l6_1_command_envelopes::{CommandLifecycleState, PromotedCommand};

#[test]
fn scene_commands_require_session() {
    // WITHOUT session: commands fail honestly
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();

    let cmd = PromotedCommand::SceneBootstrap;
    let command_id = executor.submit_command(cmd).expect("submit");

    let result = executor.execute_command(command_id, &mut runtime);
    assert!(result.is_err(), "Should fail without session");

    let state = executor.get_envelope(command_id).unwrap().lifecycle_state;
    assert_eq!(state, CommandLifecycleState::RetryableFailure);
}

#[test]
fn scene_commands_with_session_succeed() {
    // WITH session: commands execute through real runtime
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();

    // Initialize real vertical slice session
    initialize_vertical_slice_in_executor(&mut executor).expect("initialize session");
    assert!(executor.has_vertical_slice_session());

    // SceneBootstrap should now succeed
    let cmd = PromotedCommand::SceneBootstrap;
    let command_id = executor.submit_command(cmd).expect("submit");

    let result = executor.execute_command(command_id, &mut runtime);
    assert!(result.is_ok(), "Should succeed with session: {:?}", result);

    let state = executor.get_envelope(command_id).unwrap().lifecycle_state;
    assert_eq!(state, CommandLifecycleState::Success);
}

#[test]
fn scene_fire_test_shot_with_session() {
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();

    initialize_vertical_slice_in_executor(&mut executor).expect("initialize session");

    // Fire test shot through real runtime
    let cmd = PromotedCommand::SceneFireTestShot {
        weapon_entity_id: 3,
    };
    let command_id = executor.submit_command(cmd).expect("submit");

    let result = executor.execute_command(command_id, &mut runtime);
    assert!(
        result.is_ok(),
        "Fire test shot should succeed: {:?}",
        result
    );

    let state = executor.get_envelope(command_id).unwrap().lifecycle_state;
    assert_eq!(state, CommandLifecycleState::Success);
}

#[test]
fn scene_reset_with_session() {
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();

    initialize_vertical_slice_in_executor(&mut executor).expect("initialize session");

    // Reset scene through real runtime
    let cmd = PromotedCommand::SceneReset;
    let command_id = executor.submit_command(cmd).expect("submit");

    let result = executor.execute_command(command_id, &mut runtime);
    assert!(result.is_ok(), "Scene reset should succeed: {:?}", result);

    let state = executor.get_envelope(command_id).unwrap().lifecycle_state;
    assert_eq!(state, CommandLifecycleState::Success);
}

#[test]
fn full_scene_workflow_through_honest_path() {
    let mut executor = CommandExecutor::new();
    let mut runtime = ToolingRuntime::new();

    // Initialize session once
    initialize_vertical_slice_in_executor(&mut executor).expect("initialize session");

    // Bootstrap -> Fire -> Reset all through same real runtime
    let bootstrap_cmd = executor
        .submit_command(PromotedCommand::SceneBootstrap)
        .expect("submit bootstrap");
    executor
        .execute_command(bootstrap_cmd, &mut runtime)
        .expect("execute bootstrap");
    assert_eq!(
        executor
            .get_envelope(bootstrap_cmd)
            .unwrap()
            .lifecycle_state,
        CommandLifecycleState::Success
    );

    let fire_cmd = executor
        .submit_command(PromotedCommand::SceneFireTestShot {
            weapon_entity_id: 3,
        })
        .expect("submit fire");
    executor
        .execute_command(fire_cmd, &mut runtime)
        .expect("execute fire");
    assert_eq!(
        executor.get_envelope(fire_cmd).unwrap().lifecycle_state,
        CommandLifecycleState::Success
    );

    let reset_cmd = executor
        .submit_command(PromotedCommand::SceneReset)
        .expect("submit reset");
    executor
        .execute_command(reset_cmd, &mut runtime)
        .expect("execute reset");
    assert_eq!(
        executor.get_envelope(reset_cmd).unwrap().lifecycle_state,
        CommandLifecycleState::Success
    );
}
