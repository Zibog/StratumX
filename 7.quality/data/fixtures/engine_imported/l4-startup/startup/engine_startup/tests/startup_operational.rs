use engine_content::{ContentManifest, ContentPack};
use engine_runtime::RuntimeProfile;
use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};

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
fn runtime_launch_plan_counts_runtime_packs() {
    let startup = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![
            ContentManifest {
                packs: vec![
                    ContentPack {
                        pack_id: 1,
                        chunk_count: 1,
                    },
                    ContentPack {
                        pack_id: 2,
                        chunk_count: 1,
                    },
                ],
                locators: vec![],
            },
            ContentManifest {
                packs: vec![ContentPack {
                    pack_id: 3,
                    chunk_count: 1,
                }],
                locators: vec![],
            },
        ],
        service_wiring: wiring(),
    });
    let plan = startup.runtime_launch_plan().unwrap();
    assert_eq!(plan.runtime_pack_count, 3);
}
