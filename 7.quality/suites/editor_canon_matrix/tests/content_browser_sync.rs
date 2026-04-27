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

fn content_browser_sync_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn content_browser_sync_all_cases(case in content_browser_sync_case_strategy()) {
        use common::*;
        let mut editor = editor();
        editor.create_material(format!("mat-{case}")).unwrap();
        assert!(editor
            .content_browser
            .visible_assets
            .iter()
            .any(|entry| entry.contains("mat-")));
    }
}
