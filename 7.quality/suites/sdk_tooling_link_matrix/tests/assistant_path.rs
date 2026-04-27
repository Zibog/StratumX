//! Assistant Path: параметризованные тесты SDK-tooling link
//!
//!原先 200 тестов (assistant_path_800 через assistant_path_999)

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
    fn assistant_path_all(case in sdk_tooling_case_strategy()) {
        run_sdk_tooling_case(case);
    }
}
