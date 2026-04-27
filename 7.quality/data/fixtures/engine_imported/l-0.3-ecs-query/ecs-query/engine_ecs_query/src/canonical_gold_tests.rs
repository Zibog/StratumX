#![allow(unused_imports)]
use super::*;
use engine_core::Generation;
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;
use smallvec::smallvec;

#[test]
fn empty_component_set_is_illegal() {
    let q = QueryDescriptor {
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
    assert!(q.validate_legality().is_err());
}
#[test]
fn write_without_scratch_is_illegal() {
    let q = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(1)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(q.validate_legality().is_err());
}
#[test]
fn cache_key_must_be_non_zero() {
    let q = QueryDescriptor {
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
    assert!(q.validate_legality().is_err());
}
#[test]
fn execute_matches_entity_with_component() {
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
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert_eq!(q.execute(&r).unwrap(), vec![e]);
}
#[test]
fn without_filter_excludes_matching_entity() {
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
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![FilterConstraint {
            op: FilterOp::Without,
            component: c
        }],
        joins: smallvec![],
    };
    assert!(q.execute(&r).unwrap().is_empty());
}
