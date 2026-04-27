use engine_core::{ComponentTypeId, EngineCoreError, Generation};
use engine_ecs::EcsSubstrate;
use engine_ecs_query::{
    AccessDescriptor, FilterConstraint, FilterOp, Partitionability, QueryAccessMode,
    QueryDescriptor, QueryLocality,
};
use engine_identity::EntityId;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation::INITIAL,
    }
}

#[test]
fn entity_descriptor_requires_registered_entity() {
    let ecs = EcsSubstrate::new();

    assert_eq!(
        ecs.entity_descriptor(entity(1)),
        Err(EngineCoreError::InvalidDescriptor(
            "entity must be registered in ecs substrate",
        ))
    );
}

#[test]
fn entity_archetype_reflects_registered_components() {
    let mut ecs = EcsSubstrate::new();
    let entity = entity(1);
    let component_a = ComponentTypeId(5);
    let component_b = ComponentTypeId(9);

    ecs.register_entity(entity);
    ecs.register_component_class(component_a);
    ecs.register_component_class(component_b);
    ecs.attach_component(entity, component_a).unwrap();
    ecs.attach_component(entity, component_b).unwrap();

    assert_eq!(
        ecs.entity_archetype(entity).unwrap(),
        vec![component_a, component_b]
    );
}

#[test]
fn query_returns_membership_descriptors() {
    let mut ecs = EcsSubstrate::new();
    let entity_a = entity(1);
    let entity_b = entity(2);
    let component_a = ComponentTypeId(3);
    let component_b = ComponentTypeId(4);

    for entity in [entity_a, entity_b] {
        ecs.register_entity(entity);
    }
    for component in [component_a, component_b] {
        ecs.register_component_class(component);
    }
    ecs.attach_component(entity_a, component_a).unwrap();
    ecs.attach_component(entity_a, component_b).unwrap();
    ecs.attach_component(entity_b, component_a).unwrap();

    let descriptor = QueryDescriptor {
        component_set: vec![component_a].into(),
        locality: QueryLocality::Cache,
        partitionability: Partitionability::Region,
        cache_key: 19,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: vec![FilterConstraint {
            op: FilterOp::With,
            component: component_b,
        }]
        .into(),
        joins: Vec::new().into(),
    };

    let matches = ecs.query(&descriptor).unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].entity, entity_a);
    assert_eq!(
        matches[0].membership.components,
        vec![component_a, component_b]
    );
}
