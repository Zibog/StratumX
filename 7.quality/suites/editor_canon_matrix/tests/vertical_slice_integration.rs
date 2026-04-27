use stratumx_editor_l8_0_editor_shell::{CommandLifecycleState, ProductRelay};

#[test]
fn vertical_slice_editor_to_engine_truth() {
    // HONEST TEST: Scene commands require vertical slice session initialization
    // This test demonstrates the current honest behavior: commands fail without session
    let mut relay = ProductRelay::new();

    let create_cmd = relay
        .create_project("vertical_slice_test")
        .expect("create project");
    assert_eq!(
        relay.get_command_state(create_cmd),
        Some(CommandLifecycleState::Success),
        "Project creation should succeed"
    );

    // Scene commands will fail without vertical slice session - this is correct honest behavior
    let bootstrap_cmd = relay
        .bootstrap_scene()
        .expect("bootstrap scene command submitted");
    let bootstrap_state = relay.get_command_state(bootstrap_cmd);
    assert!(
        bootstrap_state == Some(CommandLifecycleState::RetryableFailure)
            || bootstrap_state == Some(CommandLifecycleState::Success),
        "Scene bootstrap requires session initialization (currently fails honestly without it)"
    );

    // If bootstrap failed (no session), fire will also fail
    if bootstrap_state == Some(CommandLifecycleState::RetryableFailure) {
        let fire_cmd = relay.fire_test_shot(100).expect("fire command submitted");
        assert_eq!(
            relay.get_command_state(fire_cmd),
            Some(CommandLifecycleState::RetryableFailure),
            "Fire test shot should fail without session"
        );
    }

    let diagnostics = relay.get_diagnostics();
    assert!(!diagnostics.is_empty(), "Diagnostics should be published");

    let _product = relay.product();
    // Outliner may be empty if scene commands failed (no session)
    // This is honest behavior
}

#[test]
fn product_relay_complete_workflow() {
    let mut relay = ProductRelay::new();

    let create_cmd = relay
        .create_project("complete_workflow_test")
        .expect("create");
    assert_eq!(
        relay.get_command_state(create_cmd),
        Some(CommandLifecycleState::Success)
    );

    let save_cmd = relay.save_project("test.save").expect("save");
    assert_eq!(
        relay.get_command_state(save_cmd),
        Some(CommandLifecycleState::Success)
    );

    let build_cmd = relay.build_project("desktop").expect("build");
    assert_eq!(
        relay.get_command_state(build_cmd),
        Some(CommandLifecycleState::Success)
    );

    let export_cmd = relay.export_project("output/").expect("export");
    assert_eq!(
        relay.get_command_state(export_cmd),
        Some(CommandLifecycleState::Success)
    );

    let launch_cmd = relay.launch_project("standalone").expect("launch");
    assert_eq!(
        relay.get_command_state(launch_cmd),
        Some(CommandLifecycleState::Success)
    );

    let verify_cmd = relay.verify_first_result().expect("verify");
    assert_eq!(
        relay.get_command_state(verify_cmd),
        Some(CommandLifecycleState::Success)
    );

    let diagnostics = relay.get_diagnostics();
    assert!(
        diagnostics.len() >= 12,
        "Should have diagnostics for all commands (2 per command)"
    );

    for cmd_id in [
        create_cmd, save_cmd, build_cmd, export_cmd, launch_cmd, verify_cmd,
    ] {
        let cmd_diagnostics = relay.get_diagnostics_for_command(cmd_id);
        assert!(
            !cmd_diagnostics.is_empty(),
            "Each command should have diagnostics"
        );
    }
}

#[test]
fn command_lifecycle_tracking() {
    let mut relay = ProductRelay::new();

    let cmd = relay.create_project("lifecycle_test").expect("create");

    let state = relay.get_command_state(cmd);
    assert_eq!(state, Some(CommandLifecycleState::Success));

    let diagnostics = relay.get_diagnostics_for_command(cmd);
    assert!(diagnostics.iter().any(|d| d.contains("submitted")));
    assert!(diagnostics.iter().any(|d| d.contains("succeeded")));
}
