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
fn ecs_assembles_registry_and_query_boundary() {
    let mut ecs = EcsSubstrate::new();
    let pos = ComponentTypeId(1);
    let e0 = entity(0);

    ecs.register_component_class(pos);
    ecs.register_entity(e0);
    ecs.attach_component(e0, pos).unwrap();

    let descriptor = QueryDescriptor {
        component_set: smallvec![pos],
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

    let rows = ecs.query(&descriptor).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].entity, e0);
    assert_eq!(rows[0].membership.components, vec![pos]);
}
