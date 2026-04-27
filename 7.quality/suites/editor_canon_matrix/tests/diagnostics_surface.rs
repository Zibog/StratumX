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

fn diagnostics_surface_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn diagnostics_surface_all_cases(case in diagnostics_surface_case_strategy()) {
        use common::*;
        let editor = editor();
        assert_eq!(
            editor
                .production_surface
                .counters
                .get("diagnostics")
                .copied()
                .unwrap_or_default(),
            editor.diagnostics_surface.diagnostics.len()
        );
        assert!(editor.build_suite.validation_runs >= 1);
    }
}
