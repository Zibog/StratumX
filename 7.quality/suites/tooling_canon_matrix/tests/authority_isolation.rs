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

fn authority_isolation_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn authority_isolation_all_cases(case in authority_isolation_case_strategy()) {
        use common::*;
        let (mut runtime, handle) = seeded(case);
        runtime
            .apply_command(
                ToolCommand::UpsertField {
                    handle,
                    key: "family".into(),
                    value: format!("family-{case}"),
                },
                CommandOrigin::User,
                ApprovalClass::None,
                BudgetClass::Interactive,
            )
            .unwrap();
        let snapshot = runtime.snapshot();
        assert_eq!(snapshot.objects.len(), 1);
        assert_eq!(runtime.workspace().open_views.len(), 3);
    }
}
