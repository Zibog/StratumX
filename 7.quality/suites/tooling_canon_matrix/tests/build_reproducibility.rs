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

fn build_reproducibility_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn build_reproducibility_all_cases(case in build_reproducibility_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let a = runtime.build_current();
        let b = runtime.build_current();
        assert_eq!(a.digest, b.digest);
        assert_eq!(a.object_count, b.object_count);
    }
}
