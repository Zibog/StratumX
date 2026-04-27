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

fn stream_boundedness_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn stream_boundedness_all_cases(case in stream_boundedness_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let handle = runtime.snapshot().objects[0].handle;
        for idx in 0..40 {
            let _ = runtime.preview_object(handle).unwrap();
            runtime.validate_snapshot();
        }
        assert!(runtime.preview_cache().len() <= 32);
        assert!(runtime.validation_history().len() <= 64);
    }
}
