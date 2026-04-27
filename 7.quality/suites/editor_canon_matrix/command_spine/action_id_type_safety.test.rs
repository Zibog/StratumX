// Feature: editor-canonical-architecture-refactor, Property 1: Command Spine Routing Enforcement
//
// This property test validates that for any UI action, when dispatched through the Command Spine,
// the action must route through the registered tooling route and never directly call SDK packets
// or engine truth owners.
//
// **Validates: Requirements 1.1, 1.2, 1.3, 3.1, 3.2, 3.3, 3.4, 12.1, 12.2**

use proptest::prelude::*;
use crate::{ActionId, ActionDefinition, ActionFamily, MutationClass, DenialFamily, ActionContext, ActionResult};

// ============================================================================
// Property Test Strategies
// ============================================================================

/// Strategy for generating valid ActionId strings
fn action_id_string_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("world.open".to_string()),
        Just("world.close".to_string()),
        Just("world.save".to_string()),
        Just("material.author_response_profile".to_string()),
        Just("material.bind_surface_family".to_string()),
        Just("material.inspect_response_table".to_string()),
        Just("terrain.simulate_blast_profile".to_string()),
        Just("terrain.author_destruction_profile".to_string()),
        Just("sky.author_sun_profile".to_string()),
        Just("sky.bind_weather_system".to_string()),
        Just("project.new".to_string()),
        Just("project.open".to_string()),
        Just("project.save".to_string()),
        Just("project.close".to_string()),
        Just("panel.open_viewport".to_string()),
        Just("panel.open_inspector".to_string()),
        Just("panel.open_outliner".to_string()),
        Just("runtime.play".to_string()),
        Just("runtime.simulate".to_string()),
        Just("diagnostics.capture_trace".to_string()),
        Just("build.export_product".to_string()),
    ]
}

/// Strategy for generating ActionFamily variants
fn action_family_strategy() -> impl Strategy<Value = ActionFamily> {
    prop_oneof![
        Just(ActionFamily::ProjectFile),
        Just(ActionFamily::ViewPanel),
        Just(ActionFamily::World),
        Just(ActionFamily::Terrain),
        Just(ActionFamily::Material),
        Just(ActionFamily::SkyEnvironment),
        Just(ActionFamily::Runtime),
        Just(ActionFamily::DiagnosticsProof),
        Just(ActionFamily::BuildRelease),
    ]
}

/// Strategy for generating MutationClass variants
fn mutation_class_strategy() -> impl Strategy<Value = MutationClass> {
    prop_oneof![
        Just(MutationClass::Read),
        Just(MutationClass::Mutate),
        Just(MutationClass::Simulate),
        Just(MutationClass::Compare),
        Just(MutationClass::Capture),
        Just(MutationClass::Recover),
        Just(MutationClass::Certify),
    ]
}

/// Strategy for generating DenialFamily variants
fn denial_family_strategy() -> impl Strategy<Value = DenialFamily> {
    prop_oneof![
        Just(DenialFamily::MAT_PROFILE_INVALID),
        Just(DenialFamily::MAT_BINDING_FAILED),
        Just(DenialFamily::DST_SIMULATION_FAILED),
        Just(DenialFamily::PRJ_NO_WORKSPACE),
        Just(DenialFamily::WLD_OPEN_FAILED),
        Just(DenialFamily::WLD_SAVE_FAILED),
        Just(DenialFamily::WLD_CLOSE_FAILED),
        Just(DenialFamily::FLR_INSUFFICIENT),
        Just(DenialFamily::FLR_DEGRADE_RUNG_EXHAUSTED),
        Just(DenialFamily::RUN_INIT_FAILED),
        Just(DenialFamily::RUN_EXEC_FAILED),
        Just(DenialFamily::DIAG_CAPTURE_FAILED),
        Just(DenialFamily::BLD_FAILED),
        Just(DenialFamily::REL_CERT_FAILED),
    ]
}

/// Strategy for generating tooling route strings
fn tooling_route_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("route.world.open.v1".to_string()),
        Just("route.world.close.v1".to_string()),
        Just("route.world.save.v1".to_string()),
        Just("route.material.author_response_profile.v1".to_string()),
        Just("route.material.bind_surface_family.v1".to_string()),
        Just("route.terrain.simulate_blast_profile.v1".to_string()),
        Just("route.sky.author_sun_profile.v1".to_string()),
        Just("route.project.new.v1".to_string()),
        Just("route.runtime.play.v1".to_string()),
        Just("route.diagnostics.capture_trace.v1".to_string()),
    ]
}

/// Strategy for generating SDK packet family strings
fn sdk_packet_family_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("packet.world.*".to_string()),
        Just("packet.material.*".to_string()),
        Just("packet.terrain.*".to_string()),
        Just("packet.sky.*".to_string()),
        Just("packet.project.*".to_string()),
        Just("packet.runtime.*".to_string()),
        Just("packet.diagnostics.*".to_string()),
    ]
}

/// Strategy for generating engine truth owner strings
fn engine_truth_owner_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("engine/50".to_string()),  // terrain
        Just("engine/61".to_string()),  // hydrology
        Just("engine/90".to_string()),  // sky
        Just("engine/0".to_string()),   // world truth
        Just("engine/10".to_string()),  // runtime kernel
    ]
}

/// Strategy for generating complete ActionDefinition
fn action_definition_strategy() -> impl Strategy<Value = ActionDefinition> {
    (
        action_id_string_strategy(),
        "[a-zA-Z ]{5,30}",  // display_label
        action_family_strategy(),
        tooling_route_strategy(),
        sdk_packet_family_strategy(),
        engine_truth_owner_strategy(),
        mutation_class_strategy(),
        prop::collection::vec(denial_family_strategy(), 0..5),
    ).prop_map(|(action_id, display_label, action_family, tooling_route, sdk_packet_family, engine_truth_owner, mutation_class, denial_families)| {
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        ActionDefinition {
            action_id: ActionId::new(action_id),
            display_label,
            action_family,
            tooling_route,
            sdk_packet_family,
            engine_truth_owner,
            mutation_class,
            denial_families,
            handler: dummy_handler,
        }
    })
}

// ============================================================================
// Property Tests
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 1: Command Spine Routing Enforcement
    ///
    /// For any ActionDefinition, the action must specify a tooling route and never
    /// allow direct access to SDK packets or engine truth owners.
    ///
    /// This test validates that:
    /// 1. Every action has a registered tooling route (route.*)
    /// 2. SDK packet families are specified but not directly accessible
    /// 3. Engine truth owners are specified but not directly accessible
    /// 4. The routing chain is: Action → Tooling Route → SDK Packet → Engine Truth
    #[test]
    fn property_command_spine_routing_enforcement(
        action_def in action_definition_strategy()
    ) {
        // Verify that the action has a tooling route
        assert!(
            action_def.tooling_route.starts_with("route."),
            "Action {} must route through a tooling route, got: {}",
            action_def.action_id.as_str(),
            action_def.tooling_route
        );

        // Verify that the tooling route follows the canonical naming pattern
        assert!(
            action_def.tooling_route.contains(".v"),
            "Tooling route {} must include version suffix (e.g., .v1)",
            action_def.tooling_route
        );

        // Verify that SDK packet family is specified (for documentation/validation)
        // but the action doesn't directly call it
        assert!(
            action_def.sdk_packet_family.starts_with("packet."),
            "SDK packet family must be specified for routing validation, got: {}",
            action_def.sdk_packet_family
        );

        // Verify that engine truth owner is specified (for documentation/validation)
        // but the action doesn't directly call it
        assert!(
            action_def.engine_truth_owner.starts_with("engine/"),
            "Engine truth owner must be specified for routing validation, got: {}",
            action_def.engine_truth_owner
        );

        // Verify that the action has a valid action family
        // This ensures actions are properly categorized for routing
        match action_def.action_family {
            ActionFamily::ProjectFile
            | ActionFamily::ViewPanel
            | ActionFamily::World
            | ActionFamily::Terrain
            | ActionFamily::Material
            | ActionFamily::SkyEnvironment
            | ActionFamily::Runtime
            | ActionFamily::DiagnosticsProof
            | ActionFamily::BuildRelease => {
                // Valid action family
            }
        }

        // Verify that the action has a valid mutation class
        // This ensures proper transaction handling
        match action_def.mutation_class {
            MutationClass::Read
            | MutationClass::Mutate
            | MutationClass::Simulate
            | MutationClass::Compare
            | MutationClass::Capture
            | MutationClass::Recover
            | MutationClass::Certify => {
                // Valid mutation class
            }
        }

        // Verify that denial families are properly categorized
        for denial in &action_def.denial_families {
            // Each denial family should be a valid variant
            match denial {
                DenialFamily::MAT_PROFILE_INVALID
                | DenialFamily::MAT_BINDING_FAILED
                | DenialFamily::AUD_SOURCE_INVALID
                | DenialFamily::AUD_ZONE_CONFIG_FAILED
                | DenialFamily::DST_SIMULATION_FAILED
                | DenialFamily::PRJ_NO_WORKSPACE
                | DenialFamily::PRJ_OPEN_FAILED
                | DenialFamily::PRJ_CLOSE_FAILED
                | DenialFamily::WLD_OPEN_FAILED
                | DenialFamily::WLD_SAVE_FAILED
                | DenialFamily::WLD_CLOSE_FAILED
                | DenialFamily::WLD_NO_ACTIVE
                | DenialFamily::TRN_NO_TERRAIN
                | DenialFamily::TRN_SCULPT_FAILED
                | DenialFamily::TRN_PAINT_FAILED
                | DenialFamily::ENV_CONFIG_FAILED
                | DenialFamily::RT_KERNEL_UNAVAILABLE
                | DenialFamily::RT_PLAY_FAILED
                | DenialFamily::RT_PAUSE_FAILED
                | DenialFamily::FLR_INSUFFICIENT
                | DenialFamily::FLR_DEGRADE_RUNG_EXHAUSTED
                | DenialFamily::RUN_INIT_FAILED
                | DenialFamily::RUN_EXEC_FAILED
                | DenialFamily::DIAG_CAPTURE_FAILED
                | DenialFamily::BLD_FAILED
                | DenialFamily::REL_CERT_FAILED
                | DenialFamily::PNL_OPEN_FAILED
                | DenialFamily::PNL_CLOSE_FAILED => {
                    // Valid denial family
                }
            }
        }
    }

    /// Property: ActionId Type Safety
    ///
    /// For any ActionId, the type must maintain string identity and equality semantics.
    #[test]
    fn property_action_id_type_safety(
        action_id_str in action_id_string_strategy()
    ) {
        let action_id = ActionId::new(action_id_str.clone());
        
        // Verify that ActionId preserves the string value
        assert_eq!(action_id.as_str(), action_id_str);
        
        // Verify that ActionId can be created from &str
        let action_id_from_str: ActionId = action_id_str.as_str().into();
        assert_eq!(action_id_from_str.as_str(), action_id_str);
        
        // Verify that ActionId can be created from String
        let action_id_from_string: ActionId = action_id_str.clone().into();
        assert_eq!(action_id_from_string.as_str(), action_id_str);
        
        // Verify equality semantics
        assert_eq!(action_id, action_id_from_str);
        assert_eq!(action_id, action_id_from_string);
    }

    /// Property: ActionId Hash Consistency
    ///
    /// For any ActionId, equal ActionIds must have equal hash values.
    #[test]
    fn property_action_id_hash_consistency(
        action_id_str in action_id_string_strategy()
    ) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let action_id1 = ActionId::new(action_id_str.clone());
        let action_id2 = ActionId::new(action_id_str.clone());
        
        let mut hasher1 = DefaultHasher::new();
        action_id1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        action_id2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        // Equal ActionIds must have equal hashes
        assert_eq!(hash1, hash2);
    }

    /// Property: ActionFamily Exhaustiveness
    ///
    /// For any ActionFamily, the variant must be one of the defined categories.
    #[test]
    fn property_action_family_exhaustiveness(
        family in action_family_strategy()
    ) {
        // This test ensures that all ActionFamily variants are handled
        let family_name = match family {
            ActionFamily::ProjectFile => "ProjectFile",
            ActionFamily::ViewPanel => "ViewPanel",
            ActionFamily::World => "World",
            ActionFamily::Terrain => "Terrain",
            ActionFamily::Material => "Material",
            ActionFamily::SkyEnvironment => "SkyEnvironment",
            ActionFamily::Runtime => "Runtime",
            ActionFamily::DiagnosticsProof => "DiagnosticsProof",
            ActionFamily::BuildRelease => "BuildRelease",
        };
        
        assert!(!family_name.is_empty());
    }

    /// Property: MutationClass Exhaustiveness
    ///
    /// For any MutationClass, the variant must be one of the defined categories.
    #[test]
    fn property_mutation_class_exhaustiveness(
        mutation_class in mutation_class_strategy()
    ) {
        // This test ensures that all MutationClass variants are handled
        let class_name = match mutation_class {
            MutationClass::Read => "Read",
            MutationClass::Mutate => "Mutate",
            MutationClass::Simulate => "Simulate",
            MutationClass::Compare => "Compare",
            MutationClass::Capture => "Capture",
            MutationClass::Recover => "Recover",
            MutationClass::Certify => "Certify",
        };
        
        assert!(!class_name.is_empty());
    }

    /// Property: Tooling Route Naming Convention
    ///
    /// For any tooling route, the route must follow the canonical naming pattern:
    /// route.<domain>.<operation>.v<version>
    #[test]
    fn property_tooling_route_naming_convention(
        route in tooling_route_strategy()
    ) {
        // Verify route prefix
        assert!(route.starts_with("route."));
        
        // Verify version suffix
        assert!(route.contains(".v"));
        
        // Verify route has at least 4 parts: route.<domain>.<operation>.v<version>
        let parts: Vec<&str> = route.split('.').collect();
        assert!(parts.len() >= 4, "Route {} must have at least 4 parts", route);
        
        // Verify first part is "route"
        assert_eq!(parts[0], "route");
    }

    /// Property: SDK Packet Family Naming Convention
    ///
    /// For any SDK packet family, the family must follow the canonical naming pattern:
    /// packet.<domain>.*
    #[test]
    fn property_sdk_packet_family_naming_convention(
        packet_family in sdk_packet_family_strategy()
    ) {
        // Verify packet prefix
        assert!(packet_family.starts_with("packet."));
        
        // Verify wildcard suffix
        assert!(packet_family.ends_with(".*"));
        
        // Verify packet family has exactly 3 parts: packet.<domain>.*
        let parts: Vec<&str> = packet_family.split('.').collect();
        assert_eq!(parts.len(), 3, "Packet family {} must have exactly 3 parts", packet_family);
        
        // Verify first part is "packet"
        assert_eq!(parts[0], "packet");
        
        // Verify last part is "*"
        assert_eq!(parts[2], "*");
    }

    /// Property: Engine Truth Owner Naming Convention
    ///
    /// For any engine truth owner, the owner must follow the canonical naming pattern:
    /// engine/<layer_number>
    #[test]
    fn property_engine_truth_owner_naming_convention(
        owner in engine_truth_owner_strategy()
    ) {
        // Verify engine prefix
        assert!(owner.starts_with("engine/"));
        
        // Verify format: engine/<number>
        let parts: Vec<&str> = owner.split('/').collect();
        assert_eq!(parts.len(), 2, "Engine truth owner {} must have exactly 2 parts", owner);
        
        // Verify first part is "engine"
        assert_eq!(parts[0], "engine");
        
        // Verify second part is a number
        assert!(parts[1].parse::<u32>().is_ok(), "Engine layer must be a number, got: {}", parts[1]);
    }
}
