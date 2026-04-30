use engine_core::ComponentTypeId;
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;

fn make_entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: engine_core::Generation(0),
    }
}

// === RegistryModel Lifecycle Tests ===

include!("ecs_registry_surface/cases_01.rs");
include!("ecs_registry_surface/cases_02.rs");
