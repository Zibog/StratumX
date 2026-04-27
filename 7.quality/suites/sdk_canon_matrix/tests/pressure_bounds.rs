//! Pressure Bounds: параметризованные тесты для проверки pressure bounds
//!
//!原先 200 отдельных тестов (pressure_bounds_0 через pressure_bounds_199)
//! объединены в параметризованные тесты.
//!
//! Total tests: 200 cases via proptest

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

fn pressure_bounds_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn pressure_bounds_all_cases(case in pressure_bounds_case_strategy()) {
        use common::*;
        let (mut runtime, session, object) = seed_runtime(case);
        let payload_len = 32 + (case % 96);
        for seq in 1..=4 {
            let packet = BridgePacket {
                session,
                sequence: seq as u64,
                lane: PacketLane::Control,
                topic: format!("packet-{case}-{seq}"),
                payload: vec![seq as u8; payload_len],
            };
            runtime.ingest_packet(packet).unwrap();
        }
        runtime
            .apply_control(BridgeControl {
                session,
                sequence: 5,
                object: Some(object),
                kind: BridgeControlKind::SetField {
                    key: format!("key-{case}"),
                    value: format!("value-{case}"),
                },
            })
            .unwrap();
        let snapshot = runtime
            .publish_snapshot(format!("pressure-{case}"))
            .unwrap();
        assert!(snapshot.object_count >= 1);
        assert!(runtime.estimated_hot_path_bytes() < 4096);
    }
}
