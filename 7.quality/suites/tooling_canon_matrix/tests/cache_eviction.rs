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

fn cache_eviction_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn cache_eviction_all_cases(case in cache_eviction_case_strategy()) {
        use common::*;
        let mut runtime = runtime();
        let handle = runtime.snapshot().objects[0].handle;
        for idx in 0..64 {
            runtime.preview_object(handle).unwrap();
        }
        let cache = runtime.preview_cache();
        assert!(cache.len() <= 32);
        assert_eq!(cache.last().unwrap().handle, handle);
    }
}
