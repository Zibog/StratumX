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

fn build_release_surface_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn build_release_surface_all_cases(case in build_release_surface_case_strategy()) {
        use common::*;
        let mut editor = editor();
        let build = editor.run_build().unwrap();
        let release = editor.run_release(format!("channel-{case}")).unwrap();
        assert_eq!(
            editor
                .build_release_surface
                .last_build
                .as_ref()
                .unwrap()
                .digest,
            build.digest
        );
        assert_eq!(
            editor
                .build_release_surface
                .last_release
                .as_ref()
                .unwrap()
                .build_digest,
            release.build_digest
        );
    }
}
