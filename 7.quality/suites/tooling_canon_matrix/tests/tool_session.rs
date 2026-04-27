//! Tool Session: проверка системы сессий инструментов

use stratumx_tooling_l6_0_tool_session::{ObjectClass, ToolingRuntime};

#[test]
fn tool_session_create() {
    let mut runtime = ToolingRuntime::new();
    let transaction_id = runtime.begin_transaction("create-world".to_string());
    let handle = runtime
        .create_object("world-root", ObjectClass::World)
        .unwrap();
    runtime.commit_transaction(transaction_id).unwrap();

    assert_eq!(runtime.objects().len(), 1);
    assert_eq!(runtime.objects().get(&handle).unwrap().label, "world-root");
    assert!(runtime.transactions()[0].committed);
}

#[test]
fn tool_session_persist() {
    let mut runtime = ToolingRuntime::new();
    let transaction_id = runtime.begin_transaction("persist-scene".to_string());
    runtime
        .create_object("scene-root", ObjectClass::Scene)
        .unwrap();
    runtime.commit_transaction(transaction_id).unwrap();

    let bytes = bincode::serialize(&runtime).unwrap();
    let restored: ToolingRuntime = bincode::deserialize(&bytes).unwrap();

    assert_eq!(restored.objects().len(), 1);
    assert_eq!(restored.transaction_count(), 1);
    assert!(restored.transactions()[0].committed);
}

#[test]
fn tool_session_restore() {
    let mut runtime = ToolingRuntime::new();
    let first = runtime
        .create_object("terrain", ObjectClass::Terrain)
        .unwrap();
    let bytes = bincode::serialize(&runtime).unwrap();
    let mut restored: ToolingRuntime = bincode::deserialize(&bytes).unwrap();
    let second = restored
        .create_object("terrain-copy", ObjectClass::Terrain)
        .unwrap();

    assert_eq!(second.0, first.0 + 1);
    assert_eq!(restored.objects().len(), 2);
}
