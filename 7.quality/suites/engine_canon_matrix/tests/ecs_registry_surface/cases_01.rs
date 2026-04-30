#[test]
fn test_new_registry_is_empty() {
    let registry = RegistryModel::new();
    assert_eq!(registry.entities().count(), 0);
}

#[test]
fn test_default_equals_new() {
    let r1 = RegistryModel::new();
    let r2 = RegistryModel::default();
    assert_eq!(r1.entities().count(), r2.entities().count());
}

// === Entity Registration Tests ===

#[test]
fn test_register_entity_adds_to_set() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    registry.register_entity(entity);
    assert_eq!(registry.entities().count(), 1);
    assert!(registry.entities().any(|e| e == entity));
}

#[test]
fn test_register_duplicate_entity_is_idempotent() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    registry.register_entity(entity);
    registry.register_entity(entity);
    assert_eq!(registry.entities().count(), 1);
}

#[test]
fn test_register_multiple_unique_entities() {
    let mut registry = RegistryModel::new();
    let e1 = make_entity(1);
    let e2 = make_entity(2);
    let e3 = make_entity(3);

    registry.register_entity(e1);
    registry.register_entity(e2);
    registry.register_entity(e3);

    assert_eq!(registry.entities().count(), 3);
}

// === Component Class Tests ===

#[test]
fn test_register_component_class() {
    let mut registry = RegistryModel::new();
    let component = ComponentTypeId(100);
    registry.register_component_class(component);
}

#[test]
fn test_register_duplicate_component_class_is_idempotent() {
    let mut registry = RegistryModel::new();
    let component = ComponentTypeId(100);
    registry.register_component_class(component);
    registry.register_component_class(component);
}

// === Component Attachment Tests ===

#[test]
fn test_attach_component_success() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);
    registry.register_component_class(component);

    let result = registry.attach_component(entity, component);
    assert!(result.is_ok());
    assert!(registry.has_component(entity, component));
}

#[test]
fn test_attach_component_unknown_entity_fails() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(999);
    let component = ComponentTypeId(100);

    registry.register_component_class(component);

    let result = registry.attach_component(entity, component);
    assert!(result.is_err());
}

#[test]
fn test_attach_component_unknown_class_fails() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);
    // Don't register component class

    let result = registry.attach_component(entity, component);
    assert!(result.is_err());
}

#[test]
fn test_attach_multiple_components_to_entity() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let c1 = ComponentTypeId(100);
    let c2 = ComponentTypeId(200);

    registry.register_entity(entity);
    registry.register_component_class(c1);
    registry.register_component_class(c2);

    registry.attach_component(entity, c1).unwrap();
    registry.attach_component(entity, c2).unwrap();

    assert!(registry.has_component(entity, c1));
    assert!(registry.has_component(entity, c2));
}

// === Component Detachment Tests ===

#[test]
fn test_detach_component_success() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);
    registry.register_component_class(component);
    registry.attach_component(entity, component).unwrap();

    assert!(registry.has_component(entity, component));

    let removed = registry.detach_component(entity, component);
    assert!(removed);
    assert!(!registry.has_component(entity, component));
}

#[test]
fn test_detach_nonexistent_component_returns_false() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);

    let removed = registry.detach_component(entity, component);
    assert!(!removed);
}

#[test]
fn test_detach_from_unregistered_entity_returns_false() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(999);
    let component = ComponentTypeId(100);

    let removed = registry.detach_component(entity, component);
    assert!(!removed);
}

// === has_component Tests ===

#[test]
fn test_has_component_false_for_unattached() {
    let mut registry = RegistryModel::new();
    let entity = make_entity(1);
    let component = ComponentTypeId(100);

    registry.register_entity(entity);
    registry.register_component_class(component);

    assert!(!registry.has_component(entity, component));
}

