use engine_core::{ComponentTypeId, EngineCoreError, Generation};
use engine_ecs_query::{
    AccessDescriptor, FilterConstraint, FilterOp, JoinRule, Partitionability, QueryAccessMode,
    QueryDescriptor, QueryLocality,
};
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;
use engine_storage_access::ScratchClass;

fn entity(slot: u32) -> EntityId {
    EntityId {
        slot,
        generation: Generation::INITIAL,
    }
}

fn base_descriptor() -> QueryDescriptor {
    QueryDescriptor {
        component_set: vec![ComponentTypeId(1)].into(),
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: Vec::<FilterConstraint>::new().into(),
        joins: Vec::<JoinRule>::new().into(),
    }
}

#[test]
fn write_queries_require_explicit_scratch_and_stable_cache_key() {
    let mut descriptor = base_descriptor();
    descriptor.access.mode = QueryAccessMode::WRITE;

    assert_eq!(
        descriptor.validate_legality(),
        Err(EngineCoreError::InvalidDescriptor(
            "write legality requires explicit scratch class",
        ))
    );

    descriptor.access.scratch = Some(ScratchClass::Owned);
    descriptor.cache_key = 0;
    assert_eq!(
        descriptor.validate_legality(),
        Err(EngineCoreError::InvalidDescriptor(
            "cache key must be explicit and stable",
        ))
    );
}

#[test]
fn empty_component_set_is_illegal() {
    let mut descriptor = base_descriptor();
    descriptor.component_set.clear();

    assert_eq!(
        descriptor.validate_legality(),
        Err(EngineCoreError::InvalidDescriptor(
            "component set must be explicit and non-empty",
        ))
    );
}

#[test]
fn execute_applies_filters_and_join_requirements() {
    let mut registry = RegistryModel::new();
    let entity_a = entity(1);
    let entity_b = entity(2);
    let component_a = ComponentTypeId(1);
    let component_b = ComponentTypeId(2);
    let component_c = ComponentTypeId(3);

    for entity in [entity_a, entity_b] {
        registry.register_entity(entity);
    }
    for component in [component_a, component_b, component_c] {
        registry.register_component_class(component);
    }
    registry.attach_component(entity_a, component_a).unwrap();
    registry.attach_component(entity_a, component_b).unwrap();
    registry.attach_component(entity_a, component_c).unwrap();
    registry.attach_component(entity_b, component_a).unwrap();
    registry.attach_component(entity_b, component_b).unwrap();

    let descriptor = QueryDescriptor {
        component_set: vec![component_a].into(),
        locality: QueryLocality::Spatial,
        partitionability: Partitionability::Chunk,
        cache_key: 77,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: true,
            scratch: None,
        },
        filters: vec![FilterConstraint {
            op: FilterOp::With,
            component: component_b,
        }]
        .into(),
        joins: vec![JoinRule {
            required: vec![component_c].into(),
        }]
        .into(),
    };

    assert_eq!(descriptor.execute(&registry).unwrap(), vec![entity_a]);
}
