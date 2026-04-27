use engine_core::{ComponentTypeId, Generation};
use engine_ecs_query::{
    AccessDescriptor, FilterConstraint, FilterOp, Partitionability, QueryAccessMode,
    QueryDescriptor, QueryLocality,
};
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;
use engine_storage_access::ScratchClass;
use smallvec::smallvec;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation::INITIAL,
    }
}

#[test]
fn query_filters_members_deterministically() {
    let mut registry = RegistryModel::new();
    let position = ComponentTypeId(1);
    let velocity = ComponentTypeId(2);
    let sleeping = ComponentTypeId(3);
    for c in [position, velocity, sleeping] {
        registry.register_component_class(c);
    }

    let e0 = entity(0);
    let e1 = entity(1);
    registry.register_entity(e0);
    registry.register_entity(e1);
    registry.attach_component(e0, position).unwrap();
    registry.attach_component(e0, velocity).unwrap();
    registry.attach_component(e1, position).unwrap();
    registry.attach_component(e1, sleeping).unwrap();

    let descriptor = QueryDescriptor {
        component_set: smallvec![position],
        locality: QueryLocality::Spatial,
        partitionability: Partitionability::Chunk,
        cache_key: 42,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![FilterConstraint {
            op: FilterOp::Without,
            component: sleeping,
        }],
        joins: smallvec![],
    };

    let result = descriptor.execute(&registry).unwrap();
    assert_eq!(result, vec![e0]);
}

#[test]
fn write_mode_requires_scratch() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(1)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 7,
        access: AccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };

    assert!(descriptor.validate_legality().is_err());

    let mut valid = descriptor;
    valid.access.scratch = Some(ScratchClass::Owned);
    assert!(valid.validate_legality().is_ok());
}
