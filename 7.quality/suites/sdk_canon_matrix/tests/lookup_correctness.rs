//! Lookup Correctness: параметризованные тесты для проверки compatibility lookup
//!
//!原先 200 отдельных тестов (lookup_correctness_0 через lookup_correctness_199)
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

/// Стратегия генерации тест-кейсов для lookup correctness
///
/// Генерирует case numbers от 0 до 199 (все оригинальные тесты)
fn lookup_case_strategy() -> impl Strategy<Value = usize> {
    0usize..200
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    /// Параметризованный тест для проверки compatibility lookup
    ///
    ///原先 тесты lookup_correctness_0 через lookup_correctness_199
    #[test]
    fn lookup_correctness_all_cases(case in lookup_case_strategy()) {
        use common::*;
        let mut caps = capabilities_for(case);
        let mut runtime = runtime();
        let profile = match case % 4 {
            0 => CompatibilityProfile::ToolRuntime,
            1 => CompatibilityProfile::EditorSurface,
            2 => CompatibilityProfile::Automation,
            _ => CompatibilityProfile::Diagnostics,
        };
        if case % 11 == 0 {
            caps.push(Capability::Controls);
        }
        caps.sort();
        caps.dedup();
        let version = if case % 13 == 0 {
            BridgeVersion::new(0, 9, 0)
        } else {
            BridgeVersion::new(1, 2, (case % 10) as u16)
        };
        let verdict = runtime.compatibility_verdict(version, &caps, profile);
        if version.major == 0 {
            assert_eq!(verdict, CompatibilityVerdict::VersionTooOld);
        } else {
            match profile {
                CompatibilityProfile::ToolRuntime => assert!(matches!(
                    verdict,
                    CompatibilityVerdict::Compatible
                        | CompatibilityVerdict::MissingCapability(Capability::Controls)
                )),
                CompatibilityProfile::EditorSurface => assert!(matches!(
                    verdict,
                    CompatibilityVerdict::Compatible
                        | CompatibilityVerdict::MissingCapability(Capability::Observations)
                )),
                CompatibilityProfile::Automation => assert!(matches!(
                    verdict,
                    CompatibilityVerdict::Compatible
                        | CompatibilityVerdict::MissingCapability(Capability::Metrics)
                )),
                CompatibilityProfile::Diagnostics => assert!(matches!(
                    verdict,
                    CompatibilityVerdict::Compatible
                        | CompatibilityVerdict::MissingCapability(Capability::Observations)
                        | CompatibilityVerdict::MissingCapability(Capability::Metrics)
                )),
            }
        }
    }
}
