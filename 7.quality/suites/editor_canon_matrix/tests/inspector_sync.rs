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

fn inspector_sync_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn inspector_sync_all_cases(case in inspector_sync_case_strategy()) {
        use common::*;
        let mut editor = editor();
        let handle = editor.create_logic_node(format!("logic-{case}")).unwrap();
        editor.select_object(handle).unwrap();
        assert!(editor.inspector.label.as_ref().unwrap().contains("logic-"));
    }
}
