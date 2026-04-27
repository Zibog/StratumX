use engine_handle::*;
use engine_core::{ComponentTypeId, Generation};
use engine_identity::EntityId;

fn make_entity_id(slot: u32) -> EntityId {
    EntityId { slot, generation: Generation(0) }
}

fn make_stable_handle(slot: u32) -> StableEntityHandle {
    StableEntityHandle {
        id: make_entity_id(slot),
        observed_generation: Generation(0),
        state: InvalidationState::Active,
    }
}

// === InvalidationState Tests ===

#[test]
fn test_invalidation_state_variants() {
    assert_ne!(InvalidationState::Active, InvalidationState::Invalidated);
}

#[test]
fn test_invalidation_state_equality() {
    assert_eq!(InvalidationState::Active, InvalidationState::Active);
    assert_eq!(InvalidationState::Invalidated, InvalidationState::Invalidated);
}

// === ValidationResult Tests ===

#[test]
fn test_validation_result_variants() {
    assert!(matches!(ValidationResult::Valid, ValidationResult::Valid));
    assert!(matches!(ValidationResult::Invalidated, ValidationResult::Invalidated));
    assert!(matches!(ValidationResult::Stale, ValidationResult::Stale));
    assert!(matches!(ValidationResult::IllegalContext, ValidationResult::IllegalContext));
}

#[test]
fn test_validation_result_equality() {
    assert_eq!(ValidationResult::Valid, ValidationResult::Valid);
    assert_eq!(ValidationResult::Stale, ValidationResult::Stale);
    assert_ne!(ValidationResult::Valid, ValidationResult::Stale);
}

// === StableEntityHandle Tests ===

#[test]
fn test_stable_entity_handle_creation() {
    let handle = make_stable_handle(1);
    assert_eq!(handle.id.slot, 1);
    assert_eq!(handle.observed_generation, Generation(0));
    assert_eq!(handle.state, InvalidationState::Active);
}

#[test]
fn test_stable_entity_handle_equality() {
    let a = make_stable_handle(1);
    let b = make_stable_handle(1);
    let c = make_stable_handle(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_stable_entity_handle_different_states() {
    let active = StableEntityHandle {
        id: make_entity_id(1),
        observed_generation: Generation(0),
        state: InvalidationState::Active,
    };
    let invalidated = StableEntityHandle {
        id: make_entity_id(1),
        observed_generation: Generation(0),
        state: InvalidationState::Invalidated,
    };
    assert_ne!(active, invalidated);
}

#[test]
fn test_stable_entity_handle_different_generations() {
    let gen0 = StableEntityHandle {
        id: make_entity_id(1),
        observed_generation: Generation(0),
        state: InvalidationState::Active,
    };
    let gen1 = StableEntityHandle {
        id: make_entity_id(1),
        observed_generation: Generation(1),
        state: InvalidationState::Active,
    };
    assert_ne!(gen0, gen1);
}

// === Handle Invalidation Tests ===

#[test]
fn test_handle_can_be_invalidated() {
    let mut handle = make_stable_handle(1);
    assert_eq!(handle.state, InvalidationState::Active);
    handle.state = InvalidationState::Invalidated;
    assert_eq!(handle.state, InvalidationState::Invalidated);
}

// === Serialization Tests ===

#[test]
fn test_invalidation_state_serialization_roundtrip() {
    for state in [InvalidationState::Active, InvalidationState::Invalidated] {
        let json = serde_json::to_string(&state).unwrap();
        let loaded: InvalidationState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, loaded);
    }
}

#[test]
fn test_stable_entity_handle_serialization_roundtrip() {
    let handle = make_stable_handle(42);
    let json = serde_json::to_string(&handle).unwrap();
    let loaded: StableEntityHandle = serde_json::from_str(&json).unwrap();
    assert_eq!(handle, loaded);
}

// === Generation Tests ===

#[test]
fn test_generation_ordering() {
    assert!(Generation(0) < Generation(1));
    assert!(Generation(1) < Generation(2));
    assert!(Generation(100) > Generation(99));
}

#[test]
fn test_generation_equality() {
    assert_eq!(Generation(42), Generation(42));
    assert_ne!(Generation(42), Generation(43));
}

// === ComponentTypeId through handle Tests ===

#[test]
fn test_component_type_id_in_context() {
    let component = ComponentTypeId(100);
    assert_eq!(component, ComponentTypeId(100));
    assert_ne!(component, ComponentTypeId(200));
}
