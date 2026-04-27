use stratumx_tooling_l6_0_tool_session::executors::{
    AudioExecutor, BuildExecutor, CanonicalCommandExecutor, CommandExecutor, EnvironmentExecutor,
    MaterialExecutor, RuntimeExecutor, ShellExecutor, TerrainExecutor, WorldExecutor,
};

#[test]
fn executor_surface_is_publicly_accessible() {
    let exported_types = [
        std::any::type_name::<CanonicalCommandExecutor>(),
        std::any::type_name::<CommandExecutor>(),
        std::any::type_name::<AudioExecutor>(),
        std::any::type_name::<BuildExecutor>(),
        std::any::type_name::<ShellExecutor>(),
        std::any::type_name::<EnvironmentExecutor>(),
        std::any::type_name::<MaterialExecutor>(),
        std::any::type_name::<RuntimeExecutor>(),
        std::any::type_name::<TerrainExecutor>(),
        std::any::type_name::<WorldExecutor>(),
    ];

    assert!(
        exported_types
            .iter()
            .all(|name| name.contains("stratumx_tooling_l6_0_tool_session")),
        "Executor exports should resolve through the tooling session crate",
    );
}
