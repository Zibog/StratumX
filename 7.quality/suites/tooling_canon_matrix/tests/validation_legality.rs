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

fn validation_legality_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn validation_legality_all_cases(case in validation_legality_case_strategy()) {
        use common::*;
        if case % 3 == 0 {
            let mut runtime = invalid_empty_workspace();
            let diagnostics = runtime.validate_snapshot();
            assert!(!diagnostics.is_empty());
            assert!(diagnostics
                .iter()
                .any(|d| d.message.contains("workspace is empty")));
        } else if case % 3 == 1 {
            let (mut runtime, handle) = seeded_invalid_missing_family(case);
            let diagnostics = runtime.validate_snapshot();
            assert!(!diagnostics.is_empty());
            assert!(diagnostics.iter().any(|d| d.handle == Some(handle)));
            assert!(diagnostics.iter().any(|d| d.message.contains("family")));
        } else {
            let (mut runtime, handle) = invalid_empty_label(case);
            let diagnostics = runtime.validate_snapshot();
            assert!(!diagnostics.is_empty());
            assert!(diagnostics.iter().any(|d| d.handle == Some(handle)));
            assert!(diagnostics.iter().any(|d| d.message.contains("label")));
        }
    }
}
