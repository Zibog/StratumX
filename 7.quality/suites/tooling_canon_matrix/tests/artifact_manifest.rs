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

fn artifact_manifest_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn artifact_manifest_all_cases(case in artifact_manifest_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let build = runtime.build_current();
        assert_eq!(
            build.manifest.kind,
            stratumx_test_support::ArtifactClass::Build
        );
        assert!(build.manifest.digest.contains("build:"));
        assert!(!build.manifest.invalidation_roots.is_empty());
    }
}
