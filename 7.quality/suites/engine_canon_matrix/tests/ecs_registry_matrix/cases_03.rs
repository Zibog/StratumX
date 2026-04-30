#[test]
fn ecs_registry_attach_membership_24() {
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
