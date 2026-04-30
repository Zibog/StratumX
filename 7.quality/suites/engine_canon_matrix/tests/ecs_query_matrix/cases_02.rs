#[test]
fn ecs_query_execute_matches_registered_entity_6() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let out = q.execute(&r).unwrap();
    assert_eq!(out, vec![e]);
}
#[test]
fn ecs_query_execute_matches_registered_entity_7() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let out = q.execute(&r).unwrap();
    assert_eq!(out, vec![e]);
}
#[test]
fn ecs_query_execute_matches_registered_entity_8() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let out = q.execute(&r).unwrap();
    assert_eq!(out, vec![e]);
}
#[test]
fn ecs_query_execute_matches_registered_entity_9() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let out = q.execute(&r).unwrap();
    assert_eq!(out, vec![e]);
}
#[test]
fn ecs_query_execute_matches_registered_entity_10() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let out = q.execute(&r).unwrap();
    assert_eq!(out, vec![e]);
}
#[test]
fn ecs_query_execute_matches_registered_entity_11() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    let q = QueryDescriptor {
        component_set: smallvec![c],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    let out = q.execute(&r).unwrap();
    assert_eq!(out, vec![e]);
}
