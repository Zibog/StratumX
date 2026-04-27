use crate::types::EntityDescriptor;
use engine_core::{ComponentTypeId, EngineCoreResult};
use engine_ecs_query::QueryDescriptor;
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;

#[derive(Debug, Default)]
pub struct EcsSubstrate {
    registry: RegistryModel,
}

impl EcsSubstrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity(&mut self, entity: EntityId) {
        self.registry.register_entity(entity);
    }

    pub fn register_component_class(&mut self, component: ComponentTypeId) {
        self.registry.register_component_class(component);
    }

    pub fn attach_component(
        &mut self,
        entity: EntityId,
        component: ComponentTypeId,
    ) -> EngineCoreResult<()> {
        self.registry.attach_component(entity, component)
    }

    pub fn entity_descriptor(&self, entity: EntityId) -> EngineCoreResult<EntityDescriptor> {
        if !self.registry.entities().any(|e| e == entity) {
            return Err(engine_core::EngineCoreError::InvalidDescriptor(
                "entity must be registered in ecs substrate",
            ));
        }
        Ok(EntityDescriptor {
            entity,
            membership: self.registry.membership(entity),
        })
    }

    pub fn entity_archetype(&self, entity: EntityId) -> EngineCoreResult<Vec<ComponentTypeId>> {
        Ok(self.entity_descriptor(entity)?.membership.components)
    }

    pub fn query(&self, descriptor: &QueryDescriptor) -> EngineCoreResult<Vec<EntityDescriptor>> {
        let matches = descriptor.execute(&self.registry)?;
        Ok(matches
            .into_iter()
            .map(|entity| EntityDescriptor {
                entity,
                membership: self.registry.membership(entity),
            })
            .collect())
    }

    pub fn registry(&self) -> &RegistryModel {
        &self.registry
    }
}
