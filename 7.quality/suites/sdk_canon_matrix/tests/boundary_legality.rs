//! Boundary Legality: параметризованные тесты для проверки boundary legality
//!
//!原先 200 отдельных тестов (boundary_legality_0 через boundary_legality_199)
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

fn boundary_legality_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn boundary_legality_all_cases(case in boundary_legality_case_strategy()) {
        use common::*;
        let runtime = runtime();
        let ordered = default_transport_policy(TransportLane::OrderedControl);
        let preview = default_transport_policy(TransportLane::BoundedPreview);
        assert!(matches!(
            runtime.transport_legality(&ordered, 128, false, false),
            LegalityVerdict::Legal
        ));
        if case.checked_rem(2) == Some(0) {
            assert!(matches!(
                runtime.transport_legality(&preview, 32, false, true),
                LegalityVerdict::Legal
            ));
        } else {
            assert!(matches!(
                runtime.transport_legality(&preview, preview.max_payload_bytes + 1, false, true),
                LegalityVerdict::Illegal(_)
            ));
        }
    }
}
