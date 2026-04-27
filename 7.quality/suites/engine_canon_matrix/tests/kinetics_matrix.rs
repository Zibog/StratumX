#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn kinetics_simulate_case_strategy() -> impl Strategy<Value = usize> {
    0usize..40
}

proptest! {
    #[test]
    fn kinetics_all_cases(|case in kinetics_simulate_case_strategy()) {
        let contact_count = case % 5;
        let projectile_count = case;
        let fam = KineticsFamily::new(KineticsConfig {
            max_contacts: 64,
            max_projectiles: 64,
        });
        let (delta, metrics) = fam
            .simulate(
                &WorldState::new(),
                &materials(),
                KineticsContext {
                    tick: Tick(0),
                    region_key: (case as i32, 0, 0),
                    contact_count,
                    projectile_count,
                },
            )
            .unwrap();
        prop_assert_eq!(metrics.contacts, contact_count);
        prop_assert_eq!(delta.apply_segments[0].family_tags[0], 10);
    }
}
