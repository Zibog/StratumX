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

fn workspace_layout_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn workspace_layout_all_cases(case in workspace_layout_case_strategy()) {
        use common::*;
        let mut editor = editor();
        editor.toggle_panel(EditorPanel::Assistant);
        editor.toggle_panel(EditorPanel::Assistant);
        assert!(editor
            .workspace_layout
            .open_panels
            .contains(&EditorPanel::Assistant));
    }
}
