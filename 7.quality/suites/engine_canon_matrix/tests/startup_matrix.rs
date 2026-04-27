#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn startup_validate_case_strategy() -> impl Strategy<Value = usize> {
    0usize..35
}

fn startup_launch_headless_case_strategy() -> impl Strategy<Value = usize> {
    0usize..25
}

fn startup_launch_realtime_case_strategy() -> impl Strategy<Value = usize> {
    0usize..20
}

proptest! {
    #[test]
    fn startup_all_validate_cases(|case in startup_validate_case_strategy()) {
        let _case = case;
        let s = StartupAssembly::new(StartupConfig {
            profile: RuntimeProfile::Headless20,
            network_role: NetworkRole::HeadlessHost,
            runtime_manifests: vec![],
            service_wiring: startup_wiring(),
        });
        prop_assert!(s.validate().accepted);
    }

    #[test]
    fn startup_all_launch_headless_cases(|case in startup_launch_headless_case_strategy()) {
        let _case = case;
        let s = StartupAssembly::new(StartupConfig {
            profile: RuntimeProfile::Headless20,
            network_role: NetworkRole::HeadlessHost,
            runtime_manifests: vec![],
            service_wiring: startup_wiring(),
        });
        prop_assert!(s.launch_headless(WorldState::new()).is_ok());
    }

    #[test]
    fn startup_all_launch_realtime_cases(|case in startup_launch_realtime_case_strategy()) {
        let _case = case;
        let s = StartupAssembly::new(StartupConfig {
            profile: RuntimeProfile::Interactive60,
            network_role: NetworkRole::LocalOnly,
            runtime_manifests: vec![],
            service_wiring: startup_wiring(),
        });
        prop_assert!(s.launch_realtime(WorldState::new()).is_ok());
    }
}
