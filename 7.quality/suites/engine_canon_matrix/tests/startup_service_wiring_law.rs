// Startup Service Wiring Law Tests

use engine_content::{ContentLocator, ContentManifest, ContentPack};
use engine_core::EngineCoreError;
use engine_runtime::RuntimeProfile;
use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};

#[test]
fn startup_invalid_profile_role_combination_fails() {
    let assembly = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: full_service_wiring(),
    });

    let decision = assembly.validate();
    assert!(!decision.accepted);
    assert_eq!(
        decision.reasons,
        vec!["headless profile cannot bind interactive host-aware role".to_string()]
    );
}

#[test]
fn startup_missing_required_service_fails_closed() {
    let assembly = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: false,
            ..full_service_wiring()
        },
    });

    let decision = assembly.validate();
    assert!(!decision.accepted);
    assert_eq!(
        decision.reasons,
        vec!["missing required startup service: networking".to_string()]
    );
    assert_eq!(
        assembly.runtime_launch_plan().unwrap_err(),
        EngineCoreError::InvalidDescriptor("missing required startup service: networking")
    );
}

#[test]
fn startup_runtime_pack_compatibility_is_checked() {
    let assembly = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![ContentManifest {
            packs: vec![ContentPack {
                pack_id: 10,
                chunk_count: 0,
            }],
            locators: vec![ContentLocator {
                uri: "pack://interactive/base".to_string(),
            }],
        }],
        service_wiring: full_service_wiring(),
    });

    let decision = assembly.validate();
    assert!(!decision.accepted);
    assert_eq!(
        decision.reasons,
        vec!["runtime pack chunk count must be non-zero".to_string()]
    );
}

#[test]
fn startup_valid_profile_emits_required_service_receipt() {
    let assembly = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: full_service_wiring(),
    });

    let receipt = assembly.service_wiring_receipt().unwrap();
    assert_eq!(receipt.required_service_count, 6);
    assert_eq!(receipt.wired_service_count, 8);
    assert_eq!(receipt.required_service_bits, 0b0011_1111);
    assert_eq!(receipt.wired_service_bits, 0b1111_1111);
}

#[test]
fn same_startup_profile_same_digest() {
    let config = StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: true,
            modeling: false,
            synthesis: true,
            ..base_interactive_wiring()
        },
    };

    let assembly1 = StartupAssembly::new(config.clone());
    let assembly2 = StartupAssembly::new(config);

    let receipt1 = assembly1.service_wiring_receipt().unwrap();
    let receipt2 = assembly2.service_wiring_receipt().unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn same_count_different_service_bits_change_digest() {
    let assembly1 = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: true,
            modeling: false,
            synthesis: true,
            ..base_interactive_wiring()
        },
    });

    let assembly2 = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: false,
            modeling: true,
            synthesis: true,
            ..base_interactive_wiring()
        },
    });

    let receipt1 = assembly1.service_wiring_receipt().unwrap();
    let receipt2 = assembly2.service_wiring_receipt().unwrap();

    assert_eq!(receipt1.wired_service_count, receipt2.wired_service_count);
    assert_ne!(receipt1.wired_service_bits, receipt2.wired_service_bits);
    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn different_profile_changes_digest() {
    let assembly1 = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: full_service_wiring(),
    });

    let assembly2 = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: true,
            modeling: false,
            synthesis: true,
            ..base_interactive_wiring()
        },
    });

    let receipt1 = assembly1.service_wiring_receipt().unwrap();
    let receipt2 = assembly2.service_wiring_receipt().unwrap();

    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

fn base_interactive_wiring() -> ServiceWiring {
    ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: false,
        modeling: false,
        synthesis: true,
    }
}

fn full_service_wiring() -> ServiceWiring {
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
