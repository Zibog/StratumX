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

fn workspace_non_truth_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn workspace_non_truth_all_cases(case in workspace_non_truth_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let snapshot_before = runtime.snapshot();
        runtime.set_workspace_focus(format!("panel-{case}"));
        let snapshot_after = runtime.snapshot();
        assert_eq!(snapshot_before.generation, snapshot_after.generation);
        assert!(runtime.workspace().focused_view.starts_with("panel-"));
    }
}
