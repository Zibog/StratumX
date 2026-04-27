use engine_core::{ComponentTypeId, EngineCoreError, Generation};
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation::INITIAL,
    }
}

#[test]
fn attach_requires_registered_entity_and_component_class() {
    let mut registry = RegistryModel::new();
    let entity = entity(1);
    let component = ComponentTypeId(11);

    assert_eq!(
        registry.attach_component(entity, component),
        Err(EngineCoreError::InvalidDescriptor(
            "entity must be registered before component attach",
        ))
    );

    registry.register_entity(entity);
    assert_eq!(
        registry.attach_component(entity, component),
        Err(EngineCoreError::InvalidDescriptor(
            "component class must be registered before attach",
        ))
    );

    registry.register_component_class(component);
    registry.attach_component(entity, component).unwrap();
    assert!(registry.has_component(entity, component));
}

#[test]
fn membership_and_component_queries_stay_structural() {
    let mut registry = RegistryModel::new();
    let entity_a = entity(1);
    let entity_b = entity(2);
    let component_a = ComponentTypeId(7);
    let component_b = ComponentTypeId(9);

    registry.register_entity(entity_a);
    registry.register_entity(entity_b);
    registry.register_component_class(component_a);
    registry.register_component_class(component_b);
    registry.attach_component(entity_a, component_a).unwrap();
    registry.attach_component(entity_a, component_b).unwrap();
    registry.attach_component(entity_b, component_b).unwrap();

    assert_eq!(
        registry.membership(entity_a).components,
        vec![component_a, component_b]
    );
    assert_eq!(
        registry
            .members_with_component(component_b)
            .collect::<Vec<_>>(),
        vec![entity_a, entity_b]
    );
    assert!(registry.detach_component(entity_a, component_b));
    assert!(!registry.has_component(entity_a, component_b));
}

#[test]
fn registry_model_roundtrips_through_binary_serde() {
    let mut registry = RegistryModel::new();
    let entity = entity(3);
    let component = ComponentTypeId(42);
    registry.register_entity(entity);
    registry.register_component_class(component);
    registry.attach_component(entity, component).unwrap();

    let bytes = bincode::serialize(&registry).unwrap();
    let restored: RegistryModel = bincode::deserialize(&bytes).unwrap();

    assert_eq!(restored, registry);
}
