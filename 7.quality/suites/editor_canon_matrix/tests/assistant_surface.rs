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

fn assistant_surface_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn assistant_surface_all_cases(case in assistant_surface_case_strategy()) {
        use common::*;
        let mut editor = editor();
        let proposal = editor.stage_assistant_goal(format!("assistant-goal-{case}"));
        editor.approve_and_apply_assistant().unwrap();
        assert_eq!(editor.assistant_surface.staged_proposal, Some(proposal));
        assert!(editor
            .tooling
            .assistant_evidence()
            .iter()
            .any(|e| e.proposal_id == proposal));
    }
}
