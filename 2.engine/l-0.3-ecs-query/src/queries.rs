use crate::QueryDescriptor;
use engine_core::EngineCoreResult;
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;

pub struct QueryInterface<'a> {
    registry: &'a RegistryModel,
}

impl<'a> QueryInterface<'a> {
    pub fn new(registry: &'a RegistryModel) -> Self {
        Self { registry }
    }

    pub fn execute(&self, descriptor: &QueryDescriptor) -> EngineCoreResult<Vec<EntityId>> {
        descriptor.execute(self.registry)
    }
}
