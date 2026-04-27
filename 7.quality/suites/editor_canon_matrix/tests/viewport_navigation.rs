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

fn viewport_navigation_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn viewport_navigation_all_cases(case in viewport_navigation_case_strategy()) {
        use common::*;
        let mut editor = editor();
        let handle = editor.outliner.items[case % editor.outliner.items.len()].0;
        editor.select_object(handle).unwrap();
        assert_eq!(editor.viewport.selected, vec![handle]);
        assert_eq!(editor.inspector.selected, Some(handle));
    }
}
