use crate::types::EntityDescriptor;
use engine_ecs_query::QueryDescriptor;
use engine_ecs_registry::RegistryModel;

pub struct EcsQuery<'a> {
    registry: &'a RegistryModel,
}

impl<'a> EcsQuery<'a> {
    pub fn new(registry: &'a RegistryModel) -> Self {
        Self { registry }
    }

    pub fn execute(&self, descriptor: &QueryDescriptor) -> Option<Vec<EntityDescriptor>> {
        descriptor.execute(self.registry).ok().map(|matches| {
            matches
                .into_iter()
                .map(|entity| EntityDescriptor {
                    entity,
                    membership: self.registry.membership(entity),
                })
                .collect()
        })
    }
}
