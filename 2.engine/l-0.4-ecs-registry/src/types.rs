use engine_core::ComponentTypeId;
use engine_identity::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryModel {
    pub(crate) entity_set: BTreeSet<EntityId>,
    pub(crate) component_classes: BTreeSet<ComponentTypeId>,
    pub(crate) presence_map: BTreeMap<EntityId, BTreeSet<ComponentTypeId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MembershipDescriptor {
    pub entity: EntityId,
    pub components: Vec<ComponentTypeId>,
}
