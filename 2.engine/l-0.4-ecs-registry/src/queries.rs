use crate::types::{MembershipDescriptor, RegistryModel};
use engine_core::ComponentTypeId;
use engine_identity::EntityId;

pub struct RegistryQuery<'a> {
    registry: &'a RegistryModel,
}

impl<'a> RegistryQuery<'a> {
    pub fn new(registry: &'a RegistryModel) -> Self {
        Self { registry }
    }

    pub fn membership(&self, entity: EntityId) -> MembershipDescriptor {
        self.registry.membership(entity)
    }

    pub fn members_with_component(&self, component: ComponentTypeId) -> Vec<EntityId> {
        self.registry.members_with_component(component).collect()
    }
}
