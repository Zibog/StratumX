//! Preview Build Release: параметризованные тесты SDK-tooling link
//!
//!原先 200 тестов (preview_build_release_800 через preview_build_release_999)

#![allow(unused_imports, unused_mut, unused_variables)]

mod common;
use common::run_sdk_tooling_case;

use proptest::prelude::*;

fn sdk_tooling_case_strategy() -> impl Strategy<Value = usize> {
    800usize..1000
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn preview_build_release_all(case in sdk_tooling_case_strategy()) {
        run_sdk_tooling_case(case);
    }
}
