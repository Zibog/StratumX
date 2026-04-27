#![allow(dead_code)]

use stratumx_test_support::{
    BridgeConfig, BridgeControl, BridgeControlKind, BridgeRuntime, BridgeVersion, Capability,
    ObjectClass, RuntimeHandle,
};

pub fn runtime() -> BridgeRuntime {
    let mut runtime = BridgeRuntime::new(BridgeConfig {
        runtime: RuntimeHandle::new(99),
        version: BridgeVersion::new(1, 2, 0),
        max_queue_depth: 1024,
        max_objects_per_snapshot: 8192,
    });
    let session = runtime.open_session("sdk-test");
    let object = runtime.register_object("root", ObjectClass::World).unwrap();
    runtime
        .apply_control(BridgeControl {
            session,
            sequence: 1,
            object: Some(object),
            kind: BridgeControlKind::SetField {
                key: "family".into(),
                value: "world_scene_family".into(),
            },
        })
        .unwrap();
    runtime
}

pub fn seed_runtime(
    case: usize,
) -> (
    BridgeRuntime,
    stratumx_test_support::SessionHandle,
    stratumx_test_support::ObjectHandle,
) {
    let mut runtime = BridgeRuntime::new(BridgeConfig {
        runtime: RuntimeHandle::new(7),
        version: BridgeVersion::new(1, 0, 0),
        max_queue_depth: 2048,
        max_objects_per_snapshot: 8192,
    });
    let session = runtime.open_session(format!("session-{case}"));
    let object = runtime
        .register_object(
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

    runtime
        .apply_control(BridgeControl {
            session,
            sequence: 1,
            object: Some(object),
            kind: BridgeControlKind::SetField {
                key: "allocation_seed".into(),
                value: format!("case-{case}"),
            },
        })
        .unwrap();

    (runtime, session, object)
}

pub fn capabilities_for(case: usize) -> Vec<Capability> {
    let mut caps = vec![Capability::Snapshots];
    if case.checked_rem(2) == Some(0) {
        caps.push(Capability::Observations);
    }
    if case.checked_rem(3) == Some(0) {
        caps.push(Capability::Metrics);
    }
    if case.checked_rem(5) == Some(0) {
        caps.push(Capability::Controls);
    }
    if case.checked_rem(7) == Some(0) {
        caps.push(Capability::ArtifactRefs);
    }
    caps.sort();
    caps.dedup();
    caps
}
