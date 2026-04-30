use engine_content::{ContentLocator, ContentManifest, ContentPack};
use engine_core::EngineCoreError;
use engine_runtime::RuntimeProfile;
use engine_startup::{
    NetworkRole, ServiceWiring, StartupAssembly, StartupConfig, StartupFailure,
    StartupFailureReason, StartupServiceId,
};

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
fn startup_missing_required_service_returns_typed_failure() {
    let assembly = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: false,
            ..full_service_wiring()
        },
    });

    let failure = assembly.runtime_launch_plan().unwrap_err();

    assert_eq!(
        failure.reason,
        StartupFailureReason::MissingRequiredService(StartupServiceId::Networking)
    );
}

#[test]
fn startup_runtime_pack_duplicate_returns_typed_failure() {
    let assembly = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![
            ContentManifest {
                packs: vec![ContentPack {
                    pack_id: 10,
                    chunk_count: 1,
                }],
                locators: vec![ContentLocator {
                    uri: "pack://interactive/base".to_string(),
                }],
            },
            ContentManifest {
                packs: vec![ContentPack {
                    pack_id: 10,
                    chunk_count: 2,
                }],
                locators: vec![ContentLocator {
                    uri: "pack://interactive/dup".to_string(),
                }],
            },
        ],
        service_wiring: full_service_wiring(),
    });

    let failure = assembly.service_wiring_receipt().unwrap_err();

    assert_eq!(failure.reason, StartupFailureReason::RuntimePackDuplicateId);
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

    let receipt1 = StartupAssembly::new(config.clone())
        .service_wiring_receipt()
        .unwrap();
    let receipt2 = StartupAssembly::new(config)
        .service_wiring_receipt()
        .unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn same_count_different_service_bits_change_digest() {
    let receipt1 = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: true,
            modeling: false,
            synthesis: true,
            ..base_interactive_wiring()
        },
    })
    .service_wiring_receipt()
    .unwrap();
    let receipt2 = StartupAssembly::new(StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: ServiceWiring {
            networking: false,
            modeling: true,
            synthesis: true,
            ..base_interactive_wiring()
        },
    })
    .service_wiring_receipt()
    .unwrap();

    assert_eq!(receipt1.wired_service_count, receipt2.wired_service_count);
    assert_ne!(receipt1.wired_service_bits, receipt2.wired_service_bits);
    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn startup_engine_core_error_bridge_preserves_reason() {
    let bridge: EngineCoreError = StartupFailure::for_reason(
        StartupFailureReason::MissingRequiredService(StartupServiceId::Networking),
    )
    .into();

    assert_eq!(
        bridge,
        EngineCoreError::InvalidDescriptor("missing required startup service: networking")
    );
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
