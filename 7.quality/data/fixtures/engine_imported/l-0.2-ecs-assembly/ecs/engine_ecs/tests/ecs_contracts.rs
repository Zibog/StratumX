use engine_core::{ComponentTypeId, Generation};
use engine_ecs::EcsSubstrate;
use engine_ecs_query::{
    AccessDescriptor, Partitionability, QueryAccessMode, QueryDescriptor, QueryLocality,
};
use engine_identity::EntityId;
use smallvec::smallvec;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation::INITIAL,
    }
}

#[test]
fn entity_descriptor_requires_registered_entity() {
    let ecs = EcsSubstrate::new();
    assert!(ecs.entity_descriptor(entity(999)).is_err());
}

#[test]
fn query_results_preserve_membership_for_multiple_entities() {
    let mut ecs = EcsSubstrate::new();
    let position = ComponentTypeId(1);
    ecs.register_component_class(position);
    for slot in [2, 1] {
        let e = entity(slot);
        ecs.register_entity(e);
        ecs.attach_component(e, position).unwrap();
    }
    let query = QueryDescriptor {
        component_set: smallvec![position],
        locality: QueryLocality::Spatial,
        partitionability: Partitionability::Region,
        cache_key: 9,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let rows = ecs.query(&query).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].membership.components, vec![position]);
    assert_eq!(rows[1].membership.components, vec![position]);
}
