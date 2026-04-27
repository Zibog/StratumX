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

fn outliner_sync_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn outliner_sync_all_cases(case in outliner_sync_case_strategy()) {
        use common::*;
        let mut editor = editor();
        let handle = editor.create_world_anchor(format!("world-{case}")).unwrap();
        assert!(editor.outliner.items.iter().any(|(h, _, _)| *h == handle));
    }
}
