//! Startup Bridge: параметризованные тесты engine-SDK link
//!
//!原先 200 тестов (startup_bridge_600 через startup_bridge_799)

#![allow(unused_imports, unused_mut, unused_variables)]

mod common;
use common::run_engine_sdk_case;

use proptest::prelude::*;

fn engine_sdk_case_strategy() -> impl Strategy<Value = usize> {
    600usize..800
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn startup_bridge_all(case in engine_sdk_case_strategy()) {
        run_engine_sdk_case(case);
    }
}
