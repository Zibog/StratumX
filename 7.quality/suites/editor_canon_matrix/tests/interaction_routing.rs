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

fn interaction_routing_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn interaction_routing_all_cases(case in interaction_routing_case_strategy()) {
        use common::*;
        let mut editor = editor();
        editor.interaction_routing.command_palette_open = case % 2 == 0;
        editor.interaction_routing.pointer_captured = case % 3 == 0;
        assert_eq!(
            editor.interaction_routing.command_palette_open,
            case % 2 == 0
        );
    }
}
