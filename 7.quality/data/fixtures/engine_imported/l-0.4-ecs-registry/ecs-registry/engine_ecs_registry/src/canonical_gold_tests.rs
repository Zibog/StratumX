#![allow(unused_imports)]
use super::*;
use engine_core::Generation;
use engine_identity::EntityId;

#[test]
fn register_entity_creates_membership_row() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    r.register_entity(e);
    assert!(r.entities().any(|x| x == e));
}
#[test]
fn attach_requires_registered_component_class() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    r.register_entity(e);
    assert!(r.attach_component(e, ComponentTypeId(1)).is_err());
}
#[test]
fn attach_sets_presence() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    assert!(r.has_component(e, c));
}
#[test]
fn detach_clears_presence() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    assert!(r.detach_component(e, c));
    assert!(!r.has_component(e, c));
}
#[test]
fn members_with_component_returns_registered_member() {
    let mut r = RegistryModel::new();
    let e = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let c = ComponentTypeId(1);
    r.register_entity(e);
    r.register_component_class(c);
    r.attach_component(e, c).unwrap();
    assert_eq!(r.members_with_component(c).collect::<Vec<_>>(), vec![e]);
}
