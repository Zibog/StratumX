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

fn shell_panels_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn shell_panels_all_cases(case in shell_panels_case_strategy()) {
        use common::*;
        let editor = editor();
        assert!(editor.anchored_panels().contains(&EditorPanel::Viewport));
        assert!(editor
            .anchored_panels()
            .contains(&EditorPanel::BuildRelease));
        assert_eq!(editor.title, "StratumX Editor");
    }
}
