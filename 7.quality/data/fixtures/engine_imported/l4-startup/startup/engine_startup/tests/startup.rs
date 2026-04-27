use engine_content::{ContentManifest, ContentPack};
use engine_runtime::RuntimeProfile;
use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};
use engine_world::WorldState;

#[test]
fn startup_builds_headless_launch_plan() {
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
        service_wiring: ServiceWiring {
            streaming: true,
            residency: true,
            memory: true,
            transfer: true,
            simulation: true,
            networking: true,
            modeling: false,
            synthesis: false,
        },
    });
    assert!(startup.validate().accepted);
    let _profile = startup.launch_headless(WorldState::new()).unwrap();
}
