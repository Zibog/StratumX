#[test]
fn test_has_component_false_for_unknown_entity() {
    let registry = RegistryModel::new();
    let entity = make_entity(999);
    let component = ComponentTypeId(100);

    assert!(!registry.has_component(entity, component));
}

// === MembershipDescriptor Tests ===

#[test]
fn test_membership_empty_components_for_new_entity() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    registry.register_entity(entity);

    let membership = registry.membership(entity);
    assert_eq!(membership.entity, entity);
    assert!(membership.components.is_empty());
}

#[test]
fn test_membership_contains_attached_components() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let c1 = ComponentTypeId(100);
    let c2 = ComponentTypeId(200);

    registry.register_entity(entity);
    registry.register_component_class(c1);
    registry.register_component_class(c2);
    registry.attach_component(entity, c1).unwrap();
    registry.attach_component(entity, c2).unwrap();

    let membership = registry.membership(entity);
    assert_eq!(membership.components.len(), 2);
    assert!(membership.components.contains(&c1));
    assert!(membership.components.contains(&c2));
}

#[test]
fn test_membership_after_detach() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);
    registry.register_component_class(component);
    registry.attach_component(entity, component).unwrap();

    registry.detach_component(entity, component);

    let membership = registry.membership(entity);
    assert!(membership.components.is_empty());
}

// === Query Tests ===

#[test]
fn test_members_with_component_empty() {
    let mut registry = RegistryModel::new();
    let component = ComponentTypeId(100);
    registry.register_component_class(component);

    let members: Vec<_> = registry.members_with_component(component).collect();
    assert!(members.is_empty());
}

#[test]
fn test_members_with_component_returns_matching_entities() {
    let mut registry = RegistryModel::new();
    let e1 = make_entity(1);
    let e2 = make_entity(2);
    let e3 = make_entity(3);
    let c = ComponentTypeId(100);

    registry.register_entity(e1);
    registry.register_entity(e2);
    registry.register_entity(e3);
    registry.register_component_class(c);

    registry.attach_component(e1, c).unwrap();
    registry.attach_component(e3, c).unwrap();

    let members: Vec<_> = registry.members_with_component(c).collect();
    assert_eq!(members.len(), 2);
    assert!(members.contains(&e1));
    assert!(members.contains(&e3));
    assert!(!members.contains(&e2));
}

// === Serialization Round-trip Tests ===

#[test]
fn test_membership_descriptor_serializes() {
    let descriptor = engine_ecs_registry::MembershipDescriptor {
        entity: make_entity(42),
        components: vec![ComponentTypeId(100), ComponentTypeId(200)],
    };

    let json = serde_json::to_string(&descriptor).unwrap();
    let loaded: engine_ecs_registry::MembershipDescriptor = serde_json::from_str(&json).unwrap();
    assert_eq!(descriptor, loaded);
}

// === Invariant Tests ===

#[test]
fn test_entity_count_matches_entity_set() {
    let mut registry = RegistryModel::new();
    for i in 0..10 {
        registry.register_entity(make_entity(i));
    }
    assert_eq!(registry.entities().count(), 10);
}

#[test]
fn test_presence_map_only_contains_registered_entities() {
    let mut registry = RegistryModel::new();
    let e1 = make_entity(1);
    let e2 = make_entity(2);
    let c = ComponentTypeId(100);

    registry.register_entity(e1);
    registry.register_entity(e2);
    registry.register_component_class(c);
    registry.attach_component(e1, c).unwrap();

    // e2 has no components but should still be in presence_map
    assert!(registry.membership(e2).entity == e2);
}

#[test]
fn test_detach_then_reattach_component() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);
    registry.register_component_class(component);

    registry.attach_component(entity, component).unwrap();
    assert!(registry.has_component(entity, component));

    registry.detach_component(entity, component);
    assert!(!registry.has_component(entity, component));

    // Re-attach should work
    registry.attach_component(entity, component).unwrap();
    assert!(registry.has_component(entity, component));
}
