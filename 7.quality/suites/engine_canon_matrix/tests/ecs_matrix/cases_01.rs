#[test]
fn ecs_substrate_descriptor_requires_registration_0() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_1() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_2() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_3() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_4() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_5() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_6() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_7() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_8() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_9() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_10() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_11() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_12() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_13() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_descriptor_requires_registration_14() {
    let ecs = EcsSubstrate::new();
    assert!(ecs
        .entity_descriptor(EntityId {
            slot: 1,
            generation: Generation::INITIAL
        })
        .is_err());
}
#[test]
fn ecs_substrate_query_reflects_membership_0() {
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
