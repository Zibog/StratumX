use engine_core::Generation;
use engine_ecs::{EcsQuery, EcsSubstrate};
use engine_ecs_query::{
    AccessDescriptor, Partitionability, QueryAccessMode, QueryDescriptor, QueryLocality,
};
use engine_identity::EntityId;
use smallvec::SmallVec;

fn make_entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation(0),
    }
}

#[test]
fn test_ecs_substrate_creation() {
    let ecs = EcsSubstrate::new();
    assert_eq!(ecs.registry().entities().count(), 0);
}

#[test]
fn test_register_entity() {
    let mut ecs = EcsSubstrate::new();
    let entity = make_entity(1);
    ecs.register_entity(entity);
    assert_eq!(ecs.registry().entities().count(), 1);
    assert!(ecs.registry().entities().any(|e| e == entity));
}

#[test]
fn test_register_multiple_entities() {
    let mut ecs = EcsSubstrate::new();
    ecs.register_entity(make_entity(1));
    ecs.register_entity(make_entity(2));
    ecs.register_entity(make_entity(3));
    assert_eq!(ecs.registry().entities().count(), 3);
}

#[test]
fn test_entity_descriptor_success() {
    let mut ecs = EcsSubstrate::new();
    let entity = make_entity(1);
    ecs.register_entity(entity);
    let desc = ecs.entity_descriptor(entity);
    assert!(desc.is_ok());
    assert_eq!(desc.unwrap().entity, entity);
}

#[test]
fn test_entity_descriptor_unknown_entity_fails() {
    let ecs = EcsSubstrate::new();
    assert!(ecs.entity_descriptor(make_entity(999)).is_err());
}

#[test]
fn test_entity_archetype_empty() {
    let mut ecs = EcsSubstrate::new();
    let entity = make_entity(1);
    ecs.register_entity(entity);
    let archetype = ecs.entity_archetype(entity);
    assert!(archetype.is_ok());
    assert!(archetype.unwrap().is_empty());
}

#[test]
fn test_ecs_new_equals_default() {
    let ecs1 = EcsSubstrate::new();
    let ecs2 = EcsSubstrate::default();
    assert_eq!(
        ecs1.registry().entities().count(),
        ecs2.registry().entities().count()
    );
}

#[test]
fn test_registry_ref_does_not_consume_ecs() {
    let mut ecs = EcsSubstrate::new();
    ecs.register_entity(make_entity(1));

    let _registry_ref = ecs.registry();
    // ECS should still be usable
    assert!(ecs.entity_descriptor(make_entity(1)).is_ok());
}

#[test]
fn test_ecs_query_preserves_failure_path() {
    let ecs = EcsSubstrate::new();
    let query = EcsQuery::new(ecs.registry());
    let descriptor = QueryDescriptor {
        component_set: SmallVec::new(),
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 7,
        filters: SmallVec::new(),
        joins: SmallVec::new(),
    };

    assert!(query.execute(&descriptor).is_err());
}
