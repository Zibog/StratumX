use engine_core::Generation;
use engine_identity::*;

// === Generation Tests ===

#[test]
fn test_generation_creation() {
    let gen = Generation(42);
    assert_eq!(gen, Generation(42));
}

#[test]
fn test_generation_equality() {
    assert_eq!(Generation(0), Generation(0));
    assert_eq!(Generation(100), Generation(100));
    assert_ne!(Generation(0), Generation(1));
}

#[test]
fn test_generation_ordering() {
    assert!(Generation(0) < Generation(1));
    assert!(Generation(99) < Generation(100));
    assert!(Generation(5) > Generation(3));
}

#[test]
fn test_generation_max_value() {
    let max = Generation(u32::MAX);
    assert_eq!(max, Generation(u32::MAX));
}

// === EntityId Tests ===

#[test]
fn test_entity_id_creation() {
    let id = EntityId { slot: 1, generation: Generation(0) };
    assert_eq!(id.slot, 1);
    assert_eq!(id.generation, Generation(0));
}

#[test]
fn test_entity_id_equality() {
    let a = EntityId { slot: 1, generation: Generation(0) };
    let b = EntityId { slot: 1, generation: Generation(0) };
    let c = EntityId { slot: 2, generation: Generation(0) };
    let d = EntityId { slot: 1, generation: Generation(1) };

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
}

#[test]
fn test_entity_id_ordering() {
    let a = EntityId { slot: 1, generation: Generation(0) };
    let b = EntityId { slot: 2, generation: Generation(0) };
    assert!(a < b);
}

#[test]
fn test_entity_id_same_slot_different_generation() {
    let old = EntityId { slot: 1, generation: Generation(0) };
    let new = EntityId { slot: 1, generation: Generation(1) };
    assert_ne!(old, new);
    assert!(old < new); // Same slot, but higher generation > lower
}

// === ComponentId Tests ===

#[test]
fn test_component_id_creation() {
    let id = ComponentId { slot: 100, generation: Generation(5) };
    assert_eq!(id.slot, 100);
    assert_eq!(id.generation, Generation(5));
}

#[test]
fn test_component_id_equality() {
    let a = ComponentId { slot: 1, generation: Generation(0) };
    let b = ComponentId { slot: 1, generation: Generation(0) };
    let c = ComponentId { slot: 1, generation: Generation(1) };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_component_id_ordering() {
    let a = ComponentId { slot: 1, generation: Generation(0) };
    let b = ComponentId { slot: 2, generation: Generation(0) };
    assert!(a < b);
}

// === IdentityDomain Tests ===

#[test]
fn test_identity_domain_variants() {
    assert_ne!(IdentityDomain::Entity, IdentityDomain::Component);
}

#[test]
fn test_identity_domain_equality() {
    assert_eq!(IdentityDomain::Entity, IdentityDomain::Entity);
    assert_eq!(IdentityDomain::Component, IdentityDomain::Component);
}

// === Serialization Tests ===

#[test]
fn test_entity_id_serialization_roundtrip() {
    let id = EntityId { slot: 42, generation: Generation(7) };
    let json = serde_json::to_string(&id).unwrap();
    let loaded: EntityId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, loaded);
}

#[test]
fn test_component_id_serialization_roundtrip() {
    let id = ComponentId { slot: 99, generation: Generation(3) };
    let json = serde_json::to_string(&id).unwrap();
    let loaded: ComponentId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, loaded);
}

#[test]
fn test_generation_serialization_roundtrip() {
    let gen = Generation(12345);
    let json = serde_json::to_string(&gen).unwrap();
    let loaded: Generation = serde_json::from_str(&json).unwrap();
    assert_eq!(gen, loaded);
}

// === Edge Cases ===

#[test]
fn test_entity_id_zero_values() {
    let id = EntityId { slot: 0, generation: Generation(0) };
    assert_eq!(id.slot, 0);
    assert_eq!(id.generation, Generation(0));
}

#[test]
fn test_entity_id_max_values() {
    let id = EntityId { slot: u32::MAX, generation: Generation(u32::MAX) };
    assert_eq!(id.slot, u32::MAX);
    assert_eq!(id.generation, Generation(u32::MAX));
}

#[test]
fn test_identity_entities_are_hashable() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let id1 = EntityId { slot: 1, generation: Generation(0) };
    let id2 = EntityId { slot: 1, generation: Generation(0) };
    let id3 = EntityId { slot: 2, generation: Generation(0) };

    set.insert(id1);
    set.insert(id2); // Duplicate
    set.insert(id3);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&id1));
    assert!(set.contains(&id3));
}
