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

fn transaction_determinism_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn transaction_determinism_all_cases(case in transaction_determinism_case_strategy()) {
        use common::*;
        let (mut runtime, handle) = seeded(case);
        for idx in 0..3 {
            runtime
                .apply_command(
                    ToolCommand::UpsertField {
                        handle,
                        key: format!("k{idx}"),
                        value: format!("v{case}-{idx}"),
                    },
                    CommandOrigin::User,
                    ApprovalClass::None,
                    BudgetClass::Interactive,
                )
                .unwrap();
        }
        let orders: Vec<u64> = runtime.ledger().iter().map(|tx| tx.order).collect();
        assert!(orders.windows(2).all(|w| w[0] < w[1]));
    }
}
