//! Authoring Runtime: проверка runtime для authoring

#[test]
fn authoring_runtime_startup() {
    use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

    let session = EditorAuthoringSession::new();

    assert!(
        !session.has_runtime_session(),
        "New session should not have runtime"
    );
}

#[test]
fn authoring_runtime_hot_reload() {
    use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

    let mut session = EditorAuthoringSession::new();
    let before = session.get_world_summary();
    session.initialize_vertical_slice_session().unwrap();
    let after = session.get_world_summary();

    assert_eq!(before.entity_count, 0);
    assert_eq!(after.entity_count, 3);
    assert!(after.active_scene.is_some());
    assert!(session.has_runtime_session());
}

#[test]
fn authoring_runtime_state_sync() {
    use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();
    let first_scene = session.get_world_summary().active_scene.clone();
    session.initialize_vertical_slice_session().unwrap();
    let second_scene = session.get_world_summary().active_scene.clone();

    assert_eq!(first_scene, second_scene);
    assert!(session.has_runtime_session());
}
