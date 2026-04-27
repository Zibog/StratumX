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

fn release_legality_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn release_legality_all_cases(case in release_legality_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let build = runtime.build_current();
        let release = runtime
            .release_build(&build, format!("dev-{case}"), true)
            .unwrap();
        assert!(release.manifest.digest.contains("release:"));
    }
}
