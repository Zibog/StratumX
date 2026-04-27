use engine_ecs_registry::MembershipDescriptor;
use engine_identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityDescriptor {
    pub entity: EntityId,
    pub membership: MembershipDescriptor,
}
