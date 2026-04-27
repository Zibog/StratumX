use engine_core::{ComponentTypeId, Generation};
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation::INITIAL,
    }
}

#[test]
fn members_with_component_are_reported_in_order() {
    let mut registry = RegistryModel::new();
    let component = ComponentTypeId(8);
    let e0 = entity(0);
    let e1 = entity(1);
    registry.register_component_class(component);
    for e in [e1, e0] {
        registry.register_entity(e);
        registry.attach_component(e, component).unwrap();
    }
    let members: Vec<_> = registry.members_with_component(component).collect();
    assert_eq!(members, vec![e0, e1]);
}

#[test]
fn detach_component_updates_presence() {
    let mut registry = RegistryModel::new();
    let component = ComponentTypeId(9);
    let e0 = entity(0);
    registry.register_entity(e0);
    registry.register_component_class(component);
    registry.attach_component(e0, component).unwrap();
    assert!(registry.detach_component(e0, component));
    assert!(!registry.has_component(e0, component));
}
