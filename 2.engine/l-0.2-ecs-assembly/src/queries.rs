use crate::types::EntityDescriptor;
use engine_core::EngineCoreResult;
use engine_ecs_query::QueryDescriptor;
use engine_ecs_registry::RegistryModel;

pub struct EcsQuery<'a> {
    registry: &'a RegistryModel,
}

impl<'a> EcsQuery<'a> {
    pub fn new(registry: &'a RegistryModel) -> Self {
        Self { registry }
    }

    pub fn execute(&self, descriptor: &QueryDescriptor) -> EngineCoreResult<Vec<EntityDescriptor>> {
        let matches = descriptor.execute(self.registry)?;
        Ok(matches
            .into_iter()
            .map(|entity| EntityDescriptor {
                entity,
                membership: self.registry.membership(entity),
            })
            .collect())
    }
}
