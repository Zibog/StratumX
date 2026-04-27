use engine_content::{ContentManifest, ContentPack};
use engine_runtime::RuntimeProfile;
use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};
use engine_world::WorldState;

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
fn invalid_profile_role_pair_is_rejected() {
    let startup = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: wiring(),
    });
    assert!(!startup.validate().accepted);
    assert!(startup.runtime_launch_plan().is_err());
}

#[test]
fn headless_launch_requires_headless_profile() {
    let startup = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![ContentManifest {
            packs: vec![ContentPack {
                pack_id: 1,
                chunk_count: 1,
            }],
            locators: vec![],
        }],
        service_wiring: wiring(),
    });
    assert!(startup.launch_headless(WorldState::new()).is_err());
}

#[test]
fn realtime_launch_rejects_headless_profile() {
    let startup = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![ContentManifest {
            packs: vec![ContentPack {
                pack_id: 1,
                chunk_count: 1,
            }],
            locators: vec![],
        }],
        service_wiring: wiring(),
    });
    assert!(startup.launch_realtime(WorldState::new()).is_err());
}
