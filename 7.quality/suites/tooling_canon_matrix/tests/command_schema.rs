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

fn command_schema_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn command_schema_all_cases(case in command_schema_case_strategy()) {
        use common::*;
        let (mut runtime, handle) = seeded(case);
        let label = format!("renamed-{case}");
        runtime
            .apply_command(
                ToolCommand::SetLabel {
                    handle,
                    label: label.clone(),
                },
                CommandOrigin::Automation,
                ApprovalClass::ReviewRequired,
                BudgetClass::Background,
            )
            .unwrap();
        let snapshot = runtime.snapshot();
        assert!(snapshot.objects.iter().any(|object| object.label == label));
        let last = runtime.ledger().last().unwrap();
        assert_eq!(last.origin, CommandOrigin::Automation);
    }
}
