//! Field Invariants: параметризованные тесты для проверки field invariants
//!
//!原先 200 отдельных тестов (field_invariants_0 через field_invariants_199)
//! объединены в параметризованные тесты для лучшей поддерживаемости.
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

/// Стратегия генерации тест-кейсов для field invariants
fn field_invariant_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    /// Параметризованный тест для проверки field invariants
    ///
    #[test]
    fn field_invariants_all_cases(case in field_invariant_case_strategy()) {
        use common::*;
        let (mut runtime, session, object) = seed_runtime(case);
        runtime
            .apply_control(BridgeControl {
                session,
                sequence: 1,
                object: Some(object),
                kind: BridgeControlKind::SetField {
                    key: format!("slot-{case}"),
                    value: format!("value-{case}"),
                },
            })
            .unwrap();
        runtime
            .apply_control(BridgeControl {
                session,
                sequence: 2,
                object: Some(object),
                kind: BridgeControlKind::AddTag {
                    tag: format!("tag-{case}"),
                },
            })
            .unwrap();
        let snapshot = runtime.publish_snapshot(format!("field-{case}")).unwrap();
        let object = snapshot
            .objects
            .iter()
            .find(|entry| entry.handle == object)
            .unwrap();
        assert!(object.fields.contains_key(&format!("slot-{case}")));
        assert!(object.tags.contains(&format!("tag-{case}")));
        assert_eq!(
            object.state_ref.class,
            stratumx_test_support::StateClass::Snapshot
        );
    }
}
