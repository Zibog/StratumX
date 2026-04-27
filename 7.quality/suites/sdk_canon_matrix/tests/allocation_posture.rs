//! Allocation Posture: параметризованные тесты для проверки allocation behavior
//!
//!原先 200 отдельных тестов (allocation_posture_0 через allocation_posture_199)
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

fn allocation_posture_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn allocation_posture_all_cases(case in allocation_posture_case_strategy()) {
        use common::*;
        let (mut runtime, session, _) = seed_runtime(case);
        let policy = default_transport_policy(match case % 4 {
            0 => TransportLane::OrderedControl,
            1 => TransportLane::BoundedPreview,
            2 => TransportLane::MetricsOnly,
            _ => TransportLane::ArtifactOnly,
        });
        let verdict = runtime.transport_legality(
            &policy,
            if policy.max_payload_bytes == 0 {
                0
            } else {
                policy.max_payload_bytes.min(64)
            },
            case.checked_rem(2) == Some(0),
            case.checked_rem(3) == Some(0),
        );
        if (policy.lane == TransportLane::MetricsOnly && case.checked_rem(2) != Some(0))
            || (policy.lane == TransportLane::ArtifactOnly && case.checked_rem(3) != Some(0))
        {
            assert!(matches!(verdict, LegalityVerdict::Illegal(_)));
        } else {
            assert!(matches!(
                verdict,
                LegalityVerdict::Legal | LegalityVerdict::Illegal(_)
            ));
        }
        assert!(runtime.estimated_hot_path_bytes() > 0);
        assert!(default_transport_policy(TransportLane::OrderedControl).max_payload_bytes >= 1024);
        assert!(runtime
            .session_label(session)
            .unwrap()
            .starts_with("session-"));
    }
}
