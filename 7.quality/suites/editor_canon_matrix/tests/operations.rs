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

fn operations_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn operations_all_cases(case in operations_case_strategy()) {
        use common::*;
        let mut editor = editor();
        editor.run_playtest_capture(format!("capture-{case}"));
        let preview = editor.preview_selected().unwrap();
        assert!(editor
            .playtest_surface
            .last_capture_label
            .as_ref()
            .unwrap()
            .contains("capture-"));
        assert!(preview.is_some());
    }
}
