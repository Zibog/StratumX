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

fn index_rebuild_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn index_rebuild_all_cases(case in index_rebuild_case_strategy()) {
        use common::*;
        let (mut runtime, handle) = seeded(case);
        runtime
            .apply_command(
                ToolCommand::AddTag {
                    handle,
                    tag: "indexed".into(),
                },
                CommandOrigin::User,
                ApprovalClass::None,
                BudgetClass::Interactive,
            )
            .unwrap();
        let index = runtime.index();
        assert!(index.by_tag.get("indexed").unwrap().contains(&handle));
    }
}
