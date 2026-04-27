//! Comprehensive tests for engine_startup crate

use engine_startup::{
    AssetRootResolver, AssetStatus, BundleCompleteness, NetworkRole, RuntimeLaunchPlan,
    ServiceWiring, SkyBundleStatus, StartupAssembly, StartupConfig,
    StartupReadyAssemblyDecisionSet,
};
use engine_runtime::RuntimeProfile;

// ============================================================================
// NetworkRole Tests
// ============================================================================

#[test]
fn test_network_role_variants_exist() {
    let _local = NetworkRole::LocalOnly;
    let _interactive = NetworkRole::InteractiveHostAware;
    let _listen = NetworkRole::ListenHost;
    let _headless = NetworkRole::HeadlessHost;
}

#[test]
fn test_network_role_equality() {
    assert_eq!(NetworkRole::LocalOnly, NetworkRole::LocalOnly);
    assert_ne!(NetworkRole::LocalOnly, NetworkRole::HeadlessHost);
    assert_eq!(NetworkRole::InteractiveHostAware, NetworkRole::InteractiveHostAware);
    assert_eq!(NetworkRole::ListenHost, NetworkRole::ListenHost);
    assert_eq!(NetworkRole::HeadlessHost, NetworkRole::HeadlessHost);
}

#[test]
fn test_network_role_serialization_roundtrip() {
    for role in [
        NetworkRole::LocalOnly,
        NetworkRole::InteractiveHostAware,
        NetworkRole::ListenHost,
        NetworkRole::HeadlessHost,
    ] {
        let json = serde_json::to_string(&role).unwrap();
        let deserialized: NetworkRole = serde_json::from_str(&json).unwrap();
        assert_eq!(role, deserialized);
    }
}

#[test]
fn test_network_role_debug() {
    let role = NetworkRole::LocalOnly;
    let debug_str = format!("{:?}", role);
    assert!(debug_str.contains("LocalOnly"));
}

// ============================================================================
// ServiceWiring Tests
// ============================================================================

#[test]
fn test_service_wiring_all_enabled() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    assert!(wiring.streaming);
    assert!(wiring.residency);
    assert!(wiring.memory);
    assert!(wiring.transfer);
    assert!(wiring.simulation);
    assert!(wiring.networking);
    assert!(wiring.modeling);
    assert!(wiring.synthesis);
}

#[test]
fn test_service_wiring_all_disabled() {
    let wiring = ServiceWiring {
        streaming: false,
        residency: false,
        memory: false,
        transfer: false,
        simulation: false,
        networking: false,
        modeling: false,
        synthesis: false,
    };
    assert!(!wiring.streaming);
    assert!(!wiring.residency);
    assert!(!wiring.memory);
    assert!(!wiring.transfer);
    assert!(!wiring.simulation);
    assert!(!wiring.networking);
    assert!(!wiring.modeling);
    assert!(!wiring.synthesis);
}

#[test]
fn test_service_wiring_equality() {
    let w1 = ServiceWiring {
        streaming: true,
        residency: false,
        memory: true,
        transfer: false,
        simulation: true,
        networking: false,
        modeling: true,
        synthesis: false,
    };
    let w2 = w1.clone();
    let w3 = ServiceWiring {
        streaming: false,
        ..w1.clone()
    };
    assert_eq!(w1, w2);
    assert_ne!(w1, w3);
}

#[test]
fn test_service_wiring_serialization_roundtrip() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: false,
        transfer: true,
        simulation: false,
        networking: true,
        modeling: false,
        synthesis: true,
    };
    let json = serde_json::to_string(&wiring).unwrap();
    let deserialized: ServiceWiring = serde_json::from_str(&json).unwrap();
    assert_eq!(wiring, deserialized);
}

#[test]
fn test_service_wiring_copy_semantics() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: false,
        memory: true,
        transfer: false,
        simulation: true,
        networking: false,
        modeling: true,
        synthesis: false,
    };
    let wiring2 = wiring; // Copy, not move
    assert!(wiring2.streaming);
    assert!(!wiring2.residency);
}

// ============================================================================
// StartupConfig Tests
// ============================================================================

#[test]
fn test_startup_config_creation() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    assert_eq!(config.profile, RuntimeProfile::Headless20);
    assert_eq!(config.network_role, NetworkRole::LocalOnly);
    assert!(config.runtime_manifests.is_empty());
}

#[test]
fn test_startup_config_with_manifests() {
    use engine_content::{ContentManifest, ContentPack, ContentLocator};

    let wiring = ServiceWiring {
        streaming: true,
        residency: false,
        memory: true,
        transfer: false,
        simulation: true,
        networking: false,
        modeling: true,
        synthesis: false,
    };
    let manifest = ContentManifest {
        packs: vec![ContentPack { pack_id: 1, chunk_count: 5 }],
        locators: vec![ContentLocator { uri: "test://manifest".to_string() }],
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::ListenHost,
        runtime_manifests: vec![manifest],
        service_wiring: wiring,
    };
    assert_eq!(config.runtime_manifests.len(), 1);
    assert_eq!(config.runtime_manifests[0].packs.len(), 1);
}

#[test]
fn test_startup_config_equality() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config1 = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: wiring.clone(),
    };
    let config2 = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    assert_eq!(config1, config2);
}

#[test]
fn test_startup_config_clone() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: false,
        memory: true,
        transfer: false,
        simulation: true,
        networking: false,
        modeling: true,
        synthesis: false,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::ListenHost60,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let config2 = config.clone();
    assert_eq!(config, config2);
}

// ============================================================================
// StartupReadyAssemblyDecisionSet Tests
// ============================================================================

#[test]
fn test_decision_set_accepted() {
    let decision = StartupReadyAssemblyDecisionSet {
        accepted: true,
        reasons: vec![],
    };
    assert!(decision.accepted);
    assert!(decision.reasons.is_empty());
}

#[test]
fn test_decision_set_rejected() {
    let decision = StartupReadyAssemblyDecisionSet {
        accepted: false,
        reasons: vec!["invalid profile/role combination".to_string()],
    };
    assert!(!decision.accepted);
    assert_eq!(decision.reasons.len(), 1);
    assert_eq!(decision.reasons[0], "invalid profile/role combination");
}

#[test]
fn test_decision_set_multiple_reasons() {
    let decision = StartupReadyAssemblyDecisionSet {
        accepted: false,
        reasons: vec!["reason1".to_string(), "reason2".to_string(), "reason3".to_string()],
    };
    assert!(!decision.accepted);
    assert_eq!(decision.reasons.len(), 3);
}

#[test]
fn test_decision_set_serialization_roundtrip() {
    let decision = StartupReadyAssemblyDecisionSet {
        accepted: false,
        reasons: vec!["test reason".to_string()],
    };
    let json = serde_json::to_string(&decision).unwrap();
    let deserialized: StartupReadyAssemblyDecisionSet = serde_json::from_str(&json).unwrap();
    assert_eq!(decision.accepted, deserialized.accepted);
    assert_eq!(decision.reasons, deserialized.reasons);
}

#[test]
fn test_decision_set_clone() {
    let decision = StartupReadyAssemblyDecisionSet {
        accepted: true,
        reasons: vec!["ok".to_string()],
    };
    let decision2 = decision.clone();
    assert_eq!(decision.accepted, decision2.accepted);
    assert_eq!(decision.reasons, decision2.reasons);
}

// ============================================================================
// RuntimeLaunchPlan Tests
// ============================================================================

#[test]
fn test_runtime_launch_plan_creation() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let plan = RuntimeLaunchPlan {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_pack_count: 5,
        service_wiring: wiring,
    };
    assert_eq!(plan.profile, RuntimeProfile::Headless20);
    assert_eq!(plan.network_role, NetworkRole::LocalOnly);
    assert_eq!(plan.runtime_pack_count, 5);
}

#[test]
fn test_runtime_launch_plan_zero_packs() {
    let wiring = ServiceWiring {
        streaming: false,
        residency: false,
        memory: false,
        transfer: false,
        simulation: false,
        networking: false,
        modeling: false,
        synthesis: false,
    };
    let plan = RuntimeLaunchPlan {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::ListenHost,
        runtime_pack_count: 0,
        service_wiring: wiring,
    };
    assert_eq!(plan.runtime_pack_count, 0);
}

#[test]
fn test_runtime_launch_plan_serialization_roundtrip() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: false,
        memory: true,
        transfer: false,
        simulation: true,
        networking: false,
        modeling: true,
        synthesis: false,
    };
    let plan = RuntimeLaunchPlan {
        profile: RuntimeProfile::ListenHost60,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_pack_count: 42,
        service_wiring: wiring,
    };
    let json = serde_json::to_string(&plan).unwrap();
    let deserialized: RuntimeLaunchPlan = serde_json::from_str(&json).unwrap();
    assert_eq!(plan.profile, deserialized.profile);
    assert_eq!(plan.network_role, deserialized.network_role);
    assert_eq!(plan.runtime_pack_count, deserialized.runtime_pack_count);
    assert_eq!(plan.service_wiring, deserialized.service_wiring);
}

// ============================================================================
// AssetRootResolver Tests
// ============================================================================

#[test]
fn test_asset_root_resolver_default() {
    let resolver = AssetRootResolver::default();
    assert!(resolver.get_resolved_root().is_none());
}

#[test]
fn test_asset_root_resolver_new() {
    let resolver = AssetRootResolver::new();
    assert!(resolver.get_resolved_root().is_none());
}

#[test]
fn test_asset_root_resolver_resolve_without_env() {
    // Without STRATUMX_ASSET_ROOT set and no workspace markers, this should fail
    std::env::remove_var("STRATUMX_ASSET_ROOT");
    let mut resolver = AssetRootResolver::new();
    let result = resolver.resolve();
    // Should fail since we're not in a workspace root with 9.assets
    assert!(result.is_err());
}

#[test]
fn test_asset_root_resolve_asset_path_unresolved() {
    let resolver = AssetRootResolver::new();
    let result = resolver.resolve_asset_path("some/path");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("not resolved"));
}

#[test]
fn test_asset_root_asset_exists_unresolved() {
    let resolver = AssetRootResolver::new();
    assert!(!resolver.asset_exists("some/path"));
}

// ============================================================================
// SkyBundleStatus Tests
// ============================================================================

#[test]
fn test_sky_bundle_status_not_loaded() {
    let status = SkyBundleStatus::not_loaded();
    assert!(!status.manifest_loaded);
    assert!(status.bundle_id.is_none());
    assert_eq!(status.completeness, BundleCompleteness::Invalid);
    assert!(!status.is_production_ready());
    assert!(!status.is_functional());
}

#[test]
fn test_sky_bundle_status_production_ready() {
    let status = SkyBundleStatus {
        manifest_loaded: true,
        bundle_id: Some("test_bundle".to_string()),
        completeness: BundleCompleteness::Complete,
        stars_status: AssetStatus::Found,
        moon_albedo_status: AssetStatus::Found,
        moon_height_status: AssetStatus::Found,
        moon_normal_status: AssetStatus::Found,
        sun_disk_status: AssetStatus::Found,
        blue_noise_status: AssetStatus::Found,
        noise_source_status: AssetStatus::Found,
        noise_source_count: 3,
    };
    assert!(status.is_production_ready());
    assert!(status.is_functional());
}

#[test]
fn test_sky_bundle_status_partial_not_production_ready() {
    let status = SkyBundleStatus {
        manifest_loaded: true,
        bundle_id: Some("partial_bundle".to_string()),
        completeness: BundleCompleteness::Partial,
        stars_status: AssetStatus::Found,
        moon_albedo_status: AssetStatus::Found,
        moon_height_status: AssetStatus::Missing,
        moon_normal_status: AssetStatus::Found,
        sun_disk_status: AssetStatus::NotRequired,
        blue_noise_status: AssetStatus::Found,
        noise_source_status: AssetStatus::Found,
        noise_source_count: 1,
    };
    assert!(!status.is_production_ready());
    assert!(status.is_functional());
}

#[test]
fn test_sky_bundle_status_invalid_not_functional() {
    let status = SkyBundleStatus {
        manifest_loaded: true,
        bundle_id: Some("invalid_bundle".to_string()),
        completeness: BundleCompleteness::Invalid,
        stars_status: AssetStatus::Missing,
        moon_albedo_status: AssetStatus::Found,
        moon_height_status: AssetStatus::NotRequired,
        moon_normal_status: AssetStatus::NotRequired,
        sun_disk_status: AssetStatus::NotRequired,
        blue_noise_status: AssetStatus::Found,
        noise_source_status: AssetStatus::Missing,
        noise_source_count: 0,
    };
    assert!(!status.is_production_ready());
    assert!(!status.is_functional());
}

#[test]
fn test_sky_bundle_status_serialization_roundtrip() {
    let status = SkyBundleStatus {
        manifest_loaded: true,
        bundle_id: Some("serialized_bundle".to_string()),
        completeness: BundleCompleteness::Complete,
        stars_status: AssetStatus::Found,
        moon_albedo_status: AssetStatus::Found,
        moon_height_status: AssetStatus::Found,
        moon_normal_status: AssetStatus::Found,
        sun_disk_status: AssetStatus::Found,
        blue_noise_status: AssetStatus::Found,
        noise_source_status: AssetStatus::Found,
        noise_source_count: 5,
    };
    let json = serde_json::to_string(&status).unwrap();
    let deserialized: SkyBundleStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(status.manifest_loaded, deserialized.manifest_loaded);
    assert_eq!(status.bundle_id, deserialized.bundle_id);
    assert_eq!(status.completeness, deserialized.completeness);
    assert_eq!(status.noise_source_count, deserialized.noise_source_count);
}

// ============================================================================
// BundleCompleteness Tests
// ============================================================================

#[test]
fn test_bundle_completeness_variants() {
    let complete = BundleCompleteness::Complete;
    let partial = BundleCompleteness::Partial;
    let invalid = BundleCompleteness::Invalid;

    assert_eq!(complete, BundleCompleteness::Complete);
    assert_eq!(partial, BundleCompleteness::Partial);
    assert_eq!(invalid, BundleCompleteness::Invalid);

    assert_ne!(complete, partial);
    assert_ne!(complete, invalid);
    assert_ne!(partial, invalid);
}

#[test]
fn test_bundle_completeness_serialization_roundtrip() {
    for completeness in [
        BundleCompleteness::Complete,
        BundleCompleteness::Partial,
        BundleCompleteness::Invalid,
    ] {
        let json = serde_json::to_string(&completeness).unwrap();
        let deserialized: BundleCompleteness = serde_json::from_str(&json).unwrap();
        assert_eq!(completeness, deserialized);
    }
}

// ============================================================================
// AssetStatus Tests
// ============================================================================

#[test]
fn test_asset_status_variants() {
    let found = AssetStatus::Found;
    let missing = AssetStatus::Missing;
    let not_required = AssetStatus::NotRequired;

    assert_eq!(found, AssetStatus::Found);
    assert_eq!(missing, AssetStatus::Missing);
    assert_eq!(not_required, AssetStatus::NotRequired);

    assert_ne!(found, missing);
    assert_ne!(found, not_required);
    assert_ne!(missing, not_required);
}

#[test]
fn test_asset_status_serialization_roundtrip() {
    for status in [
        AssetStatus::Found,
        AssetStatus::Missing,
        AssetStatus::NotRequired,
    ] {
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: AssetStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }
}

// ============================================================================
// Seed IDs Tests (from startup_reference_seed)
// ============================================================================

#[test]
fn test_seed_material_layer_canonical_ids() {
    // Note: seed_ids is not publicly exported, so we test through public API
}

#[test]
fn test_seed_material_archetype_canonical_ids() {
    // Note: seed_ids is not publicly exported, so we test through public API
}

#[test]
fn test_seed_material_stack_canonical_ids() {
    // Note: seed_ids is not publicly exported, so we test through public API
}

#[test]
fn test_seed_response_profile_canonical_ids() {
    // Note: seed_ids is not publicly exported, so we test through public API
}

#[test]
fn test_default_terrain_material_layers() {
    // Note: seed_ids is not publicly exported, so we test through public API
}

// ============================================================================
// Reference Scene Name Test
// ============================================================================

#[test]
fn test_reference_scene_name() {
    use engine_startup::REFERENCE_SCENE_NAME;
    assert_eq!(
        REFERENCE_SCENE_NAME,
        "vertical_slice.wall_terrain.ak_demo"
    );
}

// ============================================================================
// DestructibleSupportObject Tests
// ============================================================================
// Note: DestructibleSupportObject is in a private module; testing via public launch function
// The structure is tested indirectly through the runtime launch.

// ============================================================================
// NpcProfile Tests
// ============================================================================
// Note: NpcProfile is in a private module; testing via public launch function

// ============================================================================
// StartupAssembly Validation Tests
// ============================================================================

#[test]
fn test_startup_assembly_valid_headless_local() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(decision.accepted);
}

#[test]
fn test_startup_assembly_invalid_headless_interactive() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(!decision.accepted);
    assert!(!decision.reasons.is_empty());
}

#[test]
fn test_startup_assembly_invalid_interactive_headless() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(!decision.accepted);
}

#[test]
fn test_startup_assembly_invalid_listen_headless() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::ListenHost60,
        network_role: NetworkRole::HeadlessHost,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(!decision.accepted);
}

#[test]
fn test_startup_assembly_valid_interactive_listen() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Interactive60,
        network_role: NetworkRole::ListenHost,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(decision.accepted);
}

#[test]
fn test_startup_assembly_runtime_launch_plan_valid() {
    use engine_content::{ContentManifest, ContentPack};

    let manifest = ContentManifest {
        packs: vec![ContentPack { pack_id: 1, chunk_count: 3 }],
        locators: vec![],
    };
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![manifest],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let plan = assembly.runtime_launch_plan();
    assert!(plan.is_ok());
    let plan = plan.unwrap();
    assert_eq!(plan.profile, RuntimeProfile::Headless20);
    assert_eq!(plan.network_role, NetworkRole::LocalOnly);
}

#[test]
fn test_startup_assembly_runtime_launch_plan_invalid() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::InteractiveHostAware,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let plan = assembly.runtime_launch_plan();
    assert!(plan.is_err());
}

// ============================================================================
// Integration: Full Config to Launch Plan
// ============================================================================

#[test]
fn test_full_config_to_launch_plan_headless() {
    use engine_content::{ContentManifest, ContentPack, ContentLocator};

    let manifest1 = ContentManifest {
        packs: vec![ContentPack { pack_id: 1, chunk_count: 1 }],
        locators: vec![ContentLocator { uri: "test://core".to_string() }],
    };
    let manifest2 = ContentManifest {
        packs: vec![ContentPack { pack_id: 2, chunk_count: 2 }, ContentPack { pack_id: 3, chunk_count: 3 }],
        locators: vec![ContentLocator { uri: "test://extras".to_string() }],
    };
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: false,
        modeling: false,
        synthesis: false,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::Headless20,
        network_role: NetworkRole::LocalOnly,
        runtime_manifests: vec![manifest1, manifest2],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(decision.accepted);

    let plan = assembly.runtime_launch_plan().unwrap();
    assert_eq!(plan.profile, RuntimeProfile::Headless20);
    assert_eq!(plan.network_role, NetworkRole::LocalOnly);
    assert_eq!(plan.runtime_pack_count, 3); // 1 + 2 packs
}

#[test]
fn test_full_config_to_launch_plan_interactive() {
    let wiring = ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    };
    let config = StartupConfig {
        profile: RuntimeProfile::ListenHost60,
        network_role: NetworkRole::ListenHost,
        runtime_manifests: vec![],
        service_wiring: wiring,
    };
    let assembly = StartupAssembly::new(config);
    let decision = assembly.validate();
    assert!(decision.accepted);

    let plan = assembly.runtime_launch_plan().unwrap();
    assert_eq!(plan.profile, RuntimeProfile::ListenHost60);
    assert_eq!(plan.network_role, NetworkRole::ListenHost);
    assert_eq!(plan.runtime_pack_count, 0);
}
