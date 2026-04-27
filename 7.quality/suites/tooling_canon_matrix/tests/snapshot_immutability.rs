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

fn snapshot_immutability_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn snapshot_immutability_all_cases(case in snapshot_immutability_case_strategy()) {
        use common::*;
        let (mut runtime, handle) = seeded(case);
        let before = runtime.snapshot();
        runtime
            .apply_command(
                ToolCommand::AddTag {
                    handle,
                    tag: format!("tag-{case}"),
                },
                CommandOrigin::User,
                ApprovalClass::None,
                BudgetClass::Interactive,
            )
            .unwrap();
        let after = runtime.snapshot();
        assert!(after.generation > before.generation);
        assert_eq!(
            before.objects.iter().filter(|o| !o.tags.is_empty()).count(),
            0
        );
    }
}
