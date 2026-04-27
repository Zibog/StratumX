#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::manual_is_multiple_of
)]
use stratumx_test_support::{
    ApprovalClass, BudgetClass, CommandOrigin, ObjectClass, ToolCommand, ToolingRuntime,
};

pub fn runtime() -> ToolingRuntime {
    let mut runtime = ToolingRuntime::new();
    let handle = runtime.create_object("root", ObjectClass::World).unwrap();
    runtime
        .apply_command(
            ToolCommand::UpsertField {
                handle,
                key: "family".into(),
                value: "world_scene_family".into(),
            },
            CommandOrigin::User,
            ApprovalClass::None,
            BudgetClass::Interactive,
        )
        .unwrap();
    runtime
}

pub fn seeded_invalid_missing_family(
    case: usize,
) -> (ToolingRuntime, stratumx_test_support::ObjectHandle) {
    let mut runtime = ToolingRuntime::new();
    let handle = runtime
        .create_object(
            format!("object-{case}"),
            match case % 5 {
                0 => ObjectClass::World,
                1 => ObjectClass::Scene,
                2 => ObjectClass::Material,
                3 => ObjectClass::Terrain,
                _ => ObjectClass::Logic,
            },
        )
        .unwrap();
    (runtime, handle)
}

pub fn invalid_empty_workspace() -> ToolingRuntime {
    let mut runtime = ToolingRuntime::new();
    runtime.clear_workspace();
    runtime
}

pub fn invalid_empty_label(case: usize) -> (ToolingRuntime, stratumx_test_support::ObjectHandle) {
    let mut runtime = ToolingRuntime::new();
    let handle = runtime
        .create_object(
            if case % 2 == 0 { "" } else { "   " },
            match case % 5 {
                0 => ObjectClass::World,
                1 => ObjectClass::Scene,
                2 => ObjectClass::Material,
                3 => ObjectClass::Terrain,
                _ => ObjectClass::Logic,
            },
        )
        .unwrap();
    (runtime, handle)
}

pub fn seeded(case: usize) -> (ToolingRuntime, stratumx_test_support::ObjectHandle) {
    let mut runtime = ToolingRuntime::new();
    let handle = runtime
        .create_object(
            format!("seeded-{case}"),
            match case % 5 {
                0 => ObjectClass::World,
                1 => ObjectClass::Scene,
                2 => ObjectClass::Material,
                3 => ObjectClass::Terrain,
                _ => ObjectClass::Logic,
            },
        )
        .unwrap();
    runtime
        .apply_command(
            ToolCommand::UpsertField {
                handle,
                key: "family".into(),
                value: "seeded_family".into(),
            },
            CommandOrigin::User,
            ApprovalClass::None,
            BudgetClass::Interactive,
        )
        .unwrap();
    (runtime, handle)
}
