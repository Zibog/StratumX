//! Opacity Preservation: параметризованные тесты для проверки object opacity
//!
//!原先 200 отдельных тестов (opacity_preservation_0 через opacity_preservation_199)
//! объединены в параметризованные тесты.

#![allow(
    clippy::manual_is_multiple_of,
    clippy::if_same_then_else,
    clippy::assertions_on_constants,
    clippy::len_zero
)]
#![allow(unused_imports, unused_mut, unused_variables)]

use proptest::prelude::*;

mod common;
use common::*;
use stratumx_test_support::*;

fn opacity_preservation_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn opacity_preservation_all_cases(case in opacity_preservation_case_strategy()) {
        use common::*;
        let (mut runtime, _session, object) = seed_runtime(case);
        let view = runtime.object_view(object).unwrap();
        let debug = format!("{:?}", view.handle);
        assert!(debug.contains("<opaque:"));
        assert!(!debug.contains("object-"));
        assert_eq!(view.identity_ref.tag, view.handle.opaque_tag());
    }
}
