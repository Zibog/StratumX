//! Snapshot Swaps: параметризованные тесты для проверки snapshot swapping
//!
//!原先 200 отдельных тестов (snapshot_swaps_0 через snapshot_swaps_199)
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

fn snapshot_swaps_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn snapshot_swaps_all_cases(case in snapshot_swaps_case_strategy()) {
        use common::*;
        let (mut runtime, session, object) = seed_runtime(case);
        runtime
            .apply_control(BridgeControl {
                session,
                sequence: 1,
                object: Some(object),
                kind: BridgeControlKind::SetField {
                    key: "family".into(),
                    value: format!("family-{case}"),
                },
            })
            .unwrap();
        let a = runtime.publish_snapshot(format!("snap-a-{case}")).unwrap();
        let b = runtime.publish_snapshot(format!("snap-b-{case}")).unwrap();
        assert!(b.epoch > a.epoch);
        assert_eq!(runtime.latest_snapshot().epoch, b.epoch);
    }
}
