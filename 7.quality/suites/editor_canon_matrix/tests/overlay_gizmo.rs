#![allow(
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::manual_is_multiple_of,
    clippy::if_same_then_else,
    clippy::assertions_on_constants,
    dead_code
)]
mod common;
use common::*;
use proptest::prelude::*;
use stratumx_test_support::*;

fn overlay_gizmo_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn overlay_gizmo_all_cases(case in overlay_gizmo_case_strategy()) {
        use common::*;
        let editor = editor();
        assert!(editor.overlay_and_gizmo.gizmos_enabled);
        assert!(editor.overlay_and_gizmo.overlay_labels.len() >= 2);
    }
}
