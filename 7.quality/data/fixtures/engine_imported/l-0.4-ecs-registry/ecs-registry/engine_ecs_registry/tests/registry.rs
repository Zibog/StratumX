use engine_core::ComponentTypeId;
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: engine_core::Generation::INITIAL,
    }
}

#[test]
fn registry_tracks_presence_and_membership() {
    let mut registry = RegistryModel::new();
    let e0 = entity(0);
    let c0 = ComponentTypeId(10);

    registry.register_entity(e0);
    registry.register_component_class(c0);
    registry.attach_component(e0, c0).unwrap();

    assert!(registry.has_component(e0, c0));
    assert_eq!(registry.membership(e0).components, vec![c0]);
}

#[test]
fn attach_requires_registration() {
    let mut registry = RegistryModel::new();
    let e0 = entity(0);
    let c0 = ComponentTypeId(10);

    assert!(registry.attach_component(e0, c0).is_err());

    registry.register_entity(e0);
    assert!(registry.attach_component(e0, c0).is_err());
}
