use engine_core::{ComponentTypeId, Generation};
use engine_ecs_query::{
    AccessDescriptor, FilterConstraint, FilterOp, JoinRule, Partitionability, QueryAccessMode,
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
fn empty_component_set_is_illegal() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(descriptor.validate_legality().is_err());
}

#[test]
fn zero_cache_key_is_illegal() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(1)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 0,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(descriptor.validate_legality().is_err());
}

#[test]
fn joins_require_all_components() {
    let a = ComponentTypeId(1);
    let b = ComponentTypeId(2);
    let e0 = entity(0);
    let e1 = entity(1);
    let mut registry = RegistryModel::new();
    for c in [a, b] {
        registry.register_component_class(c);
    }
    for e in [e0, e1] {
        registry.register_entity(e);
    }
    registry.attach_component(e0, a).unwrap();
    registry.attach_component(e0, b).unwrap();
    registry.attach_component(e1, a).unwrap();

    let descriptor = QueryDescriptor {
        component_set: smallvec![a],
        locality: QueryLocality::Partition,
        partitionability: Partitionability::Chunk,
        cache_key: 77,
        access: AccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: Some(ScratchClass::Owned),
        },
        filters: smallvec![FilterConstraint {
            op: FilterOp::With,
            component: a
        }],
        joins: smallvec![JoinRule {
            required: smallvec![b]
        }],
    };
    assert_eq!(descriptor.execute(&registry).unwrap(), vec![e0]);
}
