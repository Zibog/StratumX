#![allow(unused_imports)]
use super::*;
use engine_core::Generation;
use engine_ecs_query::{
    AccessDescriptor, Partitionability, QueryAccessMode, QueryDescriptor, QueryLocality,
};
use engine_identity::EntityId;
use smallvec::smallvec;

#[test]
fn register_entity_allows_descriptor_lookup() {
    let mut ecs = EcsSubstrate::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    ecs.register_entity(e);
    assert!(ecs.entity_descriptor(e).is_ok());
}
#[test]
fn descriptor_requires_registration() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn attach_component_round_trips_membership() {
    let mut ecs = EcsSubstrate::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    ecs.register_entity(e);
    ecs.register_component_class(c);
    ecs.attach_component(e, c).unwrap();
    assert_eq!(
        ecs.entity_descriptor(e).unwrap().membership.components,
        vec![c]
    );
}
#[test]
fn query_delegates_to_registry() {
    let mut ecs = EcsSubstrate::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    ecs.register_entity(e);
    ecs.register_component_class(c);
    ecs.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Spatial,
        partitionability: Partitionability::Chunk,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert_eq!(ecs.query(&q).unwrap().len(), 1);
}
#[test]
fn registry_accessor_exposes_backing_model() {
    let ecs = EcsSubstrate::new();
    assert_eq!(ecs.registry().entities().count(), 0);
}
