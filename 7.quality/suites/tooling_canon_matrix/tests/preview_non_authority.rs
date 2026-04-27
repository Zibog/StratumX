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

fn preview_non_authority_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn preview_non_authority_all_cases(case in preview_non_authority_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let handle = runtime.snapshot().objects[0].handle;
        let preview = runtime.preview_object(handle).unwrap();
        let snapshot = runtime.snapshot();
        assert!(preview.digest.contains(':'));
        assert_eq!(snapshot.objects.len(), 1);
    }
}
