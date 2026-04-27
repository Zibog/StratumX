#![allow(unused_imports)]
use super::*;

fn wiring() -> ServiceWiring {
    ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    }
}
#[test]
fn validate_rejects_interactive_role_on_headless_profile() {
    let s = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: wiring(),
    });
    assert!(!s.validate().accepted);
}
#[test]
fn runtime_launch_plan_counts_packs() {
    let s = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: wiring(),
    });
    assert_eq!(s.runtime_launch_plan().unwrap().runtime_pack_count, 0);
}
#[test]
fn launch_headless_requires_headless_profile() {
    let s = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: wiring(),
    });
    assert!(s.launch_headless(engine_world::WorldState::new()).is_err());
}
#[test]
fn launch_headless_accepts_headless_profile() {
    let s = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: wiring(),
    });
    assert!(s.launch_headless(engine_world::WorldState::new()).is_ok());
}
#[test]
fn launch_realtime_rejects_headless_profile() {
    let s = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: wiring(),
    });
    assert!(s.launch_realtime(engine_world::WorldState::new()).is_err());
}
