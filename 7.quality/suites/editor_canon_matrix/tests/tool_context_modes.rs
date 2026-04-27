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

fn tool_context_modes_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn tool_context_modes_all_cases(case in tool_context_modes_case_strategy()) {
        use common::*;
        let mut editor = editor();
        let mode = match case % 6 {
            0 => ToolMode::Select,
            1 => ToolMode::Translate,
            2 => ToolMode::Rotate,
            3 => ToolMode::Scale,
            4 => ToolMode::Paint,
            _ => ToolMode::Script,
        };
        editor.set_tool_mode(mode);
        assert_eq!(editor.tool_context.active_mode, mode);
    }
}
