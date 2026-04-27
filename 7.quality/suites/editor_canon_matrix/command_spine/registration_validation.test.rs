//! Property Test: Action Registration Validation
//!
//! **Property 19: Action Registration Validation**
//! **Validates: Requirements 16.5, 17.5, 17.6**
//!
//! Verifies that the RegistryBuilder validates action definitions during registration.
//! Invalid definitions (missing metadata, empty fields) are rejected with descriptive errors.

use crate::{
    ActionContext, ActionDefinition, ActionFamily, ActionId, ActionResult, MutationClass,
};
use crate::registry_builder::{RegistrationError, RegistryBuilder};
use proptest::prelude::*;

// ============================================================================
// Property Test Generators
// ============================================================================

/// Generates action definitions with missing or invalid metadata
fn arb_invalid_action_definition() -> impl Strategy<Value = (ActionDefinition, &'static str)> {
    fn dummy_handler(_context: ActionContext) -> ActionResult {
        ActionResult::success(None)
    }

    prop_oneof![
        // Invalid action_id (empty)
        Just((
            ActionDefinition {
                action_id: ActionId::new(""),
                display_label: "Valid Label".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "route.test.v1".to_string(),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "invalid_action_id"
        )),
        // Empty display_label
        Just((
            ActionDefinition {
                action_id: ActionId::new("test.action"),
                display_label: "".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "route.test.v1".to_string(),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "empty_display_label"
        )),
        // Empty tooling_route
        Just((
            ActionDefinition {
                action_id: ActionId::new("test.action"),
                display_label: "Valid Label".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "".to_string(),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "empty_tooling_route"
        )),
        // Empty sdk_packet_family
        Just((
            ActionDefinition {
                action_id: ActionId::new("test.action"),
                display_label: "Valid Label".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "route.test.v1".to_string(),
                sdk_packet_family: "".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "empty_sdk_packet_family"
        )),
        // Empty engine_truth_owner
        Just((
            ActionDefinition {
                action_id: ActionId::new("test.action"),
                display_label: "Valid Label".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "route.test.v1".to_string(),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "empty_engine_truth_owner"
        )),
        // Whitespace-only display_label
        Just((
            ActionDefinition {
                action_id: ActionId::new("test.action"),
                display_label: "   ".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "route.test.v1".to_string(),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "whitespace_display_label"
        )),
        // Whitespace-only tooling_route
        Just((
            ActionDefinition {
                action_id: ActionId::new("test.action"),
                display_label: "Valid Label".to_string(),
                action_family: ActionFamily::World,
                tooling_route: "   ".to_string(),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            },
            "whitespace_tooling_route"
        )),
    ]
}

/// Generates valid action definitions
fn arb_valid_action_definition() -> impl Strategy<Value = ActionDefinition> {
    (
        prop_oneof![
            Just("world.open"),
            Just("world.close"),
            Just("material.author"),
            Just("terrain.sculpt"),
            Just("runtime.play"),
        ],
        prop_oneof![
            Just(ActionFamily::World),
            Just(ActionFamily::Material),
            Just(ActionFamily::Terrain),
            Just(ActionFamily::Runtime),
        ],
    )
        .prop_map(|(action_str, family)| {
            fn dummy_handler(_context: ActionContext) -> ActionResult {
                ActionResult::success(None)
            }

            ActionDefinition {
                action_id: ActionId::new(action_str),
                display_label: format!("Test {}", action_str),
                action_family: family,
                tooling_route: format!("route.{}.v1", action_str),
                sdk_packet_family: "packet.test.*".to_string(),
                engine_truth_owner: "engine/test".to_string(),
                mutation_class: MutationClass::Mutate,
                denial_families: vec![],
                handler: dummy_handler,
            }
        })
}

// ============================================================================
// Property Tests
// ============================================================================

/// **Property 19: Action Registration Validation**
///
/// **Validates: Requirements 16.5, 17.5, 17.6**
///
/// This property verifies that the RegistryBuilder validates action definitions
/// during registration. Invalid definitions are rejected with descriptive errors.
///
/// The test verifies:
/// 1. Definitions with empty action_id are rejected
/// 2. Definitions with empty display_label are rejected
/// 3. Definitions with empty tooling_route are rejected
/// 4. Definitions with empty sdk_packet_family are rejected
/// 5. Definitions with empty engine_truth_owner are rejected
/// 6. Whitespace-only fields are treated as empty and rejected
/// 7. Error messages are descriptive and identify the problem
#[test]
fn prop_registration_validation_rejects_invalid_definitions() {
    // Feature: editor-canonical-architecture-refactor, Property 19: Registration Validation
    let config = ProptestConfig::with_cases(50);
    proptest!(config, |((definition, error_type) in arb_invalid_action_definition())| {
        let mut builder = RegistryBuilder::new();
        
        // Attempt to register invalid definition
        let result = builder.register_action(definition.clone());
        
        // Verify registration fails
        prop_assert!(
            result.is_err(),
            "Invalid definition should be rejected: {}",
            error_type
        );
        
        // Verify error is descriptive
        let error = result.unwrap_err();
        let error_message = error.to_string();
        
        prop_assert!(
            !error_message.is_empty(),
            "Error message should be descriptive"
        );
        
        // Verify error type matches expected validation failure
        match error_type {
            "invalid_action_id" => {
                prop_assert!(
                    matches!(error, RegistrationError::InvalidActionId(_)),
                    "Should return InvalidActionId error"
                );
            }
            "empty_display_label" | "whitespace_display_label" => {
                prop_assert!(
                    matches!(error, RegistrationError::EmptyDisplayLabel(_)),
                    "Should return EmptyDisplayLabel error"
                );
            }
            "empty_tooling_route" | "whitespace_tooling_route" => {
                prop_assert!(
                    matches!(error, RegistrationError::EmptyToolingRoute(_)),
                    "Should return EmptyToolingRoute error"
                );
            }
            "empty_sdk_packet_family" => {
                prop_assert!(
                    matches!(error, RegistrationError::EmptySdkPacketFamily(_)),
                    "Should return EmptySdkPacketFamily error"
                );
            }
            "empty_engine_truth_owner" => {
                prop_assert!(
                    matches!(error, RegistrationError::EmptyEngineTruthOwner(_)),
                    "Should return EmptyEngineTruthOwner error"
                );
            }
            _ => {}
        }
    });
}

/// **Property: Valid Definitions Are Accepted**
///
/// Verifies that valid action definitions are accepted by the RegistryBuilder.
#[test]
fn prop_registration_validation_accepts_valid_definitions() {
    // Feature: editor-canonical-architecture-refactor, Property 19: Registration Validation
    let config = ProptestConfig::with_cases(100);
    proptest!(config, |(definition in arb_valid_action_definition())| {
        let mut builder = RegistryBuilder::new();
        
        // Attempt to register valid definition
        let result = builder.register_action(definition.clone());
        
        // Verify registration succeeds
        prop_assert!(
            result.is_ok(),
            "Valid definition should be accepted"
        );
        
        // Verify builder contains the action
        prop_assert_eq!(
            builder.len(),
            1,
            "Builder should contain one action after registration"
        );
    });
}

/// **Property: All Required Fields Are Validated**
///
/// Verifies that all required metadata fields are validated during registration.
#[test]
fn prop_all_required_fields_validated() {
    // Feature: editor-canonical-architecture-refactor, Property 19: Registration Validation
    
    fn dummy_handler(_context: ActionContext) -> ActionResult {
        ActionResult::success(None)
    }
    
    let mut builder = RegistryBuilder::new();
    
    // Test each required field individually
    
    // 1. Empty action_id
    let result = builder.register_action(ActionDefinition {
        action_id: ActionId::new(""),
        display_label: "Valid".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.test.v1".to_string(),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class: MutationClass::Mutate,
        denial_families: vec![],
        handler: dummy_handler,
    });
    assert!(result.is_err(), "Empty action_id should be rejected");
    assert!(matches!(result.unwrap_err(), RegistrationError::InvalidActionId(_)));
    
    // 2. Empty display_label
    let result = builder.register_action(ActionDefinition {
        action_id: ActionId::new("test.action"),
        display_label: "".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.test.v1".to_string(),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class: MutationClass::Mutate,
        denial_families: vec![],
        handler: dummy_handler,
    });
    assert!(result.is_err(), "Empty display_label should be rejected");
    assert!(matches!(result.unwrap_err(), RegistrationError::EmptyDisplayLabel(_)));
    
    // 3. Empty tooling_route
    let result = builder.register_action(ActionDefinition {
        action_id: ActionId::new("test.action"),
        display_label: "Valid".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "".to_string(),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class: MutationClass::Mutate,
        denial_families: vec![],
        handler: dummy_handler,
    });
    assert!(result.is_err(), "Empty tooling_route should be rejected");
    assert!(matches!(result.unwrap_err(), RegistrationError::EmptyToolingRoute(_)));
    
    // 4. Empty sdk_packet_family
    let result = builder.register_action(ActionDefinition {
        action_id: ActionId::new("test.action"),
        display_label: "Valid".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.test.v1".to_string(),
        sdk_packet_family: "".to_string(),
        engine_truth_owner: "engine/test".to_string(),
        mutation_class: MutationClass::Mutate,
        denial_families: vec![],
        handler: dummy_handler,
    });
    assert!(result.is_err(), "Empty sdk_packet_family should be rejected");
    assert!(matches!(result.unwrap_err(), RegistrationError::EmptySdkPacketFamily(_)));
    
    // 5. Empty engine_truth_owner
    let result = builder.register_action(ActionDefinition {
        action_id: ActionId::new("test.action"),
        display_label: "Valid".to_string(),
        action_family: ActionFamily::World,
        tooling_route: "route.test.v1".to_string(),
        sdk_packet_family: "packet.test.*".to_string(),
        engine_truth_owner: "".to_string(),
        mutation_class: MutationClass::Mutate,
        denial_families: vec![],
        handler: dummy_handler,
    });
    assert!(result.is_err(), "Empty engine_truth_owner should be rejected");
    assert!(matches!(result.unwrap_err(), RegistrationError::EmptyEngineTruthOwner(_)));
}

/// **Property: Error Messages Are Descriptive**
///
/// Verifies that validation errors include descriptive messages that help
/// developers identify and fix the problem.
#[test]
fn prop_error_messages_are_descriptive() {
    // Feature: editor-canonical-architecture-refactor, Property 19: Registration Validation
    let config = ProptestConfig::with_cases(50);
    proptest!(config, |((definition, _error_type) in arb_invalid_action_definition())| {
        let mut builder = RegistryBuilder::new();
        
        let result = builder.register_action(definition.clone());
        
        if let Err(error) = result {
            let error_message = error.to_string();
            
            // Verify error message is not empty
            prop_assert!(
                !error_message.is_empty(),
                "Error message should not be empty"
            );
            
            // Verify error message contains useful information
            prop_assert!(
                error_message.len() > 10,
                "Error message should be descriptive (more than 10 characters)"
            );
            
            // Verify error message identifies the field or problem
            let contains_field_info = error_message.contains("action")
                || error_message.contains("label")
                || error_message.contains("route")
                || error_message.contains("packet")
                || error_message.contains("owner")
                || error_message.contains("empty")
                || error_message.contains("invalid");
            
            prop_assert!(
                contains_field_info,
                "Error message should identify the problematic field: {}",
                error_message
            );
        }
    });
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_validation_rejects_empty_action_id() {
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let mut builder = RegistryBuilder::new();
        
        let definition = ActionDefinition {
            action_id: ActionId::new(""),
            display_label: "Valid Label".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.test.v1".to_string(),
            sdk_packet_family: "packet.test.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        let result = builder.register_action(definition);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RegistrationError::InvalidActionId(_)));
    }

    #[test]
    fn test_validation_rejects_empty_display_label() {
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let mut builder = RegistryBuilder::new();
        
        let definition = ActionDefinition {
            action_id: ActionId::new("test.action"),
            display_label: "".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.test.v1".to_string(),
            sdk_packet_family: "packet.test.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        let result = builder.register_action(definition);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RegistrationError::EmptyDisplayLabel(_)));
    }

    #[test]
    fn test_validation_accepts_valid_definition() {
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let mut builder = RegistryBuilder::new();
        
        let definition = ActionDefinition {
            action_id: ActionId::new("world.open"),
            display_label: "Open World".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.world.open.v1".to_string(),
            sdk_packet_family: "packet.world.*".to_string(),
            engine_truth_owner: "engine/world".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        let result = builder.register_action(definition);
        assert!(result.is_ok());
        assert_eq!(builder.len(), 1);
    }

    #[test]
    fn test_validation_error_messages_are_descriptive() {
        fn dummy_handler(_context: ActionContext) -> ActionResult {
            ActionResult::success(None)
        }
        
        let mut builder = RegistryBuilder::new();
        
        let definition = ActionDefinition {
            action_id: ActionId::new(""),
            display_label: "Valid".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.test.v1".to_string(),
            sdk_packet_family: "packet.test.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            denial_families: vec![],
            handler: dummy_handler,
        };
        
        let result = builder.register_action(definition);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        let error_message = error.to_string();
        
        assert!(!error_message.is_empty());
        assert!(error_message.contains("action") || error_message.contains("Invalid"));
    }
}
