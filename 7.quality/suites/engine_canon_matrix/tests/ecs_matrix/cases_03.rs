#[test]
fn ecs_substrate_query_reflects_membership_7() {
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
        access: QueryAccessDescriptor {
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
fn ecs_substrate_query_reflects_membership_8() {
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
        access: QueryAccessDescriptor {
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
fn ecs_substrate_query_reflects_membership_9() {
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
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert_eq!(ecs.query(&q).unwrap().len(), 1);
}
