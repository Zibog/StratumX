#[test]
fn ecs_registry_attach_membership_0() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_1() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_2() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_3() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_4() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_5() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_6() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_7() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_8() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_9() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_10() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
#[test]
fn ecs_registry_attach_membership_11() {
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
    assert_eq!(r.membership(e).components.len(), 1);
}
