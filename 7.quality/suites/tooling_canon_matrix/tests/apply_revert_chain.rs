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

fn apply_revert_chain_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn apply_revert_chain_all_cases(case in apply_revert_chain_case_strategy()) {
        use common::*;
        let (mut runtime, _) = seeded(case);
        let proposal = runtime.stage_proposal(
            format!("goal-{case}"),
            vec![ToolCommand::CreateObject {
                label: format!("assistant-{case}"),
                class: ObjectClass::Logic,
            }],
        );
        runtime.approve_proposal(proposal).unwrap();
        let applied = runtime.apply_proposal(proposal).unwrap();
        let reverted = runtime.revert_proposal(proposal).unwrap();
        assert_eq!(applied.len(), 1);
        assert_eq!(reverted.len(), 1);
    }
}
