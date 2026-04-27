// SDK Canon Matrix - smoke test

mod common;
use common::*;

#[test]
fn smoke_test_bridge_runtime() {
    let runtime = runtime();
    assert!(runtime.session_count() > 0);
}

#[test]
fn smoke_test_seed_runtime() {
    let (runtime, session, object) = seed_runtime(0);
    assert!(runtime.session_count() > 0);
    assert!(session.0 > 0);
    assert!(object.0 > 0);
}

#[test]
fn smoke_test_capabilities() {
    let caps = capabilities_for(0);
    assert!(!caps.is_empty());
}
