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

fn derived_non_authority_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn derived_non_authority_all_cases(case in derived_non_authority_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let snapshot = runtime.snapshot();
        let summary = stratumx_test_support::DerivedSummary::from_snapshot(&snapshot);
        assert!(summary.active_objects >= 1);
        assert_eq!(summary.retired_objects, 0);
    }
}
